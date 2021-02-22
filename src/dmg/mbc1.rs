use crate::dmg::busconnection::BusConnection;

pub struct MBC1 {
    rom_bank: Vec<u8>,
    ram_bank: Vec<u8>,
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_numer: u8,
    mode: BankingMode,
}

// TODO: allocate the data correctly
impl MBC1 {
    // TODO: Will have to read a save file into the ram as well
    pub fn new(data: Vec<u8>) -> MBC1 {
        MBC1 {
            rom_bank: ::std::iter::repeat(0u8).take(10).collect(),
            ram_bank: ::std::iter::repeat(0u8).take(10).collect(),
            ram_enable: false,
            rom_bank_number: 0,
            ram_bank_numer: 0,
            mode: BankingMode::Rom,
        }
    }
}

// 00 Simple ROM Banking Mode (default)
// 01 RAM Banking Mode / Advanced ROM Banking Mode
enum BankingMode {
    Rom,
    Ram,
}

impl BusConnection for MBC1 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0..=0x3FFF => self.rom_bank[address as usize],
            0x4000..=0x7FFF => self.rom_bank[(address - 0x4000) as usize], // TODO: Fix the addressing here
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram_bank[(address - 0xA000) as usize]
                } else { 0 }
            }
            _ => panic!("This should never happen, address: {:#02x} out of bounds for MBC1", address)
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => self.ram_enable = value == 0x0A,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x1F,
            0x4000..=0x5FFF => self.ram_bank_numer = value & 3,
            0x6000..=0x7FFF => {
                self.mode = match value & 1 {
                    0 => BankingMode::Rom,
                    1 => BankingMode::Ram,
                    _ => panic!("We've defied a law of mathematics!!"),
                }
            }
            // TODO: I don't think this is correct
            0xA000..=0xBFFF => self.ram_bank[(address - 0xA000) as usize + ((self.ram_bank_numer as usize * 8 * 1024) as usize)] = value,
            _ => panic!("This should never happen, address: {:#02x} out of bounds for MBC1", address)
        }
    }
}