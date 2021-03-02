use crate::dmg::busconnection::BusConnection;
use crate::dmg::ppu::lcdc::Lcdc;
use crate::dmg::ppu::stat::{LcdMode, Stat};

// The number of CPU cycles taken to draw one scanline
const SCANLINE_COUNTER_MAX: u16 = 456;

// The first 80 of the 456 cycles to draw a scanline are used in mode 2,
// searching sprite attributes. (465 - 80 = 476)
const SEARCHING_FOR_SPRITES: u16 = 376;

// The second section of the 456 cycles is 172 cycles spent in mode 3,
// Transfering to the lcd driver. (376 - 172)
const TRANSFERING_TO_LCD_DRIVER: u16 = 204;

// The DMG screen resolution is 160x144 meaning there are 144 visible lines
// Everything afterwards is invisible.
const VISIBLE_SCAN_LINES: u8 = 144;

// The total number of visible and invisible scanlines
const MAX_SCAN_LINES: u8 = 153;

/// The Pixel Processing Unit
pub struct PPU {
    lcdc: Lcdc, // 0xFF40
    stat: Stat, // 0xFF41
    scy: u8,    // 0xFF42
    scx: u8,    // 0xFF43
    ly: u8,     // 0xFF44
    lyc: u8,    // 0xFF45

    bgp: u8,  // 0xFF47
    obp0: u8, // 0xFF48
    obp1: u8, // 0xFF49

    wy: u8, //0xFF4A
    wx: u8, //0xFF4B
    scanline_counter: u16,
    vram: [u8; 8192],

    /// An array of 40, 4-byte objects
    oam: [u8; 160], // could also be [u32; 40]
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            lcdc: Lcdc::default(),
            stat: Stat::default(),
            scy: 0,
            scx: 0,
            ly: 0, // The current scanline we're on
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            scanline_counter: SCANLINE_COUNTER_MAX, // Similar to the timer counter and how we count down. There are 456 dots per scanline,
            vram: [0; 8192],
            oam: [0; 160],
        }
    }
}

impl BusConnection for PPU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.read_vram(address),
            0xFE00..=0xFE9F => self.read_oam(address),
            0xFF40 => u8::from(&self.lcdc),
            0xFF41 => u8::from(&self.stat),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("This should never happen! Address: {:#02x}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.write_vram(address, value),
            0xFE00..=0xFE9F => self.write_oam(address, value),
            0xFF40 => self.lcdc = Lcdc::from(&value),
            0xFF41 => self.stat = Stat::from(&value),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = 0,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => panic!("This should never happen! Address: {:#02x}", address),
        }
    }
}

impl PPU {
    pub fn step(&mut self, cycles: u8) {
        self.set_lcd_status();

        if self.lcdc.lcdc_enabled() {
            let (new_count, did_overflow) = self.scanline_counter.overflowing_sub(cycles as u16);
            self.scanline_counter = new_count;
            if did_overflow {
                // Move to the next scanline
                self.ly = self.ly.wrapping_add(1);

                // Reset the scanline counter
                self.scanline_counter = SCANLINE_COUNTER_MAX;

                // Vertical blank period
                if self.ly == VISIBLE_SCAN_LINES {
                    // Trigger vblank
                    // TODO: Trigger the interrupt 0
                } else if self.ly > MAX_SCAN_LINES {
                    self.ly = 0;
                } else if self.ly < VISIBLE_SCAN_LINES {
                    self.draw_scanline();
                }
            }
        }
    }

