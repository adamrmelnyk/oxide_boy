#![feature(destructuring_assignment)]

mod cpu;

use cpu::instructions::{JumpCond, StackTarget};
use cpu::memory::{LoadByteSource, LoadByteTarget, LoadType, MemoryBus};
use cpu::registers::FlagsRegister;

pub use cpu::instructions::SixteenBitArithmeticTarget;
pub use cpu::instructions::{ArithmeticTarget, Instruction};
pub use cpu::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    sp: u16,
    bus: MemoryBus,
    pub is_halted: bool,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagsRegister {
                    zero: false,
                    negative: false,
                    carry: false,
                    half_carry: false,
                },
                h: 0,
                l: 0,
            },
            bus: MemoryBus::default(),
            pc: 0,
            sp: 0,
            is_halted: false,
        }
    }
}

impl CPU {
    fn set_register_by_target(&mut self, target: &ArithmeticTarget, value: u8) {
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if instruction_byte == 0xCB {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description)
        };

        self.pc = next_pc;
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        if !self.is_halted {
            match instruction {
                Instruction::ADD(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.add(value);
                }
                Instruction::ADDHL(target) => {
                    let value = self.sixteen_bit_register_value(&target);
                    let new_value = self.addhl(value);
                    self.registers.set_hl(new_value);
                }
                Instruction::ADDSP(target) => {}
                Instruction::INC16(target) => self.inc_16(target),
                Instruction::DEC16(target) => self.dec_16(target),
                Instruction::SUB(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.sub(value);
                }
                Instruction::ADC(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.adc(value);
                }
                Instruction::SBC(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.sbc(value);
                }
                Instruction::AND(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.and(value);
                }
                Instruction::OR(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.or(value);
                }
                Instruction::XOR(target) => {
                    let value = self.register_value(&target);
                    self.registers.a = self.xor(value);
                }
                Instruction::CP(target) => {
                    let value = self.register_value(&target);
                    self.cp(value);
                }
                Instruction::INC(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.inc(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::DEC(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.dec(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::CCF => self.ccf(),
                Instruction::SCF => self.scf(),
                Instruction::RRA => self.rra(),
                Instruction::RLA => self.rla(),
                Instruction::RRCA => self.rrca(),
                Instruction::RRLA => self.rlca(),
                Instruction::CPL => self.cpl(),
                Instruction::BIT => {}
                Instruction::RESET => {}
                Instruction::SET => {}
                Instruction::SRL => {}
                Instruction::RL(target) => {}
                Instruction::RRC(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.rrc(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::RLC(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.rlc(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::SRA(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.sra(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::SLA(target) => {
                    let value = self.register_value(&target);
                    let new_value = self.sla(value);
                    self.set_register_by_target(&target, new_value);
                }
                Instruction::SWAP(target) => {}
                Instruction::JP(condition) => {
                    self.jump(self.should_jump(condition));
                }
                Instruction::LD(load_type) => self.load(load_type),
                Instruction::HALT => self.halt(),
                Instruction::NOP => { /* NO OP, simply advances the pc */ }
                Instruction::PUSH(target) => self.push_from_target(target),
                Instruction::POP(target) => self.pop_and_store(target),
                Instruction::CALL(condition) => {
                    self.call(self.should_jump(condition));
                }
                Instruction::RET(condition) => {
                    self.ret(self.should_jump(condition));
                }
            }
        }
        self.pc.wrapping_add(1) // After each operation we increment the pc and return the value
    }

    /// Helper method for returning the value of an 8bit register
    pub fn register_value(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    pub fn sixteen_bit_register_value(&self, target: &SixteenBitArithmeticTarget) -> u16 {
        match target {
            SixteenBitArithmeticTarget::AF => self.registers.get_af(),
            SixteenBitArithmeticTarget::BC => self.registers.get_bc(),
            SixteenBitArithmeticTarget::DE => self.registers.get_de(),
            SixteenBitArithmeticTarget::HL => self.registers.get_hl(),
            SixteenBitArithmeticTarget::SP => self.sp,
        }
    }

    fn byte_from_lbs(&self, source: &LoadByteSource) -> u8 {
        match source {
            LoadByteSource::A => self.registers.a,
            LoadByteSource::B => self.registers.b,
            LoadByteSource::C => self.registers.c,
            LoadByteSource::D => self.registers.d,
            LoadByteSource::E => self.registers.e,
            LoadByteSource::H => self.registers.h,
            LoadByteSource::L => self.registers.l,
            LoadByteSource::D8 => self.read_next_byte(), // TODO: Double check this
            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    fn should_jump(&self, condition: JumpCond) -> bool {
        match condition {
            JumpCond::NotZero => !self.registers.f.zero,
            JumpCond::Zero => self.registers.f.zero,
            JumpCond::NotCarry => !self.registers.f.carry,
            JumpCond::Carry => self.registers.f.carry,
            JumpCond::Always => true,
        }
    }

    // A = A + s
    // * 0 * * *
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // HL = HL + ss; BC,DE,HL,SP
    // - 0 * *
    fn addhl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.set_flag_registers_nz(
            false,
            (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xFF,
            did_overflow);
        new_value
    }

    // SP = SP + e
    // 0 0 * *
    fn addsp(&mut self, value: u16) -> u16 {
        unimplemented!();
    }

    // ss = ss + 1
    // - - - -
    fn inc_16(&mut self, target: SixteenBitArithmeticTarget) {
        let value = self.sixteen_bit_register_value(&target).wrapping_add(1);
        self.set_16b_register_by_target(value, target);
    }

    // ss = ss - 1
    // - - - -
    fn dec_16(&mut self, target: SixteenBitArithmeticTarget) {
        let value = self.sixteen_bit_register_value(&target).wrapping_sub(1);
        self.set_16b_register_by_target(value, target);
    }

    // Helper function for 16 bit registers
    fn set_16b_register_by_target(&mut self, value: u16, target: SixteenBitArithmeticTarget) {
        match target {
            SixteenBitArithmeticTarget::AF => self.registers.set_af(value),
            SixteenBitArithmeticTarget::BC => self.registers.set_bc(value),
            SixteenBitArithmeticTarget::DE => self.registers.set_de(value),
            SixteenBitArithmeticTarget::HL => self.registers.set_hl(value),
            SixteenBitArithmeticTarget::SP => self.sp = value,
        }
    }

    // A = A + s + CY
    // * 0 * *
    fn adc(&mut self, value: u8) -> u8 {
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);
        let carry = if self.registers.f.carry {1} else {0};
        if self.registers.f.carry {
            (new_value, did_overflow) = new_value.overflowing_add(1u8);
        }
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + (carry & 0xF)> 0xF;
        new_value
    }

    // A = A - s
    fn sub(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a.wrapping_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = true;
        self.registers.f.carry = self.registers.a < value;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
        new_value
    }

    // A = A - s -CY
    // * 1 * *
    fn sbc(&mut self, value: u8) -> u8 {
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_sub(value);
        let carry = if self.registers.f.carry { 1 } else { 0 };
        if self.registers.f.carry {
            (new_value, did_overflow) = new_value.overflowing_sub(1u8);
        }
        self.registers.set_flag_registers(
            new_value == 0,
            true,
            (self.registers.a & 0xF) < (value & 0xF) + (carry & 0xF),
            did_overflow,
        );
        new_value
    }

    // A = A & s
    // * 0 1 0
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers
            .set_flag_registers(new_value == 0, false, true, false);
        new_value
    }

    // A = A | s
    // * 0 0 0
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A = A ^ s
    // * 0 0 0
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A - s
    // * 1 * *
    fn cp(&mut self, value: u8) {
        let (_, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.negative = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
    }

    // s = s + 1
    // * 0 * -
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.half_carry = (value & 0xF) + (1 & 0xF) > 0xF; // TODO: Double check this
        new_value
    }

    // s = s - 1
    // * 1 * -
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = true;
        self.registers.f.half_carry = (value & 0xF) < 1; // TODO; Double check this
        new_value
    }

    fn ccf(&mut self) {
        self.registers.f.carry = !self.registers.f.carry;
    }

    fn scf(&mut self) {
        self.registers.f.carry = true;
    }

    // 0 0 0 *
    fn rra(&mut self) {
        unimplemented!();
    }

    // 0 0 0 *
    fn rla(&mut self) {
        unimplemented!();
    }

    // 0 0 0 *
    fn rrca(&mut self) {
        unimplemented!();
    }

    fn rlca(&mut self) {
        unimplemented!();
    }

    fn cpl(&mut self) {
        unimplemented!();
    }

    fn bit() {}

    fn reset() {}

    fn set() {}

    // * 0 0 *
    fn srl() {}

    // * 0 0 *
    fn rr() {}

    // * 0 0 *
    fn rl() {}

    // Rotate right and carry
    // * 0 0 *
    fn rrc(&mut self, value: u8) -> u8 {
        self.registers.f.carry = (value & 0x1) == 1;
        let new_value = value.rotate_right(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // Rotate left and carry
    // * 0 0 *
    fn rlc(&mut self, value: u8) -> u8 {
        self.registers.f.carry = (value & 0x80) == 0x80;
        let new_value = value.rotate_left(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // * 0 0 *
    fn sra(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x01) == 1;
        new_value
    }

    // * 0 0 *
    fn sla(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.negative = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;
        new_value
    }

    fn swap() {}

    fn jump(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let least_sig = self.bus.read_byte(self.pc + 1) as u16;
            let most_sig = self.bus.read_byte(self.pc + 2) as u16;
            (most_sig << 8) | least_sig
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn load(&mut self, load_type: LoadType) {
        match load_type {
            LoadType::Byte(target, source) => {
                // TODO: This will need to be made into it's own method
                let source_value = self.byte_from_lbs(&source);
                match target {
                    LoadByteTarget::A => self.registers.a = source_value,
                    LoadByteTarget::B => self.registers.b = source_value,
                    LoadByteTarget::C => self.registers.c = source_value,
                    LoadByteTarget::D => self.registers.d = source_value,
                    LoadByteTarget::E => self.registers.e = source_value,
                    LoadByteTarget::H => self.registers.h = source_value,
                    LoadByteTarget::L => self.registers.l = source_value,
                    LoadByteTarget::HLI => {
                        self.bus.write_byte(self.registers.get_hl(), source_value)
                    }
                }
                // If we read from the D8, we should move the pc up one extra spot
                match source {
                    LoadByteSource::D8 => {
                        self.pc.wrapping_add(1);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Halt CPU until an interrupt occurs.
    /// - - - -
    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn push_from_target(&mut self, target: StackTarget) {
        let value = match target {
            StackTarget::AF => self.registers.get_af(),
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
        };
        self.sp = self.sp.wrapping_add(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_add(1);
        self.bus.write_byte(self.sp, ((value & 0xFF) >> 8) as u8);
    }

    // (SP-1) = ssh, (SP-2) = ssl, SP = SP-2
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop_and_store(&mut self, target: StackTarget) {
        let result = self.pop();
        match target {
            StackTarget::AF => self.registers.set_af(result),
            StackTarget::BC => self.registers.set_bc(result),
            StackTarget::DE => self.registers.set_de(result),
            StackTarget::HL => self.registers.set_hl(result),
        }
    }

    // ddl == (SP), ddh = (SP+1), SP = SP+2
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn ret(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    // TODO: Implement
    fn read_next_byte(&self) -> u8 {
        unimplemented!();
    }

    // TODO: Implement
    fn read_next_word(&self) -> u16 {
        unimplemented!();
    }
}
