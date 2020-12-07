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
    assert_eq!(1, pc); // Should do nothing but inc the pc
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn add_test() {
    let mut cpu = setup();
    cpu.registers.a = 10;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADD(ArithmeticTarget::B));
    assert_eq!(11, cpu.register_value(&ArithmeticTarget::A));
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn add_overflow_test() {
    let mut cpu = setup();
    cpu.registers.a = 255;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADD(ArithmeticTarget::B));
    assert_eq!(0, cpu.register_value(&ArithmeticTarget::A));
    assert_flags_znhc(cpu.registers, true, false, true, true);
}

#[test]
fn sub_test() {
    let mut cpu = setup();
    cpu.registers.a = 255;
    cpu.registers.b = 1;
    cpu.execute(Instruction::SUB(ArithmeticTarget::B));
    assert_eq!(254, cpu.register_value(&ArithmeticTarget::A));
    assert_flags_znhc(cpu.registers, false, true, false, false);
}

#[test]
fn sub_underflow_test() {
    let mut cpu = setup();
    cpu.registers.b = 1;
    cpu.execute(Instruction::SUB(ArithmeticTarget::B));
    assert_eq!(255, cpu.register_value(&ArithmeticTarget::A));
    assert_flags_znhc(cpu.registers, false, true, true, true);
}

#[test]
fn adc_test() {
    let mut cpu = setup();
    cpu.registers.f.carry = true;
    cpu.registers.a = 1;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADC(ArithmeticTarget::B));
    assert_eq!(3, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn sbc_test() {
    // TODO
}

#[test]
fn and_test() {
    let mut cpu = setup();
    cpu.registers.a = 0xFF;
    cpu.registers.b = 10;
    cpu.execute(Instruction::AND(ArithmeticTarget::B));
    assert_eq!(10, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, true, false);
}

#[test]
fn or_test() {
    let mut cpu = setup();
    cpu.registers.a = 0xF0;
    cpu.registers.b = 0x0F;
    cpu.execute(Instruction::OR(ArithmeticTarget::B));
    assert_eq!(0xFF, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn xor_test() {
    let mut cpu = setup();
    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x0F;
    cpu.execute(Instruction::XOR(ArithmeticTarget::B));
    assert_eq!(0xF0, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn cp_test() {
    let mut cpu = setup();
    cpu.registers.b = 1;
    cpu.execute(Instruction::CP(ArithmeticTarget::B));
    assert_flags_znhc(cpu.registers, false, true, true, true);
}