    fn set_lcd_status(&mut self) {
        if !self.lcdc.lcdc_enabled() {
            self.scanline_counter = SCANLINE_COUNTER_MAX;
            self.ly = 0;
            self.stat.mode_flag = LcdMode::VBlank;
        } else {
            let current_mode = self.stat.mode_flag;
            let mut interrupt_triggered = false;
            let new_mode;

            if self.ly >= VISIBLE_SCAN_LINES {
                new_mode = LcdMode::VBlank;
                self.stat.mode_flag = LcdMode::VBlank;
                interrupt_triggered = self.stat.mode_01;
            } else if self.scanline_counter >= SEARCHING_FOR_SPRITES {
                new_mode = LcdMode::SearchSpriteAttributes;
                self.stat.mode_flag = LcdMode::SearchSpriteAttributes;
                interrupt_triggered = self.stat.mode_10;
            } else if self.scanline_counter >= TRANSFERING_TO_LCD_DRIVER {
                new_mode = LcdMode::TransferingDataToLCDDriver;
                self.stat.mode_flag = LcdMode::TransferingDataToLCDDriver;
            } else {
                new_mode = LcdMode::HBlank;
                self.stat.mode_flag = LcdMode::HBlank;
                interrupt_triggered = self.stat.mode_00;
            }

            if interrupt_triggered && (new_mode != current_mode) {
                // TODO: RequestInterrupt(1)
            }

            if self.ly == self.lyc {
                self.stat.coincidence_flag = true;
                if self.stat.coincidence_selectable {
                    // TODO: Request Interrupt 1
                }
            } else {
                self.stat.coincidence_flag = false;
            }
        }
    }

    fn draw_scanline(&mut self) {
        if self.lcdc.bg_window_display() {
            self.render_tiles();
        }

        if self.lcdc.obj_display() {
            self.render_sprites();
        }
    }

    /// Renders the window and background tiles
    fn render_tiles(&self) {
        // TODO
    }

    fn render_sprites(&self) {
        // TODO
    }

    /// VRAM is only accessible during Modes 0-2
    /// Reading when the mode flag is set to 3 will return the default value of 0xFF
    /// See: https://gbdev.io/pandocs/#accessing-vram-and-oam for more info
    fn read_vram(&self, address: u16) -> u8 {
        if self.stat.mode_flag == LcdMode::TransferingDataToLCDDriver {
            0xFF
        } else {
            self.vram[(address - 0x8000) as usize]
        }
    }

    /// VRAM is only accessible during Modes 0-2
    /// Writing when the mode flag is set to 3 will not change the data
    fn write_vram(&mut self, address: u16, value: u8) {
        if self.stat.mode_flag != LcdMode::TransferingDataToLCDDriver {
            self.vram[(address - 0x8000) as usize] = value;
        }
    }

    /// OAM is only accessible during Modes 0 & 1
    /// Reading when the mode flag is set to 2 or 3 will return the default value of 0xFF
    /// See: https://gbdev.io/pandocs/#accessing-vram-and-oam for more info
    fn read_oam(&self, address: u16) -> u8 {
        match self.stat.mode_flag {
            LcdMode::HBlank | LcdMode::VBlank => self.oam[(address - 0xFE00) as usize],
            LcdMode::SearchSpriteAttributes | LcdMode::TransferingDataToLCDDriver => 0xFF,
        }
    }

    /// OAM is only accessible during Modes 0 & 1
    /// Writing when the mode flag is set to 2 or 3 will not change the data
    fn write_oam(&mut self, address: u16, value: u8) {
        if self.stat.mode_flag == LcdMode::HBlank || self.stat.mode_flag == LcdMode::VBlank {
            self.oam[(address - 0xFE00) as usize] = value;
        }
    }

    #[cfg(test)]
    pub fn lcdc(&self) -> u8 {
        u8::from(&self.lcdc)
    }

    #[cfg(test)]
    pub fn stat(&self) -> u8 {
        u8::from(&self.stat)
    }

    #[cfg(test)]
    pub fn vram(&self) -> [u8; 8192] {
        self.vram
    }

    #[cfg(test)]
    pub fn oam(&self) -> [u8; 160] {
        self.oam
    }
}

