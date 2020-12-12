use gb_emulator::{
    ArithmeticTarget, Instruction, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource,
    LoadWordTarget,
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

// TODO: ADD, ADC, SUB etc tests

#[test]
fn from_byte_load() {
    for i in 0x40..0x47 {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x48..0x4F {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x50..0x57 {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x58..0x5F {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x60..0x67 {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x68..0x6F {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::from(l_nib)
            ))
        );
    }

    for i in 0x70..0x77 {
        let l_nib = i & 0x0F;
        if i != 0x76 {
            let op = Instruction::from_byte(i, false).unwrap();
            assert_eq!(
                op,
                Instruction::LD(LoadType::Byte(
                    LoadByteTarget::HLI,
                    LoadByteSource::from(l_nib)
                ))
            );
        }
    }

    for i in 0x78..0x7F {
        let l_nib = i & 0x0F;
        let op = Instruction::from_byte(i, false).unwrap();
        assert_eq!(
            op,
            Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::from(l_nib)
            ))
        );
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
fn from_byte_non_prefix_nop() {
    let op = Instruction::from_byte(0x00, false).unwrap();
    assert_eq!(op, Instruction::NOP);
}

#[test]
fn from_byte_non_prefix_halt() {
    let op = Instruction::from_byte(0x76, false).unwrap();
    assert_eq!(op, Instruction::HALT);
}
