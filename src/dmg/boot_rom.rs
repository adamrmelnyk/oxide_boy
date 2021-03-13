use crate::dmg::busconnection::BusConnection;

use std::fs::File;
use std::io::Read;

use log::error;

const BOOT_ROM: &str = "src/dmg/rom/DMG_ROM.bin";

pub struct BootRom {
    rom: [u8; 0xFF + 1],
    enabled: bool,
}

impl Default for BootRom {
    fn default() -> BootRom {
        BootRom {
            rom: load_boot_rom(),
            enabled: true,
        }
    }
}

fn load_boot_rom() -> [u8; 0x100] {
    let mut buffer = [0u8; 0xFF + 1];
    match File::open(BOOT_ROM) {
        Ok(mut file) => match file.read(&mut buffer[..]) {
            Ok(_bytes) => buffer,
            Err(err) => {
                error!("Error reading file: {}", err);
                [0u8; 0x100]
            }
        },
        Err(err) => {
            error!("Error opening file: {}", err);
            [0u8; 0x100]
        }
    }
}

impl BusConnection for BootRom {
    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0..=0xFF => { /* Do nothing this is ROM */ }
            0xFF50 => self.enabled = value == 0,
            _ => panic!("This should never happen!"),
        }
    }
}

impl BootRom {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[test]
fn read_memory() {
    let rom = BootRom::default();
    assert_eq!(
        rom.read_byte(0),
        0x31,
        "The first entry on the boot rom should be 0x31"
    );
    assert_eq!(
        rom.read_byte(0xFF),
        0x50,
        "The last entry on the boot rom should be 0x50"
    );
}
