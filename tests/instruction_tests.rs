use oxide_boy::{
    ArithmeticTarget, StackTarget, Instruction, JumpCond, LoadByteSource, LoadByteTarget, LoadType,
    LoadWordSource, LoadWordTarget,
};

#[test]
fn from_byte_prefix_rlc() {
    let op0 = Instruction::from_byte(0x00, true).unwrap();
    let op1 = Instruction::from_byte(0x01, true).unwrap();
    let op2 = Instruction::from_byte(0x02, true).unwrap();
    let op3 = Instruction::from_byte(0x03, true).unwrap();
    let op4 = Instruction::from_byte(0x04, true).unwrap();
    let op5 = Instruction::from_byte(0x05, true).unwrap();
    let op6 = Instruction::from_byte(0x06, true).unwrap();
    let op7 = Instruction::from_byte(0x07, true).unwrap();

    assert_eq!(op0, Instruction::RLC(ArithmeticTarget::B, 8));
    assert_eq!(op1, Instruction::RLC(ArithmeticTarget::C, 8));
    assert_eq!(op2, Instruction::RLC(ArithmeticTarget::D, 8));
    assert_eq!(op3, Instruction::RLC(ArithmeticTarget::E, 8));
    assert_eq!(op4, Instruction::RLC(ArithmeticTarget::H, 8));
    assert_eq!(op5, Instruction::RLC(ArithmeticTarget::L, 8));
    assert_eq!(op6, Instruction::RLC(ArithmeticTarget::HLI, 16));
    assert_eq!(op7, Instruction::RLC(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_prefix_rrc() {
    let op0 = Instruction::from_byte(0x08, true).unwrap();
    let op1 = Instruction::from_byte(0x09, true).unwrap();
    let op2 = Instruction::from_byte(0x0A, true).unwrap();
    let op3 = Instruction::from_byte(0x0B, true).unwrap();
    let op4 = Instruction::from_byte(0x0C, true).unwrap();
    let op5 = Instruction::from_byte(0x0D, true).unwrap();
    let op6 = Instruction::from_byte(0x0E, true).unwrap();
    let op7 = Instruction::from_byte(0x0F, true).unwrap();

    assert_eq!(op0, Instruction::RRC(ArithmeticTarget::B, 8));
    assert_eq!(op1, Instruction::RRC(ArithmeticTarget::C, 8));
    assert_eq!(op2, Instruction::RRC(ArithmeticTarget::D, 8));
    assert_eq!(op3, Instruction::RRC(ArithmeticTarget::E, 8));
    assert_eq!(op4, Instruction::RRC(ArithmeticTarget::H, 8));
    assert_eq!(op5, Instruction::RRC(ArithmeticTarget::L, 8));
    assert_eq!(op6, Instruction::RRC(ArithmeticTarget::HLI, 16));
    assert_eq!(op7, Instruction::RRC(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_prefix_rl() {
    let op0 = Instruction::from_byte(0x10, true).unwrap();
    let op1 = Instruction::from_byte(0x11, true).unwrap();
    let op2 = Instruction::from_byte(0x12, true).unwrap();
    let op3 = Instruction::from_byte(0x13, true).unwrap();
    let op4 = Instruction::from_byte(0x14, true).unwrap();
    let op5 = Instruction::from_byte(0x15, true).unwrap();
    let op6 = Instruction::from_byte(0x16, true).unwrap();
    let op7 = Instruction::from_byte(0x17, true).unwrap();

    assert_eq!(op0, Instruction::RL(ArithmeticTarget::B, 8));
    assert_eq!(op1, Instruction::RL(ArithmeticTarget::C, 8));
    assert_eq!(op2, Instruction::RL(ArithmeticTarget::D, 8));
    assert_eq!(op3, Instruction::RL(ArithmeticTarget::E, 8));
    assert_eq!(op4, Instruction::RL(ArithmeticTarget::H, 8));
    assert_eq!(op5, Instruction::RL(ArithmeticTarget::L, 8));
    assert_eq!(op6, Instruction::RL(ArithmeticTarget::HLI, 16));
    assert_eq!(op7, Instruction::RL(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_prefix_rr() {
    let op0 = Instruction::from_byte(0x1F, true).unwrap();
    assert_eq!(op0, Instruction::RR(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_prefix_rr_hli() {
    let op0 = Instruction::from_byte(0x1E, true).unwrap();
    assert_eq!(op0, Instruction::RR(ArithmeticTarget::HLI, 16));
}

#[test]
fn from_byte_prefix_sra() {
    let op = Instruction::from_byte(0x28, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::B, 8));
    let op = Instruction::from_byte(0x29, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::C, 8));
    let op = Instruction::from_byte(0x2A, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::D, 8));
    let op = Instruction::from_byte(0x2B, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::E, 8));
    let op = Instruction::from_byte(0x2C, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::H, 8));
    let op = Instruction::from_byte(0x2D, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::L, 8));
    let op = Instruction::from_byte(0x2E, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::HLI, 16));
    let op = Instruction::from_byte(0x2F, true).unwrap();
    assert_eq!(op, Instruction::SRA(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_prefix_sla() {
    let op = Instruction::from_byte(0x20, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::B, 8));
    let op = Instruction::from_byte(0x21, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::C, 8));
    let op = Instruction::from_byte(0x22, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::D, 8));
    let op = Instruction::from_byte(0x23, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::E, 8));
    let op = Instruction::from_byte(0x24, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::H, 8));
    let op = Instruction::from_byte(0x25, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::L, 8));
    let op = Instruction::from_byte(0x26, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::HLI, 16));
    let op = Instruction::from_byte(0x27, true).unwrap();
    assert_eq!(op, Instruction::SLA(ArithmeticTarget::A, 8));
}

#[test]
fn from_byte_add() {
    for byte in 0x80..=0x87 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x86 {
            assert_eq!(op, Instruction::ADD(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::ADD(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xC6, false).unwrap();
    assert_eq!(op, Instruction::ADD(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_adc() {
    for byte in 0x88..=0x8F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x8E {
            assert_eq!(op, Instruction::ADC(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::ADC(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xCE, false).unwrap();
    assert_eq!(op, Instruction::ADC(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_sub() {
    for byte in 0x90..=0x97 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x96 {
            assert_eq!(op, Instruction::SUB(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::SUB(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xD6, false).unwrap();
    assert_eq!(op, Instruction::SUB(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_sbc() {
    for byte in 0x98..=0x9F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x9E {
            assert_eq!(op, Instruction::SBC(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::SBC(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xDE, false).unwrap();
    assert_eq!(op, Instruction::SBC(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_and() {
    for byte in 0xA0..=0xA7 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0xA6 {
            assert_eq!(op, Instruction::AND(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::AND(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xE6, false).unwrap();
    assert_eq!(op, Instruction::AND(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_xor() {
    for byte in 0xA8..=0xAF {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0xAE {
            assert_eq!(op, Instruction::XOR(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::XOR(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xEE, false).unwrap();
    assert_eq!(op, Instruction::XOR(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_or() {
    for byte in 0xB0..=0xB7 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0xB6 {
            assert_eq!(op, Instruction::OR(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::OR(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xF6, false).unwrap();
    assert_eq!(op, Instruction::OR(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_cp() {
    for byte in 0xB8..=0xBF {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0xBE {
            assert_eq!(op, Instruction::CP(ArithmeticTarget::HLI, 8));
        } else {
            assert_eq!(op, Instruction::CP(ArithmeticTarget::from(byte), 4));
        }
    }
    let op = Instruction::from_byte(0xFE, false).unwrap();
    assert_eq!(op, Instruction::CP(ArithmeticTarget::D8, 8));
}

#[test]
fn from_byte_load() {
    for byte in 0x40..=0x47 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x46 {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::B, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x48..=0x4F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x4E {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::C, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x50..=0x57 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x56 {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::D, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x58..=0x5F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x5E {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::E, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x60..=0x67 {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x66 {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::H, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x68..=0x6F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x6E {
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI), 8)
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::L, LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in 0x70..=0x77 {
        if byte != 0x76 {
            let op = Instruction::from_byte(byte, false).unwrap();
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::from(byte)),
                    8
                )
            );
        }
    }

    for byte in 0x78..=0x7F {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x7E {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(byte)),
                    8
                )
            );
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::from(byte)),
                    4
                )
            );
        }
    }

    for byte in vec![0x06, 0x16, 0x26, 0x36, 0x0E, 0x1E, 0x2E, 0x3E] {
        let op = Instruction::from_byte(byte, false).unwrap();
        if byte == 0x36 {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::D8),
                    12
                )
            )
        } else {
            assert_eq!(
                op,
                Instruction::LD(
                    LoadType::Byte(LoadByteTarget::from(byte), LoadByteSource::D8),
                    8
                )
            )
        }
    }
}

#[test]
fn from_byte_ld_word() {
    let op = Instruction::from_byte(0x01, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16), 12)
    );

    let op = Instruction::from_byte(0x11, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16), 12)
    );

    let op = Instruction::from_byte(0x21, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16), 12)
    );

    let op = Instruction::from_byte(0x31, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16), 12)
    );
}

#[test]
fn load_a_from_c_plus_0xff00() {
    let op = Instruction::from_byte(0xF2, false).unwrap();
    assert_eq!(op, Instruction::LDAC(8));
}

#[test]
fn load_c_plus_0xff00_from_a() {
    let op = Instruction::from_byte(0xE2, false).unwrap();
    assert_eq!(op, Instruction::LDCA(8));
}

#[test]
fn from_byte_non_prefix_nop() {
    let op = Instruction::from_byte(0x00, false).unwrap();
    assert_eq!(op, Instruction::NOP(4));
}

#[test]
fn from_byte_non_prefix_halt() {
    let op = Instruction::from_byte(0x76, false).unwrap();
    assert_eq!(op, Instruction::HALT(4));
}

#[test]
fn load_word_from_sp() {
    let op = Instruction::from_byte(0x08, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::D16, LoadWordSource::SP), 20)
    );
}

#[test]
fn load_bci() {
    let op = Instruction::from_byte(0x02, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Byte(LoadByteTarget::BCI, LoadByteSource::A), 8)
    );
}

#[test]
fn load_dei() {
    let op = Instruction::from_byte(0x12, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Byte(LoadByteTarget::DEI, LoadByteSource::A), 8)
    );
}

#[test]
fn load_hlinc() {
    let op = Instruction::from_byte(0x22, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Byte(LoadByteTarget::HLINC, LoadByteSource::A), 8)
    );
}

#[test]
fn load_hlec() {
    let op = Instruction::from_byte(0x32, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Byte(LoadByteTarget::HLDEC, LoadByteSource::A), 8)
    );
}

#[test]
fn load_hl_from_sp() {
    let op = Instruction::from_byte(0xF9, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL), 8)
    );
}

#[test]
fn cpl() {
    let op = Instruction::from_byte(0x2F, false).unwrap();
    assert_eq!(op, Instruction::CPL(4));
}

#[test]
fn jump_relative_test() {
    let op = Instruction::from_byte(0x18, false).unwrap();
    assert_eq!(op, Instruction::JR(JumpCond::Always, 12, 12));
}

#[test]
fn jump_relative_test_nz() {
    let op = Instruction::from_byte(0x20, false).unwrap();
    assert_eq!(op, Instruction::JR(JumpCond::NotZero, 8, 12));
}

#[test]
fn jump_relative_test_z() {
    let op = Instruction::from_byte(0x28, false).unwrap();
    assert_eq!(op, Instruction::JR(JumpCond::Zero, 8, 12));
}

#[test]
fn jump_relative_test_nc() {
    let op = Instruction::from_byte(0x30, false).unwrap();
    assert_eq!(op, Instruction::JR(JumpCond::NotCarry, 8, 12));
}

#[test]
fn jump_relative_test_c() {
    let op = Instruction::from_byte(0x38, false).unwrap();
    assert_eq!(op, Instruction::JR(JumpCond::Carry, 8, 12));
}

#[test]
fn test_jphl() {
    let op = Instruction::from_byte(0xE9, false).unwrap();
    assert_eq!(op, Instruction::JPHL(4));
}

#[test]
fn test_ldha() {
    let op = Instruction::from_byte(0xE0, false).unwrap();
    assert_eq!(op, Instruction::LDHA(12));
}

#[test]
fn test_ldha8() {
    let op = Instruction::from_byte(0xF0, false).unwrap();
    assert_eq!(op, Instruction::LDHA8(12));
}

#[test]
fn test_ei() {
    let op = Instruction::from_byte(0xFB, false).unwrap();
    assert_eq!(op, Instruction::EI(4));
}

#[test]
fn test_di() {
    let op = Instruction::from_byte(0xF3, false).unwrap();
    assert_eq!(op, Instruction::DI(4));
}

#[test]
fn bit_bl() {
    let op = Instruction::from_byte(0x46, true).unwrap();
    assert_eq!(op, Instruction::BIT(0, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x56, true).unwrap();
    assert_eq!(op, Instruction::BIT(2, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x66, true).unwrap();
    assert_eq!(op, Instruction::BIT(4, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x76, true).unwrap();
    assert_eq!(op, Instruction::BIT(6, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x4E, true).unwrap();
    assert_eq!(op, Instruction::BIT(1, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x5E, true).unwrap();
    assert_eq!(op, Instruction::BIT(3, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x6E, true).unwrap();
    assert_eq!(op, Instruction::BIT(5, ArithmeticTarget::HLI, 12));
    let op = Instruction::from_byte(0x7E, true).unwrap();
    assert_eq!(op, Instruction::BIT(7, ArithmeticTarget::HLI, 12));
}

#[test]
fn reset() {
    let op = Instruction::from_byte(0x80, true).unwrap();
    assert_eq!(op, Instruction::RESET(0, ArithmeticTarget::B, 8));
}

#[test]
fn set() {
    let op = Instruction::from_byte(0xC0, true).unwrap();
    assert_eq!(op, Instruction::SET(0, ArithmeticTarget::B, 8));
}

#[test]
fn ret() {
    let op = Instruction::from_byte(0xC0, false).unwrap();
    assert_eq!(op, Instruction::RET(JumpCond::NotZero, 8, 20));
    let op = Instruction::from_byte(0xD0, false).unwrap();
    assert_eq!(op, Instruction::RET(JumpCond::NotCarry, 8, 20));
    let op = Instruction::from_byte(0xC8, false).unwrap();
    assert_eq!(op, Instruction::RET(JumpCond::Zero, 8, 20));
    let op = Instruction::from_byte(0xD8, false).unwrap();
    assert_eq!(op, Instruction::RET(JumpCond::Carry, 8, 20));
    let op = Instruction::from_byte(0xC9, false).unwrap();
    assert_eq!(op, Instruction::RET(JumpCond::Always, 16, 16));
}

#[test]
fn pop() {
    let op = Instruction::from_byte(0xC1, false).unwrap();
    assert_eq!(op, Instruction::POP(StackTarget::BC, 12));
    let op = Instruction::from_byte(0xD1, false).unwrap();
    assert_eq!(op, Instruction::POP(StackTarget::DE, 12));
    let op = Instruction::from_byte(0xE1, false).unwrap();
    assert_eq!(op, Instruction::POP(StackTarget::HL, 12));
    let op = Instruction::from_byte(0xF1, false).unwrap();
    assert_eq!(op, Instruction::POP(StackTarget::AF, 12));
}

#[test]
fn push() {
    let op = Instruction::from_byte(0xC5, false).unwrap();
    assert_eq!(op, Instruction::PUSH(StackTarget::BC, 16));
    let op = Instruction::from_byte(0xD5, false).unwrap();
    assert_eq!(op, Instruction::PUSH(StackTarget::DE, 16));
    let op = Instruction::from_byte(0xE5, false).unwrap();
    assert_eq!(op, Instruction::PUSH(StackTarget::HL, 16));
    let op = Instruction::from_byte(0xF5, false).unwrap();
    assert_eq!(op, Instruction::PUSH(StackTarget::AF, 16));
}

#[test]
fn undefined_function_tests() {
    for i in vec![
        0xD3, 0xE3, 0xE4, 0xF4, 0xDB, 0xEB, 0xEC, 0xFC, 0xDD, 0xED, 0xFD,
    ] {
        let op = Instruction::from_byte(i, false);
        assert_eq!(op, None);
    }
}
