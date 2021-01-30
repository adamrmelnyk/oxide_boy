#![feature(destructuring_assignment)]

mod dmg;

pub use dmg::cpu::CPU;
pub use dmg::instructions::{
    ArithmeticTarget, Instruction, JumpCond, RestartAddr, SixteenBitArithmeticTarget, StackTarget,
};
pub use dmg::memory::{
    Interrupt, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget,
};
pub use dmg::registers::Registers;
