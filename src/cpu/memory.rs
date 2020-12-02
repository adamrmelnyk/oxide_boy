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

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}
