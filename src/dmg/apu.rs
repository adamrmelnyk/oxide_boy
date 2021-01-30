/// The Audio Processing Unit
pub struct Apu {
    // Sound Mode 1 Registers
    // TODO: Consider making SoundMode1 into it's own object
    sweep_register: u8,           // 0xFF10
    length_wave_pattern_duty: u8, // 0xFF11
    envelope: u8,                 // 0xFF12
    frequency_lo: u8,             // 0xFF13
    frequency_hi: u8,             // 0xFF14

    // Sound Mode 2 Registers
    sm2_length_wave_pattern_duty: u8, // 0xFF16
    sm2_envelope: u8,                 //0xFF17
    sm2_frequency_lo: u8,             // 0xFF18
    sm2_frequency_hi: u8,             // 0xFF19

    // Sound Mode 3 Registers
    sm3_sound_toggle: u8,          // 0xFF1A
    sm3_sound_length: u8,          // 0xFF1B
    sm3_select_output_level: u8,   // 0xFF1C
    sm3_frequency_lower_data: u8,  //0xFF1D
    sm3_frequency_higher_data: u8, //0xFF1E

    // Sound Mode 4 Registers
    sm4_sound_length: u8,        // 0xFF20
    sm4_initial_volume: u8,      // 0xFF21
    sm4_polynomial_counter: u8,  //0xFF22
    sm4_counter_consecutive: u8, //0xFF23

    channel_control: u8,              //0xFF24
    sound_output_terminal: u8,        //0xFF25
    sound_on_off: u8,                 // 0xFF26
    wave_pattern_ram: WavePatternRam, // 0xFF30 - 0xFF3F // arbitrary sound data storage
}

impl Default for Apu {
    fn default() -> Apu {
        Apu {
            sweep_register: 0,
            length_wave_pattern_duty: 0,
            envelope: 0,
            frequency_lo: 0,
            frequency_hi: 0,
            sm2_length_wave_pattern_duty: 0,
            sm2_envelope: 0,
            sm2_frequency_lo: 0,
            sm2_frequency_hi: 0,
            sm3_sound_toggle: 0,
            sm3_sound_length: 0,
            sm3_select_output_level: 0,
            sm3_frequency_lower_data: 0,
            sm3_frequency_higher_data: 0,
            sm4_sound_length: 0,
            sm4_initial_volume: 0,
            sm4_polynomial_counter: 0,
            sm4_counter_consecutive: 0,
            channel_control: 0,
            sound_output_terminal: 0,
            sound_on_off: 0,
            wave_pattern_ram: WavePatternRam { ram: [0; 16] },
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

            0xFF16 => self.sm2_length_wave_pattern_duty,
            0xFF17 => self.sm2_envelope,
            0xFF18 => self.sm2_frequency_lo,
            0xFF19 => self.sm2_frequency_hi,

            0xFF1A => self.sm3_sound_toggle,
            0xFF1B => self.sm3_sound_length,
            0xFF1C => self.sm3_select_output_level,
            0xFF1D => self.sm3_frequency_lower_data,
            0xFF1E => self.sm3_frequency_higher_data,

            0xFF20 => self.sm4_sound_length,
            0xFF21 => self.sm4_initial_volume,
            0xFF22 => self.sm4_polynomial_counter,
            0xFF23 => self.sm4_counter_consecutive,

            0xFF24 => self.channel_control,
            0xFF25 => self.sound_output_terminal,
            0xFF26 => self.sound_on_off,
            0xFF30..=0xFF3F => self.wave_pattern_ram.read(address),
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

            0xFF16 => self.sm2_length_wave_pattern_duty = value,
            0xFF17 => self.sm2_envelope = value,
            0xFF18 => self.sm2_frequency_lo = value,
            0xFF19 => self.sm2_frequency_hi = value,

            0xFF1A => self.sm3_sound_toggle = value,
            0xFF1B => self.sm3_sound_length = value,
            0xFF1C => self.sm3_select_output_level = value,
            0xFF1D => self.sm3_frequency_lower_data = value,
            0xFF1E => self.sm3_frequency_higher_data = value,

            0xFF20 => self.sm4_sound_length = value,
            0xFF21 => self.sm4_initial_volume = value,
            0xFF22 => self.sm4_polynomial_counter = value,
            0xFF23 => self.sm4_counter_consecutive = value,

            0xFF24 => self.channel_control = value,
            0xFF25 => self.sound_output_terminal = value,
            0xFF26 => self.sound_on_off = value,
            0xFF30..=0xFF3F => self.wave_pattern_ram.write(address, value),
            _ => panic!("This should never happen"),
        }
    }

    #[cfg(test)]
    pub fn sweep_register(&self) -> u8 {
        self.sweep_register
    }

    #[cfg(test)]
    pub fn wave_pattern_ram(&self) -> [u8; 16] {
        self.wave_pattern_ram.ram()
    }
}

struct WavePatternRam {
    ram: [u8; 0xFF40 - 0xFF30],
}

impl WavePatternRam {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF30..=0xFF3F => self.ram[(address - 0xFF30) as usize],
            _ => panic!("This address: {:#02x} does not belong to the WavePatternRam", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF30..=0xFF3F => self.ram[(address - 0xFF30) as usize] = value,
            _ => panic!("This address: {:#02x} does not belong to the WavePatternRam", address),
        }
    }

    #[cfg(test)]
    fn ram(&self) -> [u8; 16] {
        self.ram
    }
}
