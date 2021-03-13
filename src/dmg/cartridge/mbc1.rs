use crate::dmg::busconnection::BusConnection;

// (max 2MByte ROM and/or 32KByte RAM)
// https://gbdev.io/pandocs/#mbc1
pub struct MBC1 {
    rom_bank: Vec<u8>,
    ram_bank: Vec<u8>,
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_numer: u8,
    mode: BankingMode,
}

impl MBC1 {
    pub fn new(data: Vec<u8>) -> MBC1 {
        MBC1 {
            rom_bank: data,
            ram_bank: vec![0u8; 0x8000], // 32KBytes // TODO: Will have to read a save file into the ram as well (non default)
            ram_enable: false,
            rom_bank_number: 0,
            ram_bank_numer: 0,
            mode: BankingMode::Rom,
        }
    }
}

// 00 Simple ROM Banking Mode (default)
// 01 RAM Banking Mode / Advanced ROM Banking Mode
#[derive(Debug, PartialEq)]
enum BankingMode {
    Rom,
    Ram,
}

impl BusConnection for MBC1 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0..=0x3FFF => self.rom_bank[address as usize],
            0x4000..=0x7FFF => {
                // If the rom bank is 0x00,0x20,0x40,0x60 we add one to the rom
                // bank number. This kind of bizarre behaviour is documented here:
                // https://gbdev.io/pandocs/#_4000-7fff-rom-bank-01-7f-read-only
                let rom_bank = match self.rom_bank_number {
                    0x00 | 0x20 | 0x40 | 0x60 => self.rom_bank_number + 1,
                    _ => self.rom_bank_number,
                };
                self.rom_bank[(rom_bank as usize * 0x4000) + (address as usize - 0x4000)]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram_bank
                        [(self.ram_bank_numer as usize * 0x2000) + (address as usize - 0xA000)]
                } else {
                    0
                }
            }
            _ => panic!(
                "This should never happen, address: {:#02x} out of bounds for MBC1",
                address
            ),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => self.ram_enable = value == 0x0A,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x1F,
            0x4000..=0x5FFF => match self.mode {
                BankingMode::Ram => self.ram_bank_numer = value & 0x3,
                BankingMode::Rom => self.rom_bank_number = value & 0x60,
            },
            0x6000..=0x7FFF => {
                self.mode = match value & 1 {
                    0 => BankingMode::Rom,
                    1 => BankingMode::Ram,
                    _ => panic!("We've defied a law of mathematics!!"),
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram_bank
                        [(self.ram_bank_numer as usize * 0x2000) + (address as usize - 0xA000)] =
                        value;
                }
            }
            _ => panic!(
                "This should never happen, address: {:#02x} out of bounds for MBC1",
                address
            ),
        }
    }
}

#[test]
fn write_ram_enabled() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    mbc1.write_byte(0x1FFF, 0x0A); // enable the ram
    mbc1.write_byte(0xA000, 0xAA); // write to ram
    assert_eq!(mbc1.read_byte(0xA000), 0xAA);
}

#[test]
fn write_ram_disabled() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    mbc1.write_byte(0x1FFF, 0x00); // disable the ram
    mbc1.write_byte(0xA000, 0xAA); // write to ram
    assert_eq!(mbc1.read_byte(0xA000), 0x00);
}

#[test]
fn read_ram_disabled() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    mbc1.write_byte(0x1FFF, 0x0A); // enable the ram
    mbc1.write_byte(0xA000, 0xAA); // write to ram
    mbc1.write_byte(0x1FFF, 0x00); // disable the ram
    assert_eq!(mbc1.read_byte(0xA000), 0);
}

#[test]
fn change_mode_to_ram() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    assert_eq!(mbc1.mode, BankingMode::Rom);
    mbc1.write_byte(0x6000, 0x1);
    assert_eq!(mbc1.mode, BankingMode::Ram);
}

#[test]
fn change_rom_bank() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    assert_eq!(mbc1.rom_bank_number, 0);
    mbc1.write_byte(0x2000, 0x10);
    assert_eq!(mbc1.rom_bank_number, 0x10);
}

#[test]
fn change_ram_bank() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    assert_eq!(mbc1.ram_bank_numer, 0x0);
    mbc1.write_byte(0x6000, 0x1);
    assert_eq!(mbc1.mode, BankingMode::Ram);
    mbc1.write_byte(0x4000, 0x1);
    assert_eq!(mbc1.ram_bank_numer, 0x1);
}

#[test]
fn write_and_read_ram_bank_3() {
    let mut mbc1 = MBC1::new(vec![0u8; 0x4000]);
    assert_eq!(mbc1.ram_bank_numer, 0x0);
    mbc1.write_byte(0x6000, 0x1);
    assert_eq!(mbc1.mode, BankingMode::Ram);
    mbc1.write_byte(0x4000, 0x3);
    assert_eq!(mbc1.ram_bank_numer, 0x3);
    mbc1.write_byte(0x1FFF, 0x0A); // enable the ram
    mbc1.write_byte(0xA000, 0xAA); // write to ram
    assert_eq!(mbc1.read_byte(0xA000), 0xAA);
    mbc1.write_byte(0x4000, 0x2);
    assert_eq!(mbc1.ram_bank_numer, 0x2);
    assert_eq!(
        mbc1.read_byte(0xA000),
        0,
        "We've changed to a new ram bank, this should be empty"
    );
}
