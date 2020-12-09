use gb_emulator::{ArithmeticTarget, Instruction};

#[test]
fn from_byte_prefix_rlc() {
    let op0 = Instruction::from_byte(0x00, true).unwrap();
    let op1 = Instruction::from_byte(0x01, true).unwrap();
    let op2 = Instruction::from_byte(0x02, true).unwrap();
    let op3 = Instruction::from_byte(0x03, true).unwrap();
    let op4 = Instruction::from_byte(0x04, true).unwrap();
    let op5 = Instruction::from_byte(0x05, true).unwrap();
    // TODO: The HL op when it's implemented
    // let op6 = Instruction::from_byte(0x06, true).unwrap();
    let op7 = Instruction::from_byte(0x07, true).unwrap();

    assert_eq!(op0, Instruction::RLC(ArithmeticTarget::B));
    assert_eq!(op1, Instruction::RLC(ArithmeticTarget::C));
    assert_eq!(op2, Instruction::RLC(ArithmeticTarget::D));
    assert_eq!(op3, Instruction::RLC(ArithmeticTarget::E));
    assert_eq!(op4, Instruction::RLC(ArithmeticTarget::H));
    assert_eq!(op5, Instruction::RLC(ArithmeticTarget::L));
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
    // TODO: The HL op when it's implemented
    // let op6 = Instruction::from_byte(0x16, true).unwrap();
    let op7 = Instruction::from_byte(0x17, true).unwrap();

    assert_eq!(op0, Instruction::RL(ArithmeticTarget::B));
    assert_eq!(op1, Instruction::RL(ArithmeticTarget::C));
    assert_eq!(op2, Instruction::RL(ArithmeticTarget::D));
    assert_eq!(op3, Instruction::RL(ArithmeticTarget::E));
    assert_eq!(op4, Instruction::RL(ArithmeticTarget::H));
    assert_eq!(op5, Instruction::RL(ArithmeticTarget::L));
    assert_eq!(op7, Instruction::RL(ArithmeticTarget::A));
}

#[test]
fn from_byte_non_prefix_nop() {
    let op = Instruction::from_byte(0x00, false).unwrap();
    assert_eq!(op, Instruction::NOP);
}
