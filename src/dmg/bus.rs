use crate::dmg::apu::Apu;
use crate::dmg::busconnection::BusConnection;
use crate::dmg::joypad::Joypad;
use crate::dmg::memory::{Interrupt, Memory};
use crate::dmg::ppu::PPU;
use crate::dmg::timer::Timer;
use crate::dmg::boot_rom::BootRom;
use crate::dmg::cartridge::Cartridge;

/// Struct for representing the bus which serves as the interface
/// through which the cpu can communicate with other devices
pub struct Bus {
    memory: Memory,
    timer: Timer,
    ppu: PPU,
    apu: Apu,
    joypad: Joypad,
    cartridge: Cartridge,
    boot_rom: BootRom,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            memory: Memory::default(),
            timer: Timer::default(),
            ppu: PPU::default(),
            apu: Apu::default(),
            joypad: Joypad::default(),
            cartridge: Cartridge::default(),
            boot_rom: BootRom::default(),
        }
    }
}

impl Bus {
    pub fn read_byte(&self, address: u16) -> u8 {
        // TODO: Add the rest pointing to other devices
        if address <= 0xFF && self.memory.read_byte(0xFF50) == 0 {
            return self.boot_rom.read_byte(address);
        }
        match address {
            0x0000 ..= 0x7FFF | 0xA000 ..= 0xBFFF => self.cartridge.read_byte(address),
            0x8000..=0x9FFF | 0xFF40..=0xFF4B | 0xFE00..=0xFE9F => self.ppu.read_byte(address),
            0xFF00 => self.joypad.read_byte(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF10..=0xFF14 | 0xFF16..=0xFF1E | 0xFF20..=0xFF26 | 0xFF30..=0xFF3F => {
                self.apu.read(address)
            }
            0xFEA0..=0xFEFF => 0xFF, /* Unused Memory. Return Default value */
            _ => self.memory.read_byte(address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // TODO: Add the rest pointing to other devices
        match address {
            0x0000 ..= 0x7FFF | 0xA000 ..= 0xBFFF => self.cartridge.write_byte(address, value),
            0xFF00 => self.joypad.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF10..=0xFF14 | 0xFF16..=0xFF1E | 0xFF20..=0xFF26 | 0xFF30..=0xFF3F => {
                self.apu.write(address, value)
            }
            0x8000..=0x9FFF | 0xFF40..=0xFF4B | 0xFE00..=0xFE9F => {
                self.ppu.write_byte(address, value)
            }
            0xFEA0..=0xFEFF => { /* Unused memory. Do Nothing */ }
            _ => self.memory.write_byte(address, value),
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let l_byte = self.read_byte(address);
        let h_byte = self.read_byte(address + 1);
        ((h_byte as u16) << 8) | l_byte as u16
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let h_byte = (value >> 8) as u8;
        let l_byte = value as u8;
        self.write_byte(address, l_byte);
        self.write_byte(address + 1, h_byte);
    }

    pub fn interrupt_flag_off(&mut self) {
        self.memory.interrupt_flag_off();
    }

    pub fn return_interrupt(&self) -> Interrupt {
        self.memory.return_interrupt()
    }

    pub fn step(&mut self, cycles: u8) {
        self.timer.step(cycles);
        self.ppu.step(cycles);
    }
}

#[cfg(test)]
fn setup() -> Bus {
    Bus::default()
}

#[test]
fn write_to_mem() {
    let mut bus = setup();
    bus.write_byte(0xC000, 0xAA);
    assert_eq!(bus.memory.read_word(0xC000), 0xAA);
}

#[test]
fn write_to_ppu_lcdc() {
    let mut bus = setup();
    bus.write_byte(0xFF40, 0xAA);
    assert_eq!(bus.ppu.lcdc(), 0xAA);
}

#[test]
fn write_to_ppu_stat() {
    let mut bus = setup();
    bus.write_byte(0xFF41, 0b00101010);
    assert_eq!(bus.ppu.stat(), 0b00101010);
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

#[test]
fn unused_memory() {
    let mut bus = setup();
    bus.write_byte(0xFEA1, 0xAA);
    for i in 0xFEA0..=0xFEFF {
        assert_eq!(bus.read_byte(i), 0xFF);
    }
}

#[test]
fn vram() {
    let mut bus = setup();
    bus.write_byte(0x9000, 0xAA);
    assert_eq!(bus.ppu.vram()[0x1000], 0xAA);
}

#[test]
fn oam() {
    let mut bus = setup();
    bus.write_byte(0xFE10, 0xAA);
    assert_eq!(bus.ppu.oam()[0x0010], 0xAA);
}

#[test]
fn disable_boot_rom() {
    let mut bus = setup();
    assert_eq!(bus.read_byte(0xFF50), 0);
    assert_eq!(bus.read_byte(0xFF), 0x50);
    bus.write_byte(0xFF50, 1);
    assert_eq!(bus.read_byte(0xFF), 0, "We should be reading from memory now instead of the bootrom");
}

#[test]
fn default_no_cart_is_rom() {
    let mut bus = setup();
    assert_eq!(bus.read_byte(0xA000), 0);
    bus.write_byte(0xA000, 10);
    assert_eq!(bus.read_byte(0xA000), 0, "0xA000 should still be zero because we have no cart and default to ROM");
}