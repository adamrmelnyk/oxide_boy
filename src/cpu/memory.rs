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
            0x40..=0x47 => LoadByteTarget::B,
            0x48..=0x4F => LoadByteTarget::C,
            0x50..=0x57 => LoadByteTarget::D,
            0x58..=0x5F => LoadByteTarget::E,
            0x60..=0x67 => LoadByteTarget::H,
            0x68..=0x6F => LoadByteTarget::L,
            0x70..=0x75 | 0x77 => LoadByteTarget::HLI,
            0x78..=0x7F => LoadByteTarget::A,
            _ => panic!("u8 {:?} cannot be converted into an LoadByteTarget", byte),
        }
    }
}

impl std::convert::From<u8> for LoadByteSource {
    fn from(nibble: u8) -> LoadByteSource {
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
            0x40..=0x7F => {
                let l_nib = byte & 0x0F;
                LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(l_nib))
            }
            _ => panic!("u8 {:?} cannot be converted into a LoadType", byte),
        }
    }
}
