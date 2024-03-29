use crate::dmg::busconnection::BusConnection;

/// These values are taken from The pan docs at https://gbdev.io/pandocs/#memory-map
/// There are other blogs and sites with conflicting information, but these values seem
/// to be be correct.
const INTERNAL_RAM_START: u16 = 0xC000;

#[cfg(test)]
const INTERNAL_RAM_END: u16 = 0xDDFF;

/// This RAM is a mirror echo of the above memory map. Any attempt to write to
const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

pub enum Interrupt {
    VBlank,
    LcdStat,
    TimerOverflow,
    SerialLink,
    JoypadPress,
    NONE,
}

pub struct Memory {
    memory: [u8; 0xFFFF + 1],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: [0; 0xFFFF + 1],
        }
    }
}

impl BusConnection for Memory {
    /// Reads bytes from memory, or from the boot rom if 0xFF50 is zero
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.memory[(address - ECHO_RAM_START + INTERNAL_RAM_START) as usize]
            }
            _ => self.memory[address as usize],
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.memory[(address - ECHO_RAM_START + INTERNAL_RAM_START) as usize] = value
            }
            _ => self.memory[address as usize] = value,
        }
    }
}

impl Memory {
    fn interrupt_enable(&self) -> u8 {
        self.memory[0xFFFF]
    }

    fn interrupt_flags(&self) -> u8 {
        self.memory[0xFF0F]
    }

    pub fn interrupt_flag_off(&mut self) {
        self.write_byte(0xFF0F, 0);
    }

    pub fn return_interrupt(&self) -> Interrupt {
        if (self.interrupt_enable() & 0x01) & (self.interrupt_flags() & 0x01) == 1 {
            Interrupt::VBlank
        } else if (self.interrupt_enable() & 0x02) & (self.interrupt_flags() & 0x02) == 1 {
            Interrupt::LcdStat
        } else if (self.interrupt_enable() & 0x03) & (self.interrupt_flags() & 0x03) == 1 {
            Interrupt::TimerOverflow
        } else if (self.interrupt_enable() & 0x04) & (self.interrupt_flags() & 0x04) == 1 {
            Interrupt::SerialLink
        } else if (self.interrupt_enable() & 0x05) & (self.interrupt_flags() & 0x05) == 1 {
            Interrupt::JoypadPress
        } else {
            Interrupt::NONE
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI, // read from the address stored in HL
    BCI,
    DEI,
    HLINC,
    HLDEC,
}

impl std::convert::From<u8> for LoadByteTarget {
    fn from(byte: u8) -> LoadByteTarget {
        match byte {
            0x02 => LoadByteTarget::BCI,
            0x12 => LoadByteTarget::DEI,
            0x22 => LoadByteTarget::HLINC,
            0x32 => LoadByteTarget::HLDEC,
            0x40..=0x47 | 0x06 => LoadByteTarget::B,
            0x48..=0x4F | 0x0E => LoadByteTarget::C,
            0x50..=0x57 | 0x16 => LoadByteTarget::D,
            0x58..=0x5F | 0x1E => LoadByteTarget::E,
            0x60..=0x67 | 0x26 => LoadByteTarget::H,
            0x68..=0x6F | 0x2E => LoadByteTarget::L,
            0x70..=0x75 | 0x77 | 0x36 => LoadByteTarget::HLI,
            0x78..=0x7F | 0x3E | 0x0A | 0x1A | 0x2A | 0x3A => LoadByteTarget::A,
            _ => panic!("u8 {:?} cannot be converted into an LoadByteTarget", byte),
        }
    }
}

impl std::convert::From<u8> for LoadByteSource {
    fn from(byte: u8) -> LoadByteSource {
        match byte {
            0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => LoadByteSource::D8,
            0x02 | 0x12 | 0x22 | 0x32 => LoadByteSource::A,
            0x0A => LoadByteSource::BCI,
            0x1A => LoadByteSource::DEI,
            0x2A => LoadByteSource::HLINC,
            0x3A => LoadByteSource::HLDEC,
            _ => {
                let nibble = byte & 0x0F;
                match nibble {
                    0x0 | 0x8 => LoadByteSource::B,
                    0x1 | 0x9 => LoadByteSource::C,
                    0x2 | 0xA => LoadByteSource::D,
                    0x3 | 0xB => LoadByteSource::E,
                    0x4 | 0xC => LoadByteSource::H,
                    0x5 | 0xD => LoadByteSource::L,
                    0x6 | 0xE => LoadByteSource::HLI,
                    0x7 | 0xF => LoadByteSource::A,
                    _ => panic!("u8 {:?} cannot be converted into an LoadByteSource", nibble),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
    BCI,
    DEI,
    HLINC,
    HLDEC,
}

#[derive(Debug, PartialEq)]
pub enum LoadWordSource {
    BC,
    DE,
    HL,
    SP,
    D16, // direct 16 bit value, read the next two bytes
}

#[derive(Debug, PartialEq)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
    D16, // direct 16 bit value, read the next two bytes
}

#[derive(Debug, PartialEq)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
}

impl std::convert::From<u8> for LoadType {
    fn from(byte: u8) -> LoadType {
        match byte {
            0x01 => LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16),
            0x11 => LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16),
            0x21 => LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16),
            0x31 => LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16),
            0x08 => LoadType::Word(LoadWordTarget::D16, LoadWordSource::SP),
            0xF9 => LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL),
            0x40..=0x7F
            | 0x06 | 0x16 | 0x26 | 0x36
            | 0x02 | 0x12 | 0x22 | 0x32
            | 0x0A | 0x1A | 0x2A | 0x3A
            | 0x0E | 0x1E | 0x2E | 0x3E => LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(byte)),
            _ => panic!("u8 {:?} cannot be converted into a LoadType", byte),
        }
    }
}

#[test]
fn read_word() {
    let mut bus = Memory::default();
    bus.write_byte(0xA000, 0xAA);
    bus.write_byte(0xA001, 0xFF);
    assert_eq!(
        bus.read_word(0xA000),
        0xFFAA,
        "We expect this to be read as little endian"
    );
}

#[test]
fn write_word() {
    let mut bus = Memory::default();
    bus.write_word(0xA000, 0xFFAA);
    assert_eq!(bus.read_byte(0xA000), 0xAA);
    assert_eq!(bus.read_byte(0xA001), 0xFF);
}

#[test]
fn echo_ram() {
    let mut bus = Memory::default();
    bus.write_byte(INTERNAL_RAM_START + 1, 0xAA);
    let mut j = 0;
    for i in INTERNAL_RAM_START..=INTERNAL_RAM_END {
        assert_eq!(bus.read_byte(ECHO_RAM_START + j), bus.read_byte(i));
        j += 1;
    }
}

#[test]
fn write_to_echo_ram() {
    let mut bus = Memory::default();
    bus.write_byte(ECHO_RAM_START + 10, 0xAA);
    assert_eq!(bus.read_byte(INTERNAL_RAM_START + 10), 0xAA);
}
