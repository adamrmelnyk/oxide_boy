#![feature(destructuring_assignment)]

mod cpu;

use cpu::instructions::{JumpCond, SixteenBitArithmeticTarget};
use cpu::memory::{LoadByteSource, LoadByteTarget, LoadType, MemoryBus};
use cpu::registers::{FlagsRegister, Registers};

/// Exposed so they can be run from main
pub use cpu::instructions::{ArithmeticTarget, Instruction};

pub struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
    is_halted: bool,
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
                    subtract: false,
                    carry: false,
                    half_carry: false,
                },
                h: 0,
                l: 0,
            },
            bus: MemoryBus::default(),
            pc: 0,
            is_halted: true,
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
                Instruction::INC16(target) => {}
                Instruction::DEC16(target) => {}
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
                Instruction::RL => {}
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
                    let should_jump = match condition {
                        JumpCond::NotZero => !self.registers.f.zero,
                        JumpCond::Zero => self.registers.f.zero,
                        JumpCond::NotCarry => !self.registers.f.carry,
                        JumpCond::Carry => self.registers.f.carry,
                        JumpCond::Always => true,
                    };
                    self.jump(should_jump);
                }
                Instruction::LD(load_type) => match load_type {
                    LoadType::Byte(target, source) => {
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
                        // If we read from the D8, we should move the pc up one more spot
                        match source {
                            LoadByteSource::D8 => {
                                self.pc.wrapping_add(1);
                            }
                            _ => {}
                        }
                    }
                },
                Instruction::HALT => self.halt(),
                Instruction::NOP => { /* NO OP, simply advances the pc */ }
            }
        }
        self.pc.wrapping_add(1) // After each operation we increment the program counter
    }

    /// Helper method for returning the value of an 8bit register
    fn register_value(&self, target: &ArithmeticTarget) -> u8 {
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

    fn sixteen_bit_register_value(&self, target: &SixteenBitArithmeticTarget) -> u16 {
        match target {
            SixteenBitArithmeticTarget::AF => self.registers.get_af(),
            SixteenBitArithmeticTarget::BC => self.registers.get_bc(),
            SixteenBitArithmeticTarget::DE => self.registers.get_de(),
            SixteenBitArithmeticTarget::HL => self.registers.get_hl(),
            // SixteenBitArithmeticTarget::SP => self.registers.get_sp(),
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
            LoadByteSource::D8 => {
                1u8 /*self.read_next_byte() */
            } // TODO: implement
            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    // A = A + s
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // HL = HL + ss; BC,DE,HL,SP
    fn addhl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        // Zero register is unaffected
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xFF; // TODO: Double check
        new_value
    }

    // SP = SP + e
    fn addsp(&mut self, value: u16) -> u16 {
        unimplemented!();
    }

    // ss = ss + 1
    fn inc_16(&mut self, value: u16) -> u16 {
        unimplemented!();
    }

    // ss = ss - 1
    fn dec_16(&mut self, value: u16) -> u16 {
        unimplemented!();
    }

    // A = A + s + CY
    // * 0 * *
    fn adc(&mut self, value: u8) -> u8 {
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);
        if self.registers.f.carry {
            (new_value, did_overflow) = self.registers.a.overflowing_add(1u8);
        }
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // A = A - s
    fn sub(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a.wrapping_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = self.registers.a < value;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
        new_value
    }

    // A = A - s -CY
    // * 1 * *
    fn sbc(&mut self, value: u8) -> u8 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };
        let new_value = self.registers.a - value - carry;
        self.registers.f.zero = new_value == 0;
        // TODO: Set the carry bits
        // self.registers.f.subtract = true;
        // self.registers.f.half_carry = false;
        // self.registers.f.carry = false;
        new_value
    }

    // A = A & s
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A = A | s
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A = A ^ s
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // A - s
    fn cp(&mut self, value: u8) {
        let (_, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
    }

    // s = s + 1
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) + (1 & 0xF) > 0xF; // TODO: Double check this
                                                                       // Carry not affected
        new_value
    }

    // s = s - 1
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) < 1; // TODO; Double check this
                                                         // Carry not affected
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
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // Rotate left and carry
    // * 0 0 *
    fn rlc(&mut self, value: u8) -> u8 {
        self.registers.f.carry = (value & 0x80) == 0x80;
        let new_value = value.rotate_left(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        new_value
    }

    // * 0 0 *
    fn sra(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x01) == 1;
        new_value
    }

    // * 0 0 *
    fn sla(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
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

    fn halt(&mut self) {
        self.is_halted = true;
    }
}
