mod cpu;

use gb_emulator::Instruction;
use gb_emulator::ArithmeticTarget;
use gb_emulator::CPU;



fn main() {
    println!("Hello, world!");
    let mut cpu = CPU::default();
    cpu.execute(Instruction::ADD(ArithmeticTarget::C));
    cpu.execute(Instruction::SUB(ArithmeticTarget::C));
    cpu.execute(Instruction::ADC(ArithmeticTarget::C));
    cpu.execute(Instruction::AND(ArithmeticTarget::C));
    cpu.execute(Instruction::OR(ArithmeticTarget::C));
    cpu.execute(Instruction::XOR(ArithmeticTarget::C));
    cpu.execute(Instruction::INC(ArithmeticTarget::A));
    cpu.execute(Instruction::DEC(ArithmeticTarget::A));
}
