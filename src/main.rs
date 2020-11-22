struct CPU {
    registers: Registers,
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

    // TODO: de register

    // TODO: hl register
}

enum Instruction {
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
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let curr = self.register_value(target);
                self.registers.a = self.add(curr);
            }
            Instruction::SUB(target) => {
                let curr = self.register_value(target);
                self.registers.a = self.sub(curr);
            }
            _ => {
                // TODO: Support more instructions
            }
        }
    }

    /// Helper method for returning the value of an 8bit register
    fn register_value(&self, target: ArithmeticTarget) -> u8 {
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
    fn adc(&mut self, value: u8) {}

    // A = A - s
    fn sub(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a - value;
        self.registers.f.carry = false;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // self.registers.f.half_carry = false; // TODO
        new_value
    }

    fn sbc() {}

    fn and() {}

    fn or() {}

    fn xor() {}

    fn cp() {}

    fn inc() {}

    fn dec() {}

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

fn main() {
    println!("Hello, world!");
    let mut cpu = CPU {
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
    };
    cpu.add(8);
}
