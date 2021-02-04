use crate::dmg::apu::Apu;
use crate::dmg::joypad::Joypad;
use crate::dmg::memory::{Interrupt, Memory};
use crate::dmg::ppu::PPU;
use crate::dmg::timer::Timer;
use crate::dmg::busconnection::BusConnection;

/// Struct for representing the bus which serves as the interface
/// through which the cpu can communicate with other devices
pub struct Bus {
    memory: Memory,
    timer: Timer,
    ppu: PPU,
    apu: Apu,
    joypad: Joypad,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            memory: Memory::default(),
            timer: Timer::default(),
            ppu: PPU::default(),
            apu: Apu::default(),
            joypad: Joypad::default(),
        }
    }
}

impl Bus {
    pub fn read_byte(&self, address: u16) -> u8 {
        // TODO: Add the rest pointing to other devices
        match address {
            0xFF00 => self.joypad.read_byte(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF10..=0xFF14 | 0xFF16..=0xFF1E | 0xFF20..=0xFF26 | 0xFF30..=0xFF3F => {
                self.apu.read(address)
            }
            0xFF40..=0xFF45 => self.ppu.read_byte(address),
            _ => self.memory.read_byte(address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // TODO: Add the rest pointing to other devices
        match address {
            0xFF00 => self.joypad.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF10..=0xFF14 | 0xFF16..=0xFF1E | 0xFF20..=0xFF26 | 0xFF30..=0xFF3F => {
                self.apu.write(address, value)
            }
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.write_byte(address, value),
            _ => self.memory.write_byte(address, value),
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        // TODO: Same as below
        self.memory.read_word(address)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        // TODO: It's possible that one of the bytes could be in one or even two, of our devices so we might have to
        // change this to use the write_byte method which takes that into account.
        self.memory.write_word(address, value);
    }

    pub fn interrupt_flag_off(&mut self) {
        self.memory.interrupt_flag_off();
    }

    pub fn return_interrupt(&self) -> Interrupt {
        self.memory.return_interrupt()
    }

    pub fn step(&mut self, cycles: u8) {
        self.ppu.step();
        self.timer.step(cycles);
    }
}

#[cfg(test)]
fn setup() -> Bus {
    Bus::default()
}

#[test]
fn write_to_mem() {
    let mut bus = setup();
    bus.write_byte(0x1000, 0xAA);
    assert_eq!(bus.memory.read_word(0x1000), 0xAA);
}

#[test]
fn write_to_ppu() {
    let mut bus = setup();
    bus.write_byte(0xFF44, 0xAA);
    assert_eq!(bus.ppu.ly(), 0xAA);
}

#[test]
fn write_to_timer_div() {
    let mut bus = setup();
    bus.write_byte(0xFF04, 0xAA);
    assert_eq!(bus.timer.div(), 0, "Writing to div should reset it to zero");
}

#[test]
fn write_to_timer_tima() {
    let mut bus = setup();
    bus.write_byte(0xFF05, 0xAA);
    assert_eq!(bus.timer.tima(), 0xAA);
}

#[test]
fn write_to_apu_sweep_register() {
    let mut bus = setup();
    bus.write_byte(0xFF10, 0xAA);
    assert_eq!(bus.apu.sweep_register(), 0xAA);
}

#[test]
fn write_to_apu_wave_pattern_ram() {
    let mut bus = setup();
    bus.write_byte(0xFF30, 0x10);
    bus.write_byte(0xFF3F, 0xAA);
    assert_eq!(bus.apu.wave_pattern_ram()[0], 0x10);
    assert_eq!(bus.apu.wave_pattern_ram()[0xF], 0xAA);
}