#[test]
fn lcd_is_enabled() {
    let mut ppu = PPU::default();
    assert_eq!(ppu.lcdc.lcdc_enabled(), false);
    ppu.lcdc = Lcdc::from(&255);
    assert_eq!(ppu.lcdc.lcdc_enabled(), true);
}

#[test]
fn ly_inc() {
    let mut ppu = PPU::default();
    ppu.lcdc = Lcdc::from(&255);
    assert_eq!(ppu.ly, 0);
    ppu.step(255);
    ppu.step(255);
    assert_eq!(ppu.ly, 1, "Ly should be incd after 456 cycles");
}

#[test]
fn write_to_ly() {
    let mut ppu = PPU::default();
    ppu.write_byte(0xFF44, 10);
    assert_eq!(ppu.lcdc(), 0, "Writing to ly should reset the value");
}

#[test]
fn write_to_vram() {
    let mut ppu = PPU::default();
    ppu.write_byte(0x9000, 0xAA);
    assert_eq!(ppu.vram()[0x1000], 0xAA);
    assert_eq!(ppu.read_byte(0x9000), 0xAA);
}

#[test]
fn write_to_end_of_vram() {
    let mut ppu = PPU::default();
    ppu.write_byte(0x9FFF, 0xAA);
    assert_eq!(ppu.vram()[0x1FFF], 0xAA);
    assert_eq!(ppu.read_byte(0x9FFF), 0xAA);
}

#[test]
fn read_vram_when_stat_mode_3() {
    let mut ppu = PPU::default();
    ppu.write_byte(0x9000, 0xAA);
    ppu.stat.mode_flag = LcdMode::TransferingDataToLCDDriver;
    assert_eq!(
        ppu.vram()[0x1000],
        0xAA,
        "The correct byte 0xAA should be present in Vram"
    );
    assert_eq!(
        ppu.read_byte(0x9000),
        0xFF,
        "VRAM read through it's public methods should return 0xFF when disabled"
    );
}

#[test]
fn write_vram_when_stat_mode_3() {
    let mut ppu = PPU::default();
    ppu.write_byte(0x9000, 0xAA);
    ppu.stat.mode_flag = LcdMode::TransferingDataToLCDDriver;
    ppu.write_byte(0x9000, 0xBB);
    assert_eq!(
        ppu.vram()[0x1000],
        0xAA,
        "VRAM should not have been written to a second time"
    );
}

#[test]
fn write_to_oam() {
    let mut ppu = PPU::default();
    ppu.write_byte(0xFE10, 0xAA);
    assert_eq!(ppu.oam()[0x0010], 0xAA);
    assert_eq!(ppu.read_byte(0xFE10), 0xAA);
}

#[test]
fn write_to_end_of_oam() {
    let mut ppu = PPU::default();
    ppu.write_byte(0xFE9F, 0xAA);
    assert_eq!(ppu.oam()[0x009F], 0xAA);
    assert_eq!(ppu.read_byte(0xFE9f), 0xAA);
}

#[test]
fn read_oam_when_stat_mode_3() {
    let mut ppu = PPU::default();
    ppu.write_byte(0xFE10, 0xAA);
    ppu.stat.mode_flag = LcdMode::TransferingDataToLCDDriver;
    assert_eq!(
        ppu.oam()[0x0010],
        0xAA,
        "The correct byte 0xAA should be present in OAM"
    );
    assert_eq!(
        ppu.read_byte(0xFE10),
        0xFF,
        "OAM read through it's public methods should return 0xFF when disabled"
    );
}

#[test]
fn write_oam_when_stat_mode_3() {
    let mut ppu = PPU::default();
    ppu.write_byte(0xFE10, 0xAA);
    ppu.stat.mode_flag = LcdMode::TransferingDataToLCDDriver;
    ppu.write_byte(0xFE10, 0xBB);
    assert_eq!(
        ppu.oam()[0x0010],
        0xAA,
        "OAM should not have been written to a second time"
    );
}

// TODO: Tests for the step function effects on ppu.stat