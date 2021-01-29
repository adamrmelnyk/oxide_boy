#![feature(destructuring_assignment)]

mod dmg;

use oxide_boy::CPU;

fn main() {
    println!("Starting emulator!");
    let mut cpu = CPU::default();
    loop {
        cpu.step();
    }
}