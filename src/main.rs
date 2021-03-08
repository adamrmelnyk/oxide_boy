#![feature(destructuring_assignment)]
pub mod dmg;

use log::info;
use oxide_boy::CPU;
use structopt::StructOpt;

use env_logger;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    #[structopt(
        about = "Runs the specified ROM",
        help = "USEAGE: run myRomFile.rom",
    )]
    Run {
        file: String,
    },
    #[structopt(
        about = "For development purposes: Runs the default rom at src/dmg/rom/DEFAULT_ROM.bin",
        help = "USEAGE: default",
    )]
    Default,
}

fn main() {
    env_logger::init();
    let args = Command::from_args();
    match args {
        Command::Run { file } => run(file),
        Command::Default => default(),
    }
}

fn default() {
    info!("Starting emulator!");
    let mut cpu = CPU::default();
    loop {
        cpu.step();
    }
}

fn run(file: String) {
    let mut cpu = CPU::new(&file);
    loop {
        cpu.step();
    }
}
