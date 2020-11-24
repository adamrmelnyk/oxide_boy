#![feature(destructuring_assignment)]

pub struct CPU {
    registers: Registers,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagsRegister {
                    zero: false,
                    subtract: false,
                    carry: false,
                    half_carry: false,
                },
                h: 0,
                l: 0,
            },
        }
    }
}

impl CPU {
    fn set_register_by_target(&mut self, target: &ArithmeticTarget, value: u8) {
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // TODO: af register

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

// TODO: We'll need this for setting the flag registers
fn kth_bit_set(value: u16, k: u8) -> bool {
    (value & (1 << (k - 1))) > 0
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
    CPL,
    // BIT,
    // RESET,
    // SET,
    // SRL,
    // RL,
    // RRC,
    // RLC,
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.add(value);
            }
            Instruction::ADDHL(target) => {}
            Instruction::SUB(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.sub(value);
            }
            Instruction::ADC(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.adc(value);
            }
            Instruction::SBC(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.sbc(value);
            }
            Instruction::AND(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.and(value);
            }
            Instruction::OR(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.or(value);
            }
            Instruction::XOR(target) => {
                let value = self.register_value(&target);
                self.registers.a = self.xor(value);
            }
            Instruction::CP(target) => {}
            Instruction::INC(target) => {
                let value = self.register_value(&target);
                let new_value = self.inc(value);
                self.set_register_by_target(&target, new_value);
            }
            Instruction::DEC(target) => {
                let value = self.register_value(&target);
                let new_value = self.dec(value);
                self.set_register_by_target(&target, new_value);
            }
            Instruction::CCF => {}
            Instruction::SCF => {}
            Instruction::RRA => {}
            Instruction::RLA => {}
            Instruction::RRCA => {}
            Instruction::RRLA => {}
            Instruction::CPL => {}
            Instruction::SRA(target) => {}
            Instruction::SLA(target) => {}
            Instruction::SWAP(target) => {}
        }
    }

    /// Helper method for returning the value of an 8bit register
    fn register_value(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    // A = A + s
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn addhl(&mut self, value: u8) {}

    // A = A + s + CY
    fn adc(&mut self, value: u8) -> u8 {
        // using #![feature(destructuring_assignment)] from nightly so I don't neeed to do this
        // let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);
        // if self.registers.f.carry {
        //     let (t_new_value, t_did_overflow) = self.registers.a.overflowing_add(1u8); // using #![feature(destructuring_assignment)] for this
        //     new_value = t_new_value;
        //     did_overflow = t_did_overflow;
        // }
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);
        if self.registers.f.carry {
            (new_value, did_overflow) = self.registers.a.overflowing_add(1u8);
        }
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // A = A - s
    fn sub(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a.wrapping_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = self.registers.a < value;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
        new_value
    }

    // A = A - s -CY
    fn sbc(&mut self, value: u8) -> u8 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let new_value = self.registers.a - value - carry;
        self.registers.f.carry = false;
        self.registers.f.zero = new_value == 0;
        // TODO: Set the carry bits
        // self.registers.f.subtract = true;
        // self.registers.f.half_carry = false;
        new_value
    }

    // A = A & s
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A = A | s
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A = A ^ s
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A - s
    fn cp() {}

    // s = s + 1
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) + (1 & 0xF) > 0xF; // TODO: Double check this
                                                                       // Carry not affected
        new_value
    }

    // s = s - 1
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) < 1; // TODO; Double check this
                                                         // Carry not affected
        new_value
    }

    fn ccf() {}

    fn scf() {}

    fn rra() {}

    fn rla() {}

    fn rrca() {}

    fn rrla() {}

    fn cpl() {}

    fn bit() {}

    fn reset() {}

    fn set() {}

    fn srl() {}

    fn rr() {}

    fn rl() {}

    fn rrc() {}

    fn rlc() {}

    fn sra() {}

    fn sla() {}

    fn swap() {}
}
