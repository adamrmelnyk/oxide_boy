const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const NEGATIVE_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub struct FlagsRegister {
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<&FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.negative { 1 } else { 0 }) << NEGATIVE_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let negative = ((byte >> NEGATIVE_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            negative,
            half_carry,
            carry,
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_af(&self) -> u16 {
        let flags_register = &self.f;
        (self.b as u16) << 8 | u8::from(flags_register) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8)
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    /// Sets all flag registers in order: ZNHC
    pub fn set_flag_registers(
        &mut self,
        zero: bool,
        negative: bool,
        half_carry: bool,
        carry: bool,
    ) {
        self.f.zero = zero;
        self.f.negative = negative;
        self.f.half_carry = half_carry;
        self.f.carry = carry;
    }

    /// Sets all flag registers except for the zero register
    pub fn set_flag_registers_nz(
        &mut self,
        negative: bool,
        half_carry: bool,
        carry: bool,) {
            self.set_flag_registers(self.f.zero, negative, half_carry, carry);
    }
}
