pub mod memory_bus {
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
    }
}
