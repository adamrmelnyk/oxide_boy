use crate::dmg::busconnection::BusConnection;

pub struct RomOnly {
    rom: Vec<u8>,
}

impl RomOnly {
    pub fn new(data: Vec<u8>) -> RomOnly {
        RomOnly { rom: data }
    }
}

impl BusConnection for RomOnly {
    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        /* Do Nothing, this is ROM */
    }
}
