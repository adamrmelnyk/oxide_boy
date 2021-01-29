/// The Audio Processing Unit
pub struct Apu {
    // Sound Mode 1 Registers
    // TODO: Consider making SoundMode1 into it's own object
    sweep_register: u8, // 0xFF10
    length_wave_pattern_duty: u8, // 0xFF11
    envelope: u8, // 0xFF12
    frequency_lo: u8,// 0xFF13
    frequency_hi: u8, // 0xFF14

    // Sound Mode 2 Registers
    //TODO: 0xFF16..FF26
}

impl Default for Apu {
    fn default() -> Apu {
        Apu {
            sweep_register: 0,
            length_wave_pattern_duty: 0,
            envelope: 0,
            frequency_lo: 0,
            frequency_hi: 0,
        }
    }
}

impl Apu {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.sweep_register,
            0xFF11 => self.length_wave_pattern_duty,
            0xFF12 => self.envelope,
            0xFF13 => self.frequency_lo,
            0xFF14 => self.frequency_hi,
            _ => panic!("This should never happen"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.sweep_register = value,
            0xFF11 => self.length_wave_pattern_duty = value,
            0xFF12 => self.envelope = value,
            0xFF13 => self.frequency_lo = value,
            0xFF14 => self.frequency_hi = value,
            _ => panic!("This should never happen"),
        }
    }

    #[cfg(test)]
    pub fn sweep_register(&self) -> u8 {
        self.sweep_register
    }
}