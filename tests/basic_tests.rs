use oxide_boy::{
    ArithmeticTarget, Instruction, JumpCond, LoadByteSource, LoadByteTarget, LoadType,
    LoadWordSource, LoadWordTarget, Registers, RestartAddr, SixteenBitArithmeticTarget, CPU,
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
    assert_eq!(registers.zero(), zero, "Zero flag does not match");
    assert_eq!(
        registers.negative(),
        negative,
        "Negative flag does not match"
    );
    assert_eq!(
        registers.half_carry(),
        half_carry,
        "Half Carry flag does not match"
    );
    assert_eq!(registers.carry(), carry, "Carry flag does not match");
}

#[test]
fn test_boot_rom() {
    let cpu = setup();
    assert_eq!(cpu.bus.read_byte(0), 0x31);
    assert_eq!(cpu.bus.read_byte(0xFF), 0x50);
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
fn inc_test_hli() {
    let mut cpu = setup();
    cpu.registers.set_hl(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x01);
    cpu.execute(Instruction::INC(ArithmeticTarget::HLI));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x02);
}

#[test]
fn dec_test_hli() {
    let mut cpu = setup();
    cpu.registers.set_hl(0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0x01);
    cpu.execute(Instruction::DEC(ArithmeticTarget::HLI));
    assert_eq!(cpu.bus.read_byte(0xA1A1), 0x00);
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

#[test]
fn addsp_test() {
    let mut cpu = setup();
    cpu.sp = 0x0001;
    cpu.bus.write_byte(0x1000, 0x01);
    cpu.pc = 0x1000;
    cpu.execute(Instruction::ADDSP);
    assert_eq!(0x0002, cpu.sp);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn addsp_overflow_test() {
    let mut cpu = setup();
    cpu.sp = 0xFFFF;
    cpu.bus.write_byte(0x1000, 0x01);
    cpu.pc = 0x1000;
    cpu.execute(Instruction::ADDSP);
    assert_eq!(0x0000, cpu.sp);
    assert_flags_znhc(cpu.registers, false, false, true, true);
}

#[test]
fn addsp_half_overflow_test() {
    let mut cpu = setup();
    cpu.sp = 0x00FF;
    cpu.bus.write_byte(0x1000, 0x01);
    cpu.pc = 0x1000;
    cpu.execute(Instruction::ADDSP);
    assert_eq!(0x0100, cpu.sp);
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
    cpu.registers.set_flags_nz(false, false, true);
    cpu.registers.a = 1;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADC(ArithmeticTarget::B));
    assert_eq!(3, cpu.registers.a);
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn adc_test_overflow() {
    let mut cpu = setup();
    cpu.registers.set_flags_nz(false, false, true);
    cpu.registers.a = 254;
    cpu.registers.b = 1;
    cpu.execute(Instruction::ADC(ArithmeticTarget::B));
    assert_eq!(0, cpu.registers.a);
    assert_flags_znhc(cpu.registers, true, false, true, true);
}

#[test]
fn adc_test_half_overflow() {
    let mut cpu = setup();
    cpu.registers.set_flags_nz(false, false, true);
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
    cpu.registers.set_flags_nz(false, false, true);
    cpu.execute(Instruction::SBC(ArithmeticTarget::B));
    assert_eq!(0, cpu.registers.a);
    assert_flags_znhc(cpu.registers, true, true, false, false);
}

#[test]
fn sbc_test_overflow() {
    let mut cpu = setup();
    cpu.registers.a = 2;
    cpu.registers.b = 2;
    cpu.registers.set_flags_nz(false, false, true);
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

#[test]
fn load_word_from_sp() {
    let mut cpu = setup();
    cpu.pc = 0x1000;
    cpu.bus.write_word(0x1000, 0xA1A1);
    cpu.sp = 0xAAAA;
    cpu.execute(Instruction::LD(LoadType::Word(
        LoadWordTarget::D16,
        LoadWordSource::SP,
    )));
    assert_eq!(cpu.bus.read_word(0xA1A1), 0xAAAA);
}

#[test]
fn load_hl_into_sp() {
    let mut cpu = setup();
    cpu.registers.set_hl(0xA1A1);
    cpu.execute(Instruction::LD(LoadType::Word(
        LoadWordTarget::SP,
        LoadWordSource::HL,
    )));
    assert_eq!(cpu.sp, 0xA1A1);
}

#[test]
fn load_byte_at_next_address_into_a_test() {
    let mut cpu = setup();
    cpu.pc = 0x1000;
    cpu.bus.write_word(0x1000, 0xA1A1);
    cpu.bus.write_byte(0xA1A1, 0xAA);
    cpu.execute(Instruction::LDA);
    assert_eq!(cpu.registers.a, 0xAA);
}

#[test]
fn cpl_test() {
    let mut cpu = setup();
    cpu.registers.a = 0x10;
    cpu.execute(Instruction::CPL);
    assert_eq!(cpu.registers.a, 0xEF);
}

#[test]
fn bit_test() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.execute(Instruction::BIT(7, ArithmeticTarget::A));
    assert_eq!(cpu.registers.zero(), false);
}

#[test]
fn res_test() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.execute(Instruction::RESET(7, ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0000_0000);
}

#[test]
fn set_test() {
    let mut cpu = setup();
    cpu.registers.a = 0b0000_0000;
    cpu.execute(Instruction::SET(0, ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0000_0001);
}

#[test]
fn swap_test() {
    let mut cpu = setup();
    cpu.registers.a = 0b1111_0000;
    cpu.execute(Instruction::SWAP(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0000_1111);
}

#[test]
fn test_rra() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.execute(Instruction::RRA);
    assert_eq!(cpu.registers.a, 0b0100_0000);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rra_carry() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.registers.set_flags_nz(false, false, true);
    cpu.execute(Instruction::RRA);
    assert_eq!(cpu.registers.a, 0b1100_0000);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rla() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.execute(Instruction::RLA);
    assert_eq!(cpu.registers.a, 0);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rla_carry() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.registers.set_flags_nz(false, false, true);
    cpu.execute(Instruction::RLA);
    assert_eq!(cpu.registers.a, 1);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rrca() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.execute(Instruction::RRCA);
    assert_eq!(cpu.registers.a, 0b1100_0000);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rlca() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.execute(Instruction::RLCA);
    assert_eq!(cpu.registers.a, 1);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_srl() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0000;
    cpu.execute(Instruction::SRL(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0100_0000);
    assert_eq!(0x80 & cpu.registers.a, 0); // MSB is zero
    assert_flags_znhc(cpu.registers, false, false, false, false);
}

#[test]
fn test_srl_overflow() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.execute(Instruction::SRL(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0100_0000);
    assert_eq!(0x80 & cpu.registers.a, 0); // MSB is zero
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rr() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.execute(Instruction::RR(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0100_0000);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rr_carry() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.registers.set_flags_nz(false, false, true);
    cpu.execute(Instruction::RR(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b1100_0000);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rl() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0001;
    cpu.execute(Instruction::RL(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0000_0010);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rl_carry() {
    let mut cpu = setup();
    cpu.registers.a = 0b1000_0011;
    cpu.registers.set_flags_nz(false, false, true);
    cpu.execute(Instruction::RL(ArithmeticTarget::A));
    assert_eq!(cpu.registers.a, 0b0000_0111);
    assert_flags_znhc(cpu.registers, false, false, false, true);
}

#[test]
fn test_rst() {
    let mut cpu = setup();
    cpu.pc = 0x1000;
    cpu.execute(Instruction::RST(RestartAddr::H28));
    assert_eq!(cpu.pc, 0x0028);
}

#[test]
fn test_jump_relative() {
    let mut cpu = setup();
    cpu.bus.write_byte(0x1000, 0b0000_0001);
    cpu.pc = 0x1000;
    cpu.execute(Instruction::JR(JumpCond::Always));
    assert_eq!(cpu.pc, 0x1001);
}

#[test]
fn test_jump_relative_negative() {
    let mut cpu = setup();
    cpu.bus.write_byte(0x1000, 0b1111_1111);
    cpu.pc = 0x1000;
    cpu.execute(Instruction::JR(JumpCond::Always));
    assert_eq!(cpu.pc, 0x0FFF);
}

#[test]
fn test_jump_to_hl() {
    let mut cpu = setup();
    cpu.registers.set_hl(0x1000);
    cpu.execute(Instruction::JPHL);
    assert_eq!(cpu.pc, 0x1000);
}

#[test]
fn test_ldha() {
    let mut cpu = setup();
    cpu.bus.write_byte(0xAAAA, 0x11);
    cpu.pc = 0xAAAA;
    cpu.registers.a = 0x12;
    cpu.execute(Instruction::LDHA);
    assert_eq!(cpu.bus.read_byte(0xFF11), cpu.registers.a);
}

#[test]
fn test_ld8a() {
    let mut cpu = setup();
    cpu.pc = 0x0011;
    cpu.bus.write_byte(0x0011, 0x11);
    cpu.bus.write_byte(0xFF11, 0x10);
    cpu.execute(Instruction::LDHA8);
    assert_eq!(cpu.registers.a, 0x10);
}

#[test]
fn test_disable_interrupts() {
    let mut cpu = setup();
    cpu.execute(Instruction::DI);
    assert_eq!(cpu.ime, false);
}

#[test]
fn test_enable_interrupts() {
    let mut cpu = setup();
    cpu.execute(Instruction::EI);
    assert_eq!(cpu.ime, true);
}

#[test]
fn test_reti() {
    let mut cpu = setup();
    cpu.pc = 0x3000;
    cpu.sp = 0xA000;
    cpu.bus.write_word(0xA000, 0x0101);
    cpu.execute(Instruction::RETI);
    assert_eq!(cpu.pc, 0x0101);
}

// TODO: Tests for pop, jump, ret, call etc
