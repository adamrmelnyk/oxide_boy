use gb_emulator::{
    ArithmeticTarget, Instruction, LoadByteSource, LoadByteTarget, LoadType, Registers,
    SixteenBitArithmeticTarget, CPU,
};

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
fn add_half_overflow_test() {
    let mut cpu = setup();
    cpu.registers.a = 15;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADD(ArithmeticTarget::B));
    assert_eq!(16, cpu.register_value(&ArithmeticTarget::A));
    assert_flags_znhc(cpu.registers, false, false, true, false);
}

#[test]
fn addhl_test() {
    let mut cpu = setup();
    cpu.registers.set_hl(1);
    cpu.registers.set_bc(10);
    cpu.execute(Instruction::ADDHL(SixteenBitArithmeticTarget::BC));
    assert_eq!(
        11,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::HL)
    );
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn addhl_overflow_test() {
    let mut cpu = setup();
    cpu.registers.set_hl(1);
    cpu.registers.set_bc(65535);
    cpu.execute(Instruction::ADDHL(SixteenBitArithmeticTarget::BC));
    assert_eq!(
        0,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::HL)
    );
    assert_flags_znhc(cpu.registers, false, false, true, true);
}

#[test]
fn addhl_half_overflow_test() {
    let mut cpu = setup();
    cpu.registers.set_hl(1);
    cpu.registers.set_bc(255);
    cpu.execute(Instruction::ADDHL(SixteenBitArithmeticTarget::BC));
    assert_eq!(
        256,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::HL)
    );
    assert_flags_znhc(cpu.registers, false, false, true, false);
}

// TODO: Fix this test
// #[test]
fn addsp_test() {
    let mut cpu = setup();
    cpu.registers.set_bc(1);
    cpu.sp = 10;
    cpu.execute(Instruction::ADDSP);
    assert_eq!(
        11,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::SP)
    );
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

// #[test]
fn addsp_overflow_test() {
    let mut cpu = setup();
    cpu.registers.set_bc(65535);
    cpu.sp = 1;
    cpu.execute(Instruction::ADDSP);
    assert_eq!(
        0,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::SP)
    );
    assert_flags_znhc(cpu.registers, false, false, true, true);
}

// #[test]
fn addsp_half_overflow_test() {
    let mut cpu = setup();
    cpu.sp = 1;
    cpu.registers.set_bc(255);
    cpu.execute(Instruction::ADDSP);
    assert_eq!(
        256,
        cpu.sixteen_bit_register_value(&SixteenBitArithmeticTarget::SP)
    );
    assert_flags_znhc(cpu.registers, false, false, true, false);
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
fn adc_test_overflow() {
    let mut cpu = setup();
    cpu.registers.f.carry = true;
    cpu.registers.a = 254;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADC(ArithmeticTarget::B));
    assert_eq!(0, cpu.registers.a);
    assert_flags_znhc(cpu.registers, true, false, true, true);
}

#[test]
fn adc_test_half_overflow() {
    let mut cpu = setup();
    cpu.registers.f.carry = true;
    cpu.registers.a = 15;
    cpu.execute(Instruction::ADC(ArithmeticTarget::B));
    assert_eq!(16, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, true, false);
}

#[test]
fn sbc_test() {
    let mut cpu = setup();
    cpu.registers.a = 3;
    cpu.registers.b = 2;
    cpu.registers.f.carry = true;
    cpu.execute(Instruction::SBC(ArithmeticTarget::B));
    assert_eq!(0, cpu.registers.a);
    assert_flags_znhc(cpu.registers, true, true, false, false);
}

#[test]
fn sbc_test_overflow() {
    let mut cpu = setup();
    cpu.registers.a = 2;
    cpu.registers.b = 2;
    cpu.registers.f.carry = true;
    cpu.execute(Instruction::SBC(ArithmeticTarget::B));
    assert_eq!(255, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, true, true, true);
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

#[test]
fn halt_test() {
    let mut cpu = setup();
    cpu.execute(Instruction::HALT);
    assert_eq!(true, cpu.is_halted);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn ccf_test() {
    let mut cpu = setup();
    cpu.execute(Instruction::CCF);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn scf_test() {
    let mut cpu = setup();
    cpu.execute(Instruction::SCF);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn load_test_bci() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.registers.set_bc(0xA1A1);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::BCI,
        LoadByteSource::A,
    )));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x10);
}

#[test]
fn load_test_dei() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.registers.set_de(0xA1A1);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::DEI,
        LoadByteSource::A,
    )));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x10);
}

#[test]
fn load_test_hlinc() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.registers.set_hl(0xA1A1);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::HLINC,
        LoadByteSource::A,
    )));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x10);
    assert_eq!(cpu.registers.get_hl() - 1, 0xA1A1);
}

#[test]
fn load_test_hldec() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.registers.set_hl(0xA1A1);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::HLDEC,
        LoadByteSource::A,
    )));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x10);
    assert_eq!(cpu.registers.get_hl() + 1, 0xA1A1);
}

#[test]
fn load_tests() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::B,
        LoadByteSource::A,
    )));
    assert_eq!(cpu.registers.b, 0x10);
}

#[test]
fn load_test_a_from_bci() {
    let mut cpu = setup();
    cpu.registers.set_bc(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x10);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::A,
        LoadByteSource::BCI,
    )));
    assert_eq!(cpu.registers.a, 0x10);
}

#[test]
fn load_test_a_from_dei() {
    let mut cpu = setup();
    cpu.registers.set_de(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x10);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::A,
        LoadByteSource::DEI,
    )));
    assert_eq!(cpu.registers.a, 0x10);
}

#[test]
fn load_test_a_from_hlinc() {
    let mut cpu = setup();
    cpu.registers.set_hl(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x10);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::A,
        LoadByteSource::HLINC,
    )));
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.get_hl(), 0xA1A2);
}

#[test]
fn load_test_a_from_hldec() {
    let mut cpu = setup();
    cpu.registers.set_hl(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x10);
    cpu.execute(Instruction::LD(LoadType::Byte(
        LoadByteTarget::A,
        LoadByteSource::HLDEC,
    )));
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.get_hl(), 0xA1A0);
}

#[test]
fn load_a_from_c_plus_0xff00() {
    let mut cpu = setup();
    cpu.bus.write_byte(0xff11, 0x10);
    cpu.registers.c = 0x11;
    cpu.execute(Instruction::LDAC);
    assert_eq!(cpu.registers.a, 0x10);
}

#[test]
fn load_c_plus_0xff00_from_a() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.registers.c = 0x11;
    cpu.execute(Instruction::LDCA);
    assert_eq!(cpu.bus.read_byte(0xFF11), 0x10);
}