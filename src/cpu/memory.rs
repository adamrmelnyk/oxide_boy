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
