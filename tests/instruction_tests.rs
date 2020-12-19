use gb_emulator::{
    ArithmeticTarget, Instruction, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource,
    LoadWordTarget,
};

// TODO: Add more load tests

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

    assert_eq!(op0, Instruction::RLC(ArithmeticTarget::B));
    assert_eq!(op1, Instruction::RLC(ArithmeticTarget::C));
    assert_eq!(op2, Instruction::RLC(ArithmeticTarget::D));
    assert_eq!(op3, Instruction::RLC(ArithmeticTarget::E));
    assert_eq!(op4, Instruction::RLC(ArithmeticTarget::H));
    assert_eq!(op5, Instruction::RLC(ArithmeticTarget::L));
    assert_eq!(op6, Instruction::RLC(ArithmeticTarget::HLI));
    assert_eq!(op7, Instruction::RLC(ArithmeticTarget::A));
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

    assert_eq!(op0, Instruction::RL(ArithmeticTarget::B));
    assert_eq!(op1, Instruction::RL(ArithmeticTarget::C));
    assert_eq!(op2, Instruction::RL(ArithmeticTarget::D));
    assert_eq!(op3, Instruction::RL(ArithmeticTarget::E));
    assert_eq!(op4, Instruction::RL(ArithmeticTarget::H));
    assert_eq!(op5, Instruction::RL(ArithmeticTarget::L));
    assert_eq!(op6, Instruction::RL(ArithmeticTarget::HLI));
    assert_eq!(op7, Instruction::RL(ArithmeticTarget::A));
}

#[test]
fn from_byte_prefix_rr() {
    let op0 = Instruction::from_byte(0x1F, true).unwrap();
    assert_eq!(op0, Instruction::RR(ArithmeticTarget::A));
}

#[test]
fn from_byte_add() {
    for byte in 0x80..0x87 {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::ADD(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xC6, false).unwrap();
    assert_eq!(op, Instruction::ADD(ArithmeticTarget::D8));
}

#[test]
fn from_byte_adc() {
    for byte in 0x88..0x8F {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::ADC(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xCE, false).unwrap();
    assert_eq!(op, Instruction::ADC(ArithmeticTarget::D8));
}

#[test]
fn from_byte_sub() {
    for byte in 0x90..0x97 {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::SUB(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xD6, false).unwrap();
    assert_eq!(op, Instruction::SUB(ArithmeticTarget::D8));
}

#[test]
fn from_byte_sbc() {
    for byte in 0x98..0x9F {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::SBC(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xDE, false).unwrap();
    assert_eq!(op, Instruction::SBC(ArithmeticTarget::D8));
}

#[test]
fn from_byte_and() {
    for byte in 0xA0..0xA7 {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::AND(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xE6, false).unwrap();
    assert_eq!(op, Instruction::AND(ArithmeticTarget::D8));
}

#[test]
fn from_byte_xor() {
    for byte in 0xA8..0xAF {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::XOR(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xEE, false).unwrap();
    assert_eq!(op, Instruction::XOR(ArithmeticTarget::D8));
}

#[test]
fn from_byte_or() {
    for byte in 0xB0..0xB7 {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::OR(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xF6, false).unwrap();
    assert_eq!(op, Instruction::OR(ArithmeticTarget::D8));
}

#[test]
fn from_byte_cp() {
    for byte in 0xB8..0xBF {
        let op = Instruction::from_byte(byte, false).unwrap();
        assert_eq!(op, Instruction::CP(ArithmeticTarget::from(byte)));
    }
    let op = Instruction::from_byte(0xFE, false).unwrap();
    assert_eq!(op, Instruction::CP(ArithmeticTarget::D8));
}

#[test]
fn from_byte_load() {
    for i in 0x40..0x47 {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::from(i)))
        );
    }

    for i in 0x48..0x4F {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::from(i)))
        );
    }

    for i in 0x50..0x57 {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::from(i)))
        );
    }

    for i in 0x58..0x5F {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::from(i)))
        );
    }

    for i in 0x60..0x67 {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::from(i)))
        );
    }

    for i in 0x68..0x6F {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::from(i)))
        );
    }

    for i in 0x70..0x77 {
        if i != 0x76 {
            let op = Instruction::from_byte(i, false).unwrap();
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::from(i)))
            );
        }
    }

    for i in 0x78..0x7F {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::from(i),
                LoadByteSource::from(i)
            ))
        );
    }

    for i in vec![0x06, 0x16, 0x26, 0x36, 0x0E, 0x1E, 0x2E, 0x3E] {
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(LoadByteTarget::from(i), LoadByteSource::D8,))
        )
    }
}

#[test]
fn from_byte_ld_word() {
    let op = Instruction::from_byte(0x01, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))
    );

    let op = Instruction::from_byte(0x11, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))
    );

    let op = Instruction::from_byte(0x21, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))
    );

    let op = Instruction::from_byte(0x31, false).unwrap();
    assert_eq!(
        op,
        Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))
    );
}

#[test]
fn load_a_from_c_plus_0xff00() {
    let op = Instruction::from_byte(0xF2, false).unwrap();
    assert_eq!(op, Instruction::LDAC);
}

#[test]
fn load_c_plus_0xff00_from_a() {
    let op = Instruction::from_byte(0xE2, false).unwrap();
    assert_eq!(op, Instruction::LDCA);
}

#[test]
fn from_byte_non_prefix_nop() {
    let op = Instruction::from_byte(0x00, false).unwrap();
    assert_eq!(op, Instruction::NOP);
}

#[test]
fn from_byte_non_prefix_halt() {
    let op = Instruction::from_byte(0x76, false).unwrap();
    assert_eq!(op, Instruction::HALT);
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
