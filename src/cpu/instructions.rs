use crate::cpu::memory::{
    LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget,
};

#[derive(Debug, PartialEq)]
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
    BIT(u8, ArithmeticTarget),
    RESET(u8, ArithmeticTarget),
    SET(u8, ArithmeticTarget),
    SRL(ArithmeticTarget),
    RL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    JP(JumpCond),
    HALT,
    NOP,
    STOP,
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
        let l_nib = byte & 0x0F;
        let target = ArithmeticTarget::from(l_nib);
        match byte {
            0x00..=0x07 => Some(Instruction::RLC(target)),
            0x08..=0x0F => Some(Instruction::RRC(target)),
            0x10..=0x17 => Some(Instruction::RL(target)),
            0x18..=0x1F => Some(Instruction::RR(target)),
            0x20..=0x27 => Some(Instruction::SLA(target)),
            0x28..=0x2F => Some(Instruction::SRA(target)),
            0x30..=0x37 => Some(Instruction::SWAP(target)),
            0x38..=0x3F => Some(Instruction::SRL(target)),
            0x40..=0x7F => Some(Instruction::BIT(from_byte_to_index(byte), target)),
            0x80..=0xBF => Some(Instruction::RESET(from_byte_to_index(byte), target)),
            0xC0..=0xFF => Some(Instruction::SET(from_byte_to_index(byte), target)),
        }
    }

    /// Takes an byte instruction and returns an optional Instruction
    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x10 => Some(Instruction::STOP),
            0x76 => Some(Instruction::HALT),
            0x01 | 0x11 | 0x21 | 0x31 => Some(Instruction::LD(LoadType::from(byte))),
            0x03 => Some(Instruction::INC16(SixteenBitArithmeticTarget::BC)),
            0x13 => Some(Instruction::INC16(SixteenBitArithmeticTarget::DE)),
            0x23 => Some(Instruction::INC16(SixteenBitArithmeticTarget::HL)),
            0x33 => Some(Instruction::INC16(SixteenBitArithmeticTarget::SP)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x34 => Some(Instruction::INC(ArithmeticTarget::HLI)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x35 => Some(Instruction::DEC(ArithmeticTarget::HLI)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x40..=0x7F => Some(Instruction::LD(LoadType::from(byte))),
            0x80..=0x87 | 0xC6 => Some(Instruction::ADD(ArithmeticTarget::from(byte))),
            0x88..=0x8F | 0xCE => Some(Instruction::ADC(ArithmeticTarget::from(byte))),
            0x90..=0x97 | 0xD6 => Some(Instruction::SUB(ArithmeticTarget::from(byte))),
            0x98..=0x9F | 0xDE => Some(Instruction::SBC(ArithmeticTarget::from(byte))),
            0xA0..=0xA7 | 0xE6 => Some(Instruction::AND(ArithmeticTarget::from(byte))),
            0xA8..=0xAF | 0xEE => Some(Instruction::XOR(ArithmeticTarget::from(byte))),
            0xB0..=0xB7 | 0xF6 => Some(Instruction::OR(ArithmeticTarget::from(byte))),
            0xB8..=0xBF | 0xFE => Some(Instruction::CP(ArithmeticTarget::from(byte))),
            0x09 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::BC)),
            0x19 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::DE)),
            0x29 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::HL)),
            0x39 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::SP)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xC0 => Some(Instruction::RET(JumpCond::NotZero)),
            0xD0 => Some(Instruction::RET(JumpCond::NotCarry)),
            0xC8 => Some(Instruction::RET(JumpCond::Zero)),
            0xD8 => Some(Instruction::RET(JumpCond::Carry)),
            0xC9 => Some(Instruction::RET(JumpCond::Always)),
            0xC2 => Some(Instruction::JP(JumpCond::NotZero)),
            0xD2 => Some(Instruction::JP(JumpCond::NotCarry)),
            0xC3 => Some(Instruction::JP(JumpCond::Always)),
            0xCB => panic!("This is a prefixed byte! This should never happen!"),
            _ => None, // TODO: Add the rest
        }
    }
}

/// Helper method for returning the index specified by
/// the byte instruction
fn from_byte_to_index(byte: u8) -> u8 {
    match byte {
        0x00..=0x3F => panic!("Incorrect instruction"),
        0x40..=0x47 | 0x80..=0x87 | 0xC0..=0xC7 => 0,
        0x48..=0x4F | 0x88..=0x8F | 0xC8..=0xCF => 1,
        0x50..=0x57 | 0x90..=0x97 | 0xD0..=0xD7 => 2,
        0x58..=0x5F | 0x98..=0x9F | 0xD8..=0xDF => 3,
        0x60..=0x67 | 0xA0..=0xA7 | 0xE0..=0xE7 => 4,
        0x68..=0x6F | 0xA8..=0xAF | 0xE8..=0xEF => 5,
        0x70..=0x77 | 0xB0..=0xB7 | 0xF0..=0xF7 => 6,
        0x78..=0x7F | 0xB8..=0xBF | 0xF8..=0xFF => 7,
    }
}

#[derive(Debug, PartialEq)]
pub enum JumpCond {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    D8,
}

impl std::convert::From<u8> for ArithmeticTarget {
    fn from(byte: u8) -> ArithmeticTarget {
        match byte {
            0xC6 | 0xD6 | 0xE6 | 0xF6 | 0xCE | 0xDE | 0xEE | 0xFE => ArithmeticTarget::D8,
            _ => {
                let nibble = byte & 0x0F;
                match nibble {
                    0x0 | 0x8 => ArithmeticTarget::B,
                    0x1 | 0x9 => ArithmeticTarget::C,
                    0x2 | 0xA => ArithmeticTarget::D,
                    0x3 | 0xB => ArithmeticTarget::E,
                    0x4 | 0xC => ArithmeticTarget::H,
                    0x5 | 0xD => ArithmeticTarget::L,
                    0x6 | 0xE => ArithmeticTarget::HLI,
                    0x7 | 0xF => ArithmeticTarget::A,
                    _ => panic!(
                        "u8 {:?} cannot be converted into an ArithmeticTarget",
                        nibble
                    ),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SixteenBitArithmeticTarget {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, PartialEq)]
pub enum StackTarget {
    AF,
    BC,
    DE,
    HL,
}
