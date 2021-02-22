use crate::dmg::busconnection::BusConnection;

use std::fs::File;
use std::io::Read;

const BOOT_ROM: &str = "src/dmg/rom/DMG_ROM.bin";

pub struct BootRom {
    rom: [u8; 0xFF + 1],
}

impl Default for BootRom {
    fn default() -> BootRom {
        let mut boot_rom = BootRom {
            rom: [0; 0xFF + 1],
        };
        boot_rom.load_boot_rom();
        boot_rom
    }
}

impl BootRom {
    /// Loads the boot rom from 0-0xFF
    fn load_boot_rom(&mut self) {
        let mut buffer = [0u8; 0xFF + 1];
        match File::open(BOOT_ROM) {
            Ok(mut file) => match file.read(&mut buffer[..]) {
                Ok(_bytes) => {
                    self.rom[0..=0xFF].copy_from_slice(&buffer);
                }
                Err(err) => eprintln!("Error reading file: {}", err),
            },
            Err(err) => eprintln!("Error opening file: {}", err),
        }
    }
}

impl BusConnection for BootRom {
    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        /* Do nothing this is ROM */
    }
}

#[test]
fn read_memory() {
    let rom = BootRom::default();
    assert_eq!(rom.read_byte(0), 0x31, "The first entry on the boot rom should be 0x31");
    assert_eq!(rom.read_byte(0xFF), 0x50, "The last entry on the boot rom should be 0x50");
}