use crate::dmg::busconnection::BusConnection;

use std::fs::File;
use std::io::Read;

const BOOT_ROM: &str = "src/dmg/rom/DMG_ROM.bin";

pub struct BootRom {
    rom: [u8; 0xFF + 1],
}

impl Default for BootRom {
    fn default() -> BootRom {
        BootRom {
            rom: load_boot_rom(),
        }
    }
}

fn load_boot_rom() -> [u8; 0x100] {
    let mut buffer = [0u8; 0xFF + 1];
    match File::open(BOOT_ROM) {
        Ok(mut file) => match file.read(&mut buffer[..]) {
            Ok(_bytes) => buffer,
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                [0u8; 0x100]
            }
        },
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            [0u8; 0x100]
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