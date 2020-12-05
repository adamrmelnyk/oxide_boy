use crate::cpu::memory::LoadType;

pub enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    ADDHL(SixteenBitArithmeticTarget),
    ADDSP(SixteenBitArithmeticTarget),
    INC16(SixteenBitArithmeticTarget),
    DEC16(SixteenBitArithmeticTarget),
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
    BIT,
    RESET,
    SET,
    SRL,
    RL,
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    JP(JumpCond),
    HALT,
    NOP,
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpCond),
    RET(JumpCond),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
            _ => None, // TODO: Add the rest
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC16(SixteenBitArithmeticTarget::BC)),
            0x13 => Some(Instruction::INC16(SixteenBitArithmeticTarget::DE)),
            _ => None, // TODO: Add the rest
        }
    }
}

pub enum JumpCond {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
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

pub enum SixteenBitArithmeticTarget {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum StackTarget {
    BC,
    DE,
    HL,
}
