use crate::dmg::bus::Bus;
use crate::dmg::instructions::{
    ArithmeticTarget, Instruction, JumpCond, RestartAddr, SixteenBitArithmeticTarget, StackTarget,
};
use crate::dmg::memory::{
    Interrupt, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget,
};
use crate::dmg::registers::{Registers,FlagsRegister};

// Interrupt starting addresses
const V_BLANK_ISR: u16 = 0x40;
const LCD_ISR: u16 = 0x48;
const TIMER_ISR: u16 = 0x50;
const SERIAL_ISR: u16 = 0x58;
const JOYPAD_ISR: u16 = 0x60;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: Bus,
    pub is_halted: bool,
    pub ime: bool, // Interrupt Master Enable
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
                f: FlagsRegister::default(),
                h: 0,
                l: 0,
            },
            bus: Bus::default(),
            pc: 0,
            sp: 0xFFFE,
            is_halted: false,
            ime: false,
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
            ArithmeticTarget::HLI => self.bus.write_byte(self.registers.get_hl(), value),
            _ => panic!("target: {:?}, not allowed", target),
        }
        if self.registers.b == 1 && &ArithmeticTarget::B == target {
            println!("Setting b to 1");
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        println!("Instruction {:#02x}", instruction_byte);
        if instruction_byte == 0xCB {
            instruction_byte = self.bus.read_byte(self.pc + 1);
            println!("Prefix: {:#02x}", instruction_byte);
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
        let prefix = if prefixed {1} else {0};
        self.pc = next_pc + prefix;
        self.bus.step(); // TODO: Not sure if this is the correct spot
        self.handle_interrupts();
    }

    fn handle_interrupts(&mut self) {
        if self.ime {
            match self.bus.return_interrupt() {
                Interrupt::VBlank => self.execute_interrupt(V_BLANK_ISR),
                Interrupt::LcdStat => self.execute_interrupt(LCD_ISR),
                Interrupt::TimerOverflow => self.execute_interrupt(TIMER_ISR),
                Interrupt::SerialLink => self.execute_interrupt(SERIAL_ISR),
                Interrupt::JoypadPress => self.execute_interrupt(JOYPAD_ISR),
                Interrupt::NONE => { /* Do nothing */ }
            }
        }
    }

    fn execute_interrupt(&mut self, isr: u16) {
        self.push(self.pc);
        self.pc = isr;
        self.bus.interrupt_flag_off();
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        let mut dont_inc_pc = false;
        if !self.is_halted {
            match instruction {
                Instruction::ADD(target) => self.add(target),
                Instruction::ADDHL(target) => self.addhl(target),
                Instruction::ADDSP => {
                    let value = self.read_next_byte();
                    let new_value = self.addsp(value);
                    self.sp = new_value;
                }
                Instruction::INC16(target) => self.inc_16(target),
                Instruction::DEC16(target) => self.dec_16(target),
                Instruction::SUB(target) => self.sub(target),
                Instruction::ADC(target) => self.adc(target),
                Instruction::SBC(target) => self.sbc(target),
                Instruction::AND(target) => self.and(target),
                Instruction::OR(target) => self.or(target),
                Instruction::XOR(target) => self.xor(target),
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
                Instruction::RLCA => self.rlca(),
                Instruction::DAA => self.daa(),
                Instruction::CPL => self.cpl(),
                Instruction::BIT(index, target) => self.bit(index, target),
                Instruction::RESET(index, target) => self.reset(index, target),
                Instruction::SET(index, target) => self.set(index, target),
                Instruction::SRL(target) => self.srl(target),
                Instruction::RL(target) => self.rl(target),
                Instruction::RR(target) => self.rr(target),
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
                Instruction::SWAP(target) => self.swap(target),
                Instruction::JP(condition) => {
                    dont_inc_pc = self.jump(self.should_jump(condition))
                },
                Instruction::JPHL => self.jump_to_address_hl(),
                Instruction::JR(condition) => {
                    dont_inc_pc = self.jump_relative(self.should_jump(condition));
                }
                Instruction::LD(load_type) => self.load(load_type),
                Instruction::LDAC => self.ldac(),
                Instruction::LDCA => self.ldca(),
                Instruction::HALT => self.halt(),
                Instruction::NOP => { /* NO OP, simply advances the pc */ }
                Instruction::STOP => self.stop(),
                Instruction::PUSH(target) => self.push_from_target(target),
                Instruction::POP(target) => self.pop_and_store(target),
                Instruction::CALL(condition) => dont_inc_pc = self.call(self.should_jump(condition)),
                Instruction::RET(condition) => {
                    dont_inc_pc = self.ret(self.should_jump(condition));
                }
                Instruction::RETI => {
                    self.reti();
                }
                Instruction::RST(addr) => self.rst(addr),
                Instruction::EI => self.enable_interupts(),
                Instruction::DI => self.disable_interupts(),
                Instruction::LDHA => self.ldha(),
                Instruction::LDHA8 => self.ldha8(),
                Instruction::LDABY => self.load_a_into_next_byte(),
                Instruction::LDA => self.load_byte_at_next_address_into_a(),
                Instruction::LDHLSP => self.ldhlsp(),
            }
        }
        if dont_inc_pc { self.pc } else { self.pc.wrapping_add(1) } // After each operation we increment the pc and return the value
    }

    /// Helper method for returning the value of an 8bit register
    pub fn register_value(&mut self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
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

    // Move this into an impl for Jumpcond?
    fn should_jump(&self, condition: JumpCond) -> bool {
        match condition {
            JumpCond::NotZero => !self.registers.zero(),
            JumpCond::Zero => self.registers.zero(),
            JumpCond::NotCarry => !self.registers.carry(),
            JumpCond::Carry => self.registers.carry(),
            JumpCond::Always => true,
        }
    }

    // A = A + s
    // * 0 * * *
    fn add(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers
            .set_flags(new_value == 0, false, half_carry, did_overflow);
        self.registers.a = new_value;
    }

    // HL = HL + ss; BC,DE,HL,SP
    // - 0 * *
    fn addhl(&mut self, target: SixteenBitArithmeticTarget) {
        let value = self.sixteen_bit_register_value(&target);
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        let half_carry = (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xFF;
        self.registers.set_flags_nz(false, half_carry, did_overflow);
        self.registers.set_hl(new_value);
    }

    // SP = SP + e
    // 0 0 * *
    fn addsp(&mut self, value: u8) -> u16 {
        let signed_val = i16::from(value as i8) as u16;
        let half_carry = (self.sp & 0xFF) + (signed_val & 0xFF) > 0xFF;
        let (new_value, did_overflow) = self.sp.overflowing_add(signed_val);
        self.registers
            .set_flags(false, false, half_carry, did_overflow);
        new_value
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
    fn adc(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_add(value);
        let carry = if self.registers.carry() { 1 } else { 0 };
        if self.registers.carry() {
            (new_value, did_overflow) = new_value.overflowing_add(1u8);
        }
        let half_carry = (self.registers.a & 0xF) + (value & 0xF) + (carry & 0xF) > 0xF;
        self.registers
            .set_flags(new_value == 0, false, half_carry, did_overflow);
        self.registers.a = new_value;
    }

    // A = A - s
    // * 1 * *
    fn sub(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let new_value = self.registers.a.wrapping_sub(value);
        let half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
        self.registers
            .set_flags(new_value == 0, true, half_carry, self.registers.a < value);
        self.registers.a = new_value;
    }

    // A = A - s -CY
    // * 1 * *
    fn sbc(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let (mut new_value, mut did_overflow) = self.registers.a.overflowing_sub(value);
        let carry = if self.registers.carry() { 1 } else { 0 };
        if self.registers.carry() {
            (new_value, did_overflow) = new_value.overflowing_sub(1u8);
        }
        let half_carry = (self.registers.a & 0xF) < (value & 0xF) + (carry & 0xF);
        self.registers
            .set_flags(new_value == 0, true, half_carry, did_overflow);
        self.registers.a = new_value;
    }

    // A = A & s
    // * 0 1 0
    fn and(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let new_value = self.registers.a & value;
        self.registers.set_flags(new_value == 0, false, true, false);
        self.registers.a = new_value;
    }

    // A = A | s
    // * 0 0 0
    fn or(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let new_value = self.registers.a | value;
        self.registers
            .set_flags(new_value == 0, false, false, false);
        self.registers.a = new_value;
    }

    // A = A ^ s
    // * 0 0 0
    fn xor(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let new_value = self.registers.a ^ value;
        self.registers
            .set_flags(new_value == 0, false, false, false);
        self.registers.a = new_value;
    }

    // A - s
    // * 1 * *
    fn cp(&mut self, value: u8) {
        let zero: bool = value == self.registers.a;
        let (_, did_overflow) = self.registers.a.overflowing_sub(value);
        let half_carry = (self.registers.a & 0xF) < (value & 0xF); // TODO: Double check this
        self.registers
            .set_flags(zero, true, half_carry, did_overflow);
    }

    // s = s + 1
    // * 0 * -
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        let half_carry = (value & 0xF) + (1 & 0xF) > 0xF; // TODO: Double check this
        self.registers
            .set_flags(new_value == 0, false, half_carry, self.registers.carry());
        new_value
    }

    // s = s - 1
    // * 1 * -
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        let half_carry = (value & 0xF) < 1; // TODO; Double check this
        self.registers
            .set_flags(new_value == 0, true, half_carry, self.registers.carry());
        new_value
    }

    /// Complement the carry flag
    /// - 0 0 *
    fn ccf(&mut self) {
        self.registers
            .set_flags_nz(false, false, !self.registers.carry());
    }

    /// Set the carry flag
    /// - 0 0 1
    fn scf(&mut self) {
        self.registers.set_flags_nz(false, false, true);
    }

    // 0 0 0 *
    fn rra(&mut self) {
        let curr_carry = if self.registers.carry() { 128 } else { 0 };
        let will_carry = (self.registers.a & 0x1) == 1;
        self.registers.a = curr_carry | (self.registers.a >> 1);
        self.registers.set_flags(false, false, false, will_carry);
    }

    // 0 0 0 *
    fn rla(&mut self) {
        let curr_carry = if self.registers.carry() { 1 } else { 0 };
        let will_carry = (self.registers.a & 0x80) == 0x80;
        self.registers.a = curr_carry | (self.registers.a << 1);
        self.registers.set_flags(false, false, false, will_carry);
    }

    // 0 0 0 *
    fn rrca(&mut self) {
        let carry = (self.registers.a & 0x1) == 1;
        self.registers.a = self.registers.a.rotate_right(1);
        self.registers.set_flags(false, false, false, carry);
    }

    fn rlca(&mut self) {
        let carry = (self.registers.a & 0x80) == 0x80;
        self.registers.a = self.registers.a.rotate_left(1);
        self.registers.set_flags(false, false, false, carry);
    }

    fn daa(&self) {
        unimplemented!();
    }

    // Complement the A register
    // - 1 1 -
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers
            .set_flags_nz(true, true, self.registers.carry());
    }

    // Test the bit in a register
    // * 0 1 -
    fn bit(&mut self, bit: u8, target: ArithmeticTarget) {
        let zero = (self.register_value(&target) & (1 << bit)) == 0;
        self.registers
            .set_flags(zero, false, true, self.registers.carry());
    }

    // Reset the bit at the given index
    // - - - -
    fn reset(&mut self, bit: u8, target: ArithmeticTarget) {
        let value = self.register_value(&target) & !(1 << bit);
        self.set_register_by_target(&target, value);
    }

    // Set the bit at the given index
    // - - - -
    fn set(&mut self, bit: u8, target: ArithmeticTarget) {
        let value = self.register_value(&target) | (1 << bit);
        self.set_register_by_target(&target, value);
    }

    // * 0 0 *
    fn srl(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let new_value = value >> 1;
        let carry = (value & 0x1) == 1;
        self.set_register_by_target(&target, new_value);
        self.registers
            .set_flags(new_value == 0, false, false, carry);
    }

    // * 0 0 *
    fn rr(&mut self, target: ArithmeticTarget) {
        let curr_carry = if self.registers.carry() { 128 } else { 0 };
        let value = self.register_value(&target);
        let new_value = curr_carry | (value >> 1);
        let carry = (value & 0x1) == 1;
        self.set_register_by_target(&target, new_value);
        self.registers
            .set_flags(new_value == 0, false, false, carry);
    }

    // * 0 0 *
    fn rl(&mut self, target: ArithmeticTarget) {
        let curr_carry = if self.registers.carry() { 1 } else { 0 };
        let value = self.register_value(&target);
        let new_value = curr_carry | (value << 1);
        let carry = (value & 0x80) == 0x80;
        self.set_register_by_target(&target, new_value);
        self.registers
            .set_flags(new_value == 0, false, false, carry);
    }

    // Rotate right and carry
    // * 0 0 *
    fn rrc(&mut self, value: u8) -> u8 {
        let carry = (value & 0x1) == 1;
        let new_value = value.rotate_right(1);
        self.registers
            .set_flags(new_value == 0, false, false, carry);
        new_value
    }

    // Rotate left and carry
    // * 0 0 *
    fn rlc(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) == 0x80;
        let new_value = value.rotate_left(1);
        self.registers
            .set_flags(new_value == 0, false, false, carry);
        new_value
    }

    // * 0 0 *
    fn sra(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        let carry = (value & 0x01) == 1;
        self.registers
            .set_flags(new_value == 0, false, false, carry);
        new_value
    }

    // * 0 0 *
    fn sla(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        let carry = (value & 0x80) == 0x80;
        self.registers
            .set_flags(new_value == 0, false, false, carry);
        new_value
    }

    // Swap upper and lower nibbles of a register
    // - - - -
    fn swap(&mut self, target: ArithmeticTarget) {
        let value = self.register_value(&target);
        let swapped = (value & 0x0F) << 4 | (value & 0xF0) >> 4;
        self.set_register_by_target(&target, swapped);
        self.registers.set_zero(swapped == 0);
    }

    /// Jump to the address specified by the next word 
    // - - - -
    fn jump(&mut self, should_jump: bool) -> bool {
        if should_jump {
            let least_sig = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
            let most_sig = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;
            self.pc = (most_sig << 8) | least_sig;
        } else {
            self.pc = self.pc.wrapping_add(3);
        }
        true
    }

    /// Add the immediate signed byte to the pc and jump to itAddr_0098
    // - - - -
    fn jump_relative(&mut self, should_jump: bool) -> bool {
        let old_pc = self.pc;
        let next_byte = self.read_next_byte() as i8;
        if should_jump {
            if next_byte > 0 {
                let jump_addr = next_byte as u16;
                println!("next byte is inc {:#02x}", next_byte);
                self.pc = self.pc.wrapping_add(jump_addr + 1);
                println!("jumping to {:#02x}, from: {:#02x}", self.pc, old_pc);
            } else {
                let jump_addr = next_byte.abs() as u16;
                println!("next byte is dec {:#02x}", next_byte);
                self.pc = self.pc.wrapping_sub(jump_addr - 1);
                println!("jumping to {:#02x}, from: {:#02x}", self.pc, old_pc);
            }
        } else {
            self.pc = self.pc.wrapping_add(1);
            println!("We didn't jump, skipping: {:#02x}", next_byte);
        }
        true
    }

    // - - - -
    fn load(&mut self, load_type: LoadType) {
        match load_type {
            LoadType::Byte(target, source) => self.load_byte_type(target, source),
            LoadType::Word(target, source) => self.load_word_type(target, source),
        }
    }

    fn load_byte_type(&mut self, target: LoadByteTarget, source: LoadByteSource) {
        let source_value = self.byte_from_lbs(&source);
        match target {
            LoadByteTarget::A => self.registers.a = source_value,
            LoadByteTarget::B => self.registers.b = source_value,
            LoadByteTarget::C => self.registers.c = source_value,
            LoadByteTarget::D => self.registers.d = source_value,
            LoadByteTarget::E => self.registers.e = source_value,
            LoadByteTarget::H => self.registers.h = source_value,
            LoadByteTarget::L => self.registers.l = source_value,
            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
            // I think the only possible source value here comes from register a.
            LoadByteTarget::BCI => self.bus.write_byte(self.registers.get_bc(), source_value), // write to the location in memory stored at the address stored in this register
            LoadByteTarget::DEI => self.bus.write_byte(self.registers.get_de(), source_value),
            LoadByteTarget::HLINC => {
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
            }
            LoadByteTarget::HLDEC => {
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
            }
        }
        match source {
            // If we read from the D8, we should move the pc up one extra spot
            // TODO: May be able to remove this condition
            LoadByteSource::D8 => {
                self.pc.wrapping_add(1);
            }
            LoadByteSource::HLINC => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
            }
            LoadByteSource::HLDEC => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
            }
            _ => {}
        }
    }

    fn byte_from_lbs(&mut self, source: &LoadByteSource) -> u8 {
        match source {
            LoadByteSource::A => self.registers.a,
            LoadByteSource::B => self.registers.b,
            LoadByteSource::C => self.registers.c,
            LoadByteSource::D => self.registers.d,
            LoadByteSource::E => self.registers.e,
            LoadByteSource::H => self.registers.h,
            LoadByteSource::L => self.registers.l,
            LoadByteSource::D8 => self.read_next_byte(), // TODO: Double check this
            LoadByteSource::HLI | LoadByteSource::HLINC | LoadByteSource::HLDEC => {
                self.bus.read_byte(self.registers.get_hl())
            }
            LoadByteSource::BCI => self.bus.read_byte(self.registers.get_bc()),
            LoadByteSource::DEI => self.bus.read_byte(self.registers.get_de()),
        }
    }

    fn load_word_type(&mut self, target: LoadWordTarget, source: LoadWordSource) {
        let source_value = self.word_from_lws(&source);
        match target {
            LoadWordTarget::BC => self.registers.set_bc(source_value),
            LoadWordTarget::DE => self.registers.set_de(source_value),
            LoadWordTarget::HL => self.registers.set_hl(source_value),
            LoadWordTarget::SP => self.sp = source_value,
            LoadWordTarget::D16 => {
                let addr = self.read_next_word();
                self.bus.write_word(addr, self.sp);
            }
        }
    }

    fn word_from_lws(&mut self, source: &LoadWordSource) -> u16 {
        match source {
            LoadWordSource::BC => self.registers.get_bc(),
            LoadWordSource::DE => self.registers.get_de(),
            LoadWordSource::HL => self.registers.get_hl(),
            LoadWordSource::SP => self.sp,
            LoadWordSource::D16 => self.read_next_word(),
        }
    }

    // Load A into the address given by the next word
    // mem[mem.next()] = A
    // - - - -
    // TODO: Maybe rename this something better
    fn load_a_into_next_byte(&mut self) {
        let addr = self.read_next_word();
        self.bus.write_byte(addr, self.registers.a);
    }

    // A = mem[nn]; n = next_word()
    // - - - -
    fn load_byte_at_next_address_into_a(&mut self) {
        let addr = self.read_next_word();
        self.registers.a = self.bus.read_byte(addr);
    }

    // A = mem[0xff00 + C]
    // - - - -
    fn ldac(&mut self) {
        let value = self.bus.read_byte(0xFF00 + self.registers.c as u16);
        self.registers.a = value;
    }

    // mem[0xff00 + C] = A
    // - - - -
    fn ldca(&mut self) {
        self.bus
            .write_byte(0xFF00 + self.registers.c as u16, self.registers.a);
    }

    // Put sp plus n effective address into hl
    // 0 0 H C
    fn ldhlsp(&mut self) {
        let byte = self.read_next_byte();
        let new_value = self.addsp(byte);
        self.registers.set_hl(new_value);
    }

    /// Halt CPU until an interrupt occurs.
    /// - - - -
    fn halt(&mut self) {
        self.is_halted = true;
    }

    // (SP-1) = ssh, (SP-2) = ssl, SP = SP-2
    // - - - -
    fn push_from_target(&mut self, target: StackTarget) {
        let value = match target {
            StackTarget::AF => self.registers.get_af(),
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
        };
        self.push(value);
    }

    // (SP-1) = ssh, (SP-2) = ssl, SP = SP-2
    // - - - -
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

    /// Pushes the address of the next instruction onto the stack
    /// - - - -
    fn call(&mut self, should_jump: bool) -> bool {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.pc = self.bus.read_word(self.pc.wrapping_add(1));
        } else {
            self.pc = next_pc;
        }
        println!("Calling {:#02x}", self.pc);
        true
    }

    /// Pops the stack and jumps to that address
    /// - - - -
    fn ret(&mut self, should_jump: bool) -> bool {
        if should_jump {
            self.pc = self.pop();
            println!("RET to {:#02x}", self.pc);
            true
        } else {
            false
        }
    }

    fn reti(&mut self) {
        self.ime = false;
        self.pc = self.pop();
    }

    fn rst(&mut self, addr: RestartAddr) {
        self.push(self.pc);
        self.pc = u16::from(addr);
    }

    // - - - -
    fn jump_to_address_hl(&mut self) {
        self.pc = self.registers.get_hl();
    }

    fn enable_interupts(&mut self) {
        self.ime = true;
    }

    fn disable_interupts(&mut self) {
        self.ime = false;
    }

    // put A into memory address $FF00+n
    // - - - -
    fn ldha(&mut self) {
        let n = self.read_next_byte();
        self.bus.write_byte(0xFF00 + n as u16, self.registers.a);
    }

    // A = mem[FF00 + n], n = mem.next
    // - - - -
    fn ldha8(&mut self) {
        let n = self.read_next_byte();
        self.registers.a = self.bus.read_byte(0xFF00 + n as u16);
        println!("A is now {:#02x}", self.registers.a);
    }

    fn stop(&mut self) {
        // TODO: We'll need to impl this op to continue
        unimplemented!();
    }

    fn read_next_byte(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        let byte = self.bus.read_byte(self.pc);
        byte
    }

    fn read_next_word(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        let word = self.bus.read_word(self.pc);
        self.pc = self.pc.wrapping_add(1); // We read two bytes so we need to increment one more;
        word
    }
}
