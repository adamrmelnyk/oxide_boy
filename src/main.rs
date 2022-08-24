pub mod dmg;

use log::info;
use oxide_boy::CPU;
use structopt::StructOpt;

use env_logger;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    #[structopt(about = "Runs the specified ROM", help = "USEAGE: run myRomFile.rom")]
    Run { file: String },
    #[structopt(
        about = "Runs the specified ROM as the BOOT ROM, use this to run custom boot roms or test roms",
        help = "USEAGE: custom myBootRom.rom myRomFile.rom"
    )]
    Custom { boot_file: String, file: String },
    #[structopt(
        about = "Runs the specified ROM, but skips the boot sequence",
        help = "USEAGE: skip myRomFile.rom"
    )]
    Skip { file: String },
    #[structopt(
        about = "For development purposes: Runs the default rom at src/dmg/rom/DEFAULT_ROM.bin",
        help = "USEAGE: default"
    )]
    Default,
}

fn main() {
    env_logger::init();
    let args = Command::from_args();
    match args {
        Command::Run { file } => run(file),
        Command::Default => default(),
        Command::Custom { boot_file, file } => custom(boot_file, file),
        Command::Skip { file } => skip(file),
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

fn custom(boot_file: String, file: String) {
    let mut cpu = CPU::custom_boot_rom(&boot_file, &file);
    loop {
        cpu.step();
    }
}

fn skip(file: String) {
    let mut cpu = CPU::new(&file);
    cpu.bus.write_byte(0xFF50, 0x1); // Disables the boot rom
    cpu.pc = 0x100;
    loop {
        cpu.step();
    }
}
