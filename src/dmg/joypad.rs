use crate::dmg::busconnection::BusConnection;

pub struct Joypad {
    unused_bit_7: bool,
    unused_bit_6: bool,
    select_button_keys: bool,    // P15
    select_direction_keys: bool, // P14
    down_or_start: bool,         // P13
    up_or_select: bool,          // P12
    left_or_button_b: bool,      // P11
    right_or_button_a: bool,     // P10
}

impl Default for Joypad {
    fn default() -> Joypad {
        Joypad {
            unused_bit_7: false,
            unused_bit_6: false,
            select_button_keys: false,
            select_direction_keys: false,
            down_or_start: false,
            up_or_select: false,
            left_or_button_b: false,
            right_or_button_a: false,
        }
    }
}

impl BusConnection for Joypad {
    fn write_byte(&mut self, address: u16, value: u8) {
        if address == 0xFF00 {
            self.unused_bit_7 = (value & 0x80) == 0x80;
            self.unused_bit_6 = (value & 0x40) == 0x40;
            self.select_button_keys = (value & 0x20) == 0x20;
            self.select_direction_keys = (value & 0x10) == 0x10;
            self.down_or_start = (value & 0x8) == 0x08;
            self.up_or_select = (value & 0x04) == 0x04;
            self.left_or_button_b = (value & 0x02) == 0x02;
            self.right_or_button_a = (value & 0x01) == 0x01;
        } else {
            panic!("The Address: {:#02x}, is not use by the Joypad", address)
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address == 0xFF00 {
            (if self.unused_bit_7 { 1 } else { 0 }) << 7
                | (if self.unused_bit_6 { 1 } else { 0 }) << 6
                | (if self.select_button_keys { 1 } else { 0 }) << 5
                | (if self.select_direction_keys { 1 } else { 0 }) << 4
                | (if self.down_or_start { 1 } else { 0 }) << 3
                | (if self.up_or_select { 1 } else { 0 }) << 2
                | (if self.left_or_button_b { 1 } else { 0 }) << 1
                | (if self.right_or_button_a { 1 } else { 0 })
        } else {
            panic!("The Address: {:#02x}, is not use by the Joypad", address)
        }
    }
}
