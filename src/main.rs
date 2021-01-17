mod cpu;

use oxide_boy::ArithmeticTarget;
use oxide_boy::Instruction;
use oxide_boy::CPU;

fn main() {
    println!("Starting emulator!");
    let mut cpu = CPU::default();
    loop {
        cpu.step();
    }
}

    // cpu.execute(Instruction::ADD(ArithmeticTarget::C));
    // cpu.execute(Instruction::SUB(ArithmeticTarget::C));
    // cpu.execute(Instruction::ADC(ArithmeticTarget::C));
    // cpu.execute(Instruction::AND(ArithmeticTarget::C));
    // cpu.execute(Instruction::OR(ArithmeticTarget::C));
    // cpu.execute(Instruction::XOR(ArithmeticTarget::C));
    // cpu.execute(Instruction::INC(ArithmeticTarget::A));
    // cpu.execute(Instruction::DEC(ArithmeticTarget::A));
    // cpu.execute(Instruction::SWAP(ArithmeticTarget::A));
