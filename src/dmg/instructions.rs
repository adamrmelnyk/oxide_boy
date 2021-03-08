use log::info;

use crate::dmg::memory::LoadType;

const PREFIXED_OPERATION_CYCLES: [u8; 256] = [
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8
];

// Note: Some of these operations include conditionals where there are two numbers
// One where a conditional is taken and one where it is not, the conditional_cycle function
// is used for these cases
const OPERATION_CYCLES: [u8; 256] = [
    4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4,
    4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    8, 12, 8, 8, 4, 4, 8, 4, 8, 8, 8, 8, 4, 4, 8, 4,
    8, 12, 8, 8, 12, 12, 12, 4, 8, 8, 8, 8, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    8, 12, 12, 16, 12, 16, 8, 16, 8, 16, 12, 4, 12, 24, 8, 16,
    8, 12, 12, 0, 12, 16, 8, 16, 8, 16, 12, 0, 12, 0, 8, 16,
    12, 12, 8, 0, 0, 16, 8, 16, 16, 4, 16, 0, 0, 0, 8, 16,
    12, 12, 8, 4, 0, 16, 8, 16, 12, 8, 16, 4, 0, 0, 8, 16
];

fn conditional_cycle(byte: u8) -> u8 {
    match byte {
        0xE9 => 4,
        0x20 | 0x30 | 0x18 | 0x28 | 0x38 => 12,
        0xC0 | 0xD0 | 0xC8 | 0xD8 => 20,
        0xC2 | 0xD2 | 0xC3 | 0xCA | 0xDA | 0xC9 => 16,
        0xC4 | 0xD4 | 0xCC | 0xDC | 0xCD => 24,

        _ => panic!("u8 {:?} cannot be converted into an JumpCond", byte),
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ADD(ArithmeticTarget, u8),
    SUB(ArithmeticTarget, u8),
    ADDHL(SixteenBitArithmeticTarget, u8),
    ADDSP(u8),
    INC16(SixteenBitArithmeticTarget, u8),
    DEC16(SixteenBitArithmeticTarget, u8),
    ADC(ArithmeticTarget, u8),
    SBC(ArithmeticTarget, u8),
    AND(ArithmeticTarget, u8),
    OR(ArithmeticTarget, u8),
    XOR(ArithmeticTarget, u8),
    CP(ArithmeticTarget, u8),
    INC(ArithmeticTarget, u8),
    DEC(ArithmeticTarget, u8),
    CCF(u8),
    SCF(u8),
    RRA(u8),
    RLA(u8),
    RRCA(u8),
    RLCA(u8),
    DAA(u8),
    CPL(u8),
    BIT(u8, ArithmeticTarget, u8),
    RESET(u8, ArithmeticTarget, u8),
    SET(u8, ArithmeticTarget, u8),
    SRL(ArithmeticTarget, u8),
    RL(ArithmeticTarget, u8),
    RR(ArithmeticTarget, u8),
    RRC(ArithmeticTarget, u8),
    RLC(ArithmeticTarget, u8),
    SRA(ArithmeticTarget, u8),
    SLA(ArithmeticTarget, u8),
    SWAP(ArithmeticTarget, u8),
    JP(JumpCond, u8, u8),
    JPHL(u8),
    HALT(u8),
    NOP(u8),
    JR(JumpCond, u8, u8),
    STOP(u8),
    LD(LoadType, u8),
    PUSH(StackTarget, u8),
    POP(StackTarget, u8),
    CALL(JumpCond, u8, u8),
    RET(JumpCond, u8, u8),
    RETI(u8),
    RST(RestartAddr, u8),
    EI(u8),
    DI(u8),
    LDHA(u8),
    LDHA8(u8),
    LDABY(u8),
    LDA(u8),
    LDAC(u8),   // Put value at address $FF00 + register C into A.  Same as: LD A,($FF00+C)
    LDCA(u8),   // Put A into LDABY address $FF00 + register C.
    LDHLSP(u8), // Put sp plus n effective address into hl
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
        let cycles = PREFIXED_OPERATION_CYCLES[byte as usize];
        match byte {
            0x00..=0x07 => Some(Instruction::RLC(target, cycles)),
            0x08..=0x0F => Some(Instruction::RRC(target, cycles)),
            0x10..=0x17 => Some(Instruction::RL(target, cycles)),
            0x18..=0x1F => Some(Instruction::RR(target, cycles)),
            0x20..=0x27 => Some(Instruction::SLA(target, cycles)),
            0x28..=0x2F => Some(Instruction::SRA(target, cycles)),
            0x30..=0x37 => Some(Instruction::SWAP(target, cycles)),
            0x38..=0x3F => Some(Instruction::SRL(target, cycles)),
            0x40..=0x7F => Some(Instruction::BIT(from_byte_to_index(byte), target, cycles)),
            0x80..=0xBF => Some(Instruction::RESET(from_byte_to_index(byte), target, cycles)),
            0xC0..=0xFF => Some(Instruction::SET(from_byte_to_index(byte), target, cycles)),
        }
    }

    /// Takes an byte instruction and returns an optional Instruction
    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        let cycles = OPERATION_CYCLES[byte as usize];
        match byte {
            0x00 => Some(Instruction::NOP(cycles)),
            0x10 => Some(Instruction::STOP(cycles)),
            0x20 | 0x30 | 0x18 | 0x28 | 0x38 => Some(Instruction::JR(
                JumpCond::from(byte),
                cycles,
                conditional_cycle(byte),
            )),
            0x76 => Some(Instruction::HALT(cycles)),
            0x40..=0x7F
            | 0x01 | 0x11 | 0x21 | 0x31
            | 0x06 | 0x16 | 0x26 | 0x36
            | 0x02 | 0x12 | 0x22 | 0x32
            | 0x0A | 0x1A | 0x2A | 0x3A
            | 0x0E | 0x1E | 0x2E | 0x3E
            | 0x08 | 0xF9 => Some(Instruction::LD(LoadType::from(byte), cycles)),
            0xE2 => Some(Instruction::LDCA(cycles)),
            0xF2 => Some(Instruction::LDAC(cycles)),
            0x03 => Some(Instruction::INC16(SixteenBitArithmeticTarget::BC, cycles)),
            0x13 => Some(Instruction::INC16(SixteenBitArithmeticTarget::DE, cycles)),
            0x23 => Some(Instruction::INC16(SixteenBitArithmeticTarget::HL, cycles)),
            0x33 => Some(Instruction::INC16(SixteenBitArithmeticTarget::SP, cycles)),
            0x0B => Some(Instruction::DEC16(SixteenBitArithmeticTarget::BC, cycles)),
            0x1B => Some(Instruction::DEC16(SixteenBitArithmeticTarget::DE, cycles)),
            0x2B => Some(Instruction::DEC16(SixteenBitArithmeticTarget::HL, cycles)),
            0x3B => Some(Instruction::DEC16(SixteenBitArithmeticTarget::SP, cycles)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B, cycles)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D, cycles)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H, cycles)),
            0x34 => Some(Instruction::INC(ArithmeticTarget::HLI, cycles)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B, cycles)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D, cycles)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::L, cycles)),
            0x35 => Some(Instruction::DEC(ArithmeticTarget::HLI, cycles)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C, cycles)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E, cycles)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L, cycles)),
            0x3C => Some(Instruction::INC(ArithmeticTarget::A, cycles)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C, cycles)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E, cycles)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L, cycles)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A, cycles)),
            0x0F => Some(Instruction::RRCA(cycles)),
            0x1F => Some(Instruction::RRA(cycles)),
            0x07 => Some(Instruction::RLCA(cycles)),
            0x17 => Some(Instruction::RLA(cycles)),
            0x27 => Some(Instruction::DAA(cycles)),
            0x37 => Some(Instruction::SCF(cycles)),
            0x2F => Some(Instruction::CPL(cycles)),
            0x3F => Some(Instruction::CCF(cycles)),
            0x80..=0x87 | 0xC6 => Some(Instruction::ADD(ArithmeticTarget::from(byte), cycles)),
            0x88..=0x8F | 0xCE => Some(Instruction::ADC(ArithmeticTarget::from(byte), cycles)),
            0x90..=0x97 | 0xD6 => Some(Instruction::SUB(ArithmeticTarget::from(byte), cycles)),
            0x98..=0x9F | 0xDE => Some(Instruction::SBC(ArithmeticTarget::from(byte), cycles)),
            0xA0..=0xA7 | 0xE6 => Some(Instruction::AND(ArithmeticTarget::from(byte), cycles)),
            0xA8..=0xAF | 0xEE => Some(Instruction::XOR(ArithmeticTarget::from(byte), cycles)),
            0xB0..=0xB7 | 0xF6 => Some(Instruction::OR(ArithmeticTarget::from(byte), cycles)),
            0xB8..=0xBF | 0xFE => Some(Instruction::CP(ArithmeticTarget::from(byte), cycles)),
            0x09 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::BC, cycles)),
            0x19 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::DE, cycles)),
            0x29 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::HL, cycles)),
            0x39 => Some(Instruction::ADDHL(SixteenBitArithmeticTarget::SP, cycles)),
            0xE8 => Some(Instruction::ADDSP(cycles)),
            0xC1 => Some(Instruction::POP(StackTarget::BC, cycles)),
            0xD1 => Some(Instruction::POP(StackTarget::DE, cycles)),
            0xE1 => Some(Instruction::POP(StackTarget::HL, cycles)),
            0xF1 => Some(Instruction::POP(StackTarget::AF, cycles)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC, cycles)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE, cycles)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL, cycles)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF, cycles)),
            0xC0 => Some(Instruction::RET(
                JumpCond::NotZero,
                cycles,
                conditional_cycle(byte),
            )),
            0xD0 => Some(Instruction::RET(
                JumpCond::NotCarry,
                cycles,
                conditional_cycle(byte),
            )),
            0xC8 => Some(Instruction::RET(
                JumpCond::Zero,
                cycles,
                conditional_cycle(byte),
            )),
            0xD8 => Some(Instruction::RET(
                JumpCond::Carry,
                cycles,
                conditional_cycle(byte),
            )),
            0xC9 => Some(Instruction::RET(
                JumpCond::Always,
                cycles,
                conditional_cycle(byte),
            )),
            0xD9 => Some(Instruction::RETI(cycles)),
            0xC2 => Some(Instruction::JP(
                JumpCond::NotZero,
                cycles,
                conditional_cycle(byte),
            )),
            0xD2 => Some(Instruction::JP(
                JumpCond::NotCarry,
                cycles,
                conditional_cycle(byte),
            )),
            0xC3 => Some(Instruction::JP(
                JumpCond::Always,
                cycles,
                conditional_cycle(byte),
            )),
            0xCA => Some(Instruction::JP(
                JumpCond::Zero,
                cycles,
                conditional_cycle(byte),
            )),
            0xDA => Some(Instruction::JP(
                JumpCond::Carry,
                cycles,
                conditional_cycle(byte),
            )),
            0xE9 => Some(Instruction::JPHL(cycles)),
            0xC4 => Some(Instruction::CALL(
                JumpCond::NotZero,
                cycles,
                conditional_cycle(byte),
            )),
            0xD4 => Some(Instruction::CALL(
                JumpCond::NotCarry,
                cycles,
                conditional_cycle(byte),
            )),
            0xCC => Some(Instruction::CALL(
                JumpCond::Zero,
                cycles,
                conditional_cycle(byte),
            )),
            0xDC => Some(Instruction::CALL(
                JumpCond::Carry,
                cycles,
                conditional_cycle(byte),
            )),
            0xCD => Some(Instruction::CALL(
                JumpCond::Always,
                cycles,
                conditional_cycle(byte),
            )),
            0xC7 => Some(Instruction::RST(RestartAddr::H00, cycles)),
            0xD7 => Some(Instruction::RST(RestartAddr::H10, cycles)),
            0xE7 => Some(Instruction::RST(RestartAddr::H20, cycles)),
            0xF7 => Some(Instruction::RST(RestartAddr::H30, cycles)),
            0xCF => Some(Instruction::RST(RestartAddr::H08, cycles)),
            0xDF => Some(Instruction::RST(RestartAddr::H18, cycles)),
            0xEF => Some(Instruction::RST(RestartAddr::H28, cycles)),
            0xFF => Some(Instruction::RST(RestartAddr::H38, cycles)),
            0xFB => Some(Instruction::EI(cycles)),
            0xF3 => Some(Instruction::DI(cycles)),
            0xEA => Some(Instruction::LDABY(cycles)),
            0xFA => Some(Instruction::LDA(cycles)),
            0xF8 => Some(Instruction::LDHLSP(cycles)),
            0xCB => panic!("This is a prefixed byte! This should never happen!"),
            0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xEB | 0xEC | 0xFC | 0xDD | 0xED | 0xFD => {
                info!("{} is an undefined function", byte);
                None
            }
            0xE0 => Some(Instruction::LDHA(cycles)),
            0xF0 => Some(Instruction::LDHA8(cycles)),
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

impl std::convert::From<u8> for JumpCond {
    fn from(byte: u8) -> JumpCond {
        match byte {
            0x18 => JumpCond::Always,
            0x20 => JumpCond::NotZero,
            0x28 => JumpCond::Zero,
            0x30 => JumpCond::NotCarry,
            0x38 => JumpCond::Carry,
            _ => panic!("u8 {:?} cannot be converted into an JumpCond", byte),
        }
    }
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

#[derive(Debug, PartialEq)]
pub enum RestartAddr {
    H00,
    H08,
    H10,
    H18,
    H20,
    H28,
    H30,
    H38,
}

impl std::convert::From<RestartAddr> for u16 {
    fn from(restart: RestartAddr) -> u16 {
        match restart {
            RestartAddr::H00 => 0x0000,
            RestartAddr::H08 => 0x0080,
            RestartAddr::H10 => 0x0010,
            RestartAddr::H18 => 0x0018,
            RestartAddr::H20 => 0x0020,
            RestartAddr::H28 => 0x0028,
            RestartAddr::H30 => 0x0030,
            RestartAddr::H38 => 0x0038,
        }
    }
}
