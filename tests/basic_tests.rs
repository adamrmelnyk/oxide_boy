use gb_emulator::{ArithmeticTarget, Instruction, Registers, SixteenBitArithmeticTarget, CPU};

pub fn setup() -> CPU {
    CPU::default()
}

// Helper method for asserting flags in the order, zero, negative, half, carry
pub fn assert_flags_znhc(
    registers: Registers,
    zero: bool,
    negative: bool,
    half_carry: bool,
    carry: bool,
) {
    assert_eq!(registers.f.zero, zero);
    assert_eq!(registers.f.negative, negative);
    assert_eq!(registers.f.half_carry, half_carry);
    assert_eq!(registers.f.carry, carry);
}

#[test]
fn inc_16_test() {
    let mut cpu = setup();
    let before = cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::BC);
    cpu.execute(Instruction::INC16(SixteenBitArithmeticTarget::BC));
    let after = cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::BC);
    assert_eq!(before + 1, after);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn dec_16_test() {
    let mut cpu = setup();
    let before = cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::BC);
    cpu.execute(Instruction::DEC16(SixteenBitArithmeticTarget::BC));
    let after = cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::BC);
    assert_eq!(before.wrapping_sub(1), after);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn inc_test() {
    let mut cpu = setup();
    let before = cpu.register_value(&ArithmeticTarget::B);
    cpu.execute(Instruction::INC(ArithmeticTarget::B));
    let after = cpu.register_value(&ArithmeticTarget::B);
    assert_eq!(before + 1, after);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn inc_test_overflow() {
    let mut cpu = setup();
    cpu.registers.a = 255;
    cpu.execute(Instruction::INC(ArithmeticTarget::A));
    let after = cpu.register_value(&ArithmeticTarget::A);
    assert_eq!(0, after);
    assert_flags_znhc(cpu.registers, true, false, true, false);
}

#[test]
fn dec_test() {
    let mut cpu = setup();
    cpu.registers.b = 1;
    cpu.execute(Instruction::DEC(ArithmeticTarget::B));
    let after = cpu.register_value(&ArithmeticTarget::B);
    assert_eq!(0, after);
    assert_flags_znhc(cpu.registers, true, true, false, false)
}

#[test]
fn dec_test_underflow() {
    let mut cpu = setup();
    cpu.execute(Instruction::DEC(ArithmeticTarget::B));
    let after = cpu.register_value(&ArithmeticTarget::B);
    assert_eq!(255, after);
    assert_flags_znhc(cpu.registers, false, true, true, false);
}

#[test]
fn noop_test() {
    let mut cpu = setup();
    let pc = cpu.execute(Instruction::NOP);
    assert_eq!(1, pc);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}
