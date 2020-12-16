pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl Default for MemoryBus {
    fn default() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&self, address: u16, value: u8) {
        unimplemented!();
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
}

impl std::convert::From<u8> for LoadByteTarget {
    fn from(byte: u8) -> LoadByteTarget {
        match byte {
            0x40..=0x47 | 0x06 => LoadByteTarget::B,
            0x48..=0x4F => LoadByteTarget::C,
            0x50..=0x57 | 0x16 => LoadByteTarget::D,
            0x58..=0x5F => LoadByteTarget::E,
            0x60..=0x67 | 0x26 => LoadByteTarget::H,
            0x68..=0x6F => LoadByteTarget::L,
            0x70..=0x75 | 0x77 | 0x36 => LoadByteTarget::HLI,
            0x78..=0x7F => LoadByteTarget::A,
            _ => panic!("u8 {:?} cannot be converted into an LoadByteTarget", byte),
        }
    }
}

impl std::convert::From<u8> for LoadByteSource {
    fn from(byte: u8) -> LoadByteSource {
        // TODO: Add the other bytes
        match byte {
            0x06 | 0x16 | 0x26 | 0x36 => LoadByteSource::D8,
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
    D8,  // direct 8 bit value, read next byte
    HLI, // read from the address stored in HL
}

#[derive(Debug, PartialEq)]
pub enum LoadWordSource {
    BC,
    DE,
    HL,
    // SP, TODO: can you read from the SP as well?
    D16, // direct 16 bit value, read the next two bytes
}

#[derive(Debug, PartialEq)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
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
            // TODO: The load types should all have the same arm, whatever looks the least messy
            0x40..=0x7F => LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(byte)),
            0x06 | 0x16 | 0x26 | 0x36 => {
                LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(byte))
            }
            0x02 | 0x12 | 0x22 | 0x32 => unimplemented!(),
            0x0A | 0x1A | 0x2A | 0x3A => unimplemented!(),
            0x0E | 0x1E | 0x2E | 0x3E => unimplemented!(),
            _ => panic!("u8 {:?} cannot be converted into a LoadType", byte),
        }
    }
}
