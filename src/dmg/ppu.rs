use crate::dmg::busconnection::BusConnection;

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
    lcdc: u8, // 0xFF40
        // TODO: Break down LCDC
    stat: Stat, // 0xFF41
    scy: u8,  // 0xFF42
    scx: u8,  // 0xFF43Scammer BEGS For His Deleted Files As I Drink His Tears
    ly: u8,   // 0xFF44
    lyc: u8,  // 0xFF45

    bgp: u8,  // 0xFF47
    obp0: u8, // 0xFF48
    obp1: u8, // 0xFF49

    wy: u8, //0xFF4A
    wx: u8, //0xFF4B
    scanline_counter: u16,
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            lcdc: 0,
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
            scanline_counter: SCANLINE_COUNTER_MAX, // Similar to the timer counter and how we count down. There are 456 dots per scanline
        }
    }
}

impl BusConnection for PPU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc,
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
            _ => panic!("This should never happen"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcdc = value,
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
            _ => panic!("This should never happen"),
        }
    }
}

impl PPU {
    pub fn step(&mut self, cycles: u8) {
        self.set_lcd_status();

        if self.lcd_enabled() {
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
        if !self.lcd_enabled() {
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
        if self.bg_display_enabled() {
            self.render_tiles();
        }

        if self.obj_display_enabled() {
            self.render_sprites();
        }
    }

    fn render_tiles(&self) {

    }

    fn render_sprites(&self) {

    }

    fn lcd_enabled(&self) -> bool {
        self.lcdc >> 7 == 1
    }

    fn bg_display_enabled(&self) -> bool {
        self.lcdc & 1 == 1
    }

    fn obj_display_enabled(&self) -> bool {
        self.lcdc & 2 == 2
    }

    #[cfg(test)]
    pub fn lcdc(&self) -> u8 {
        self.lcdc
    }
}

const COINCIDENCE_SELECTABLE_POS: u8 = 6;
const MODE_10_POS: u8 = 5;
const MODE_01_POS: u8 = 4;
const MODE_00_POS: u8 = 3;
const COINCIDENCE_FLAG_POS: u8 = 2;

#[derive(Debug, PartialEq)]
struct Stat {
    coincidence_selectable: bool,
    mode_10: bool,
    mode_01: bool,
    mode_00: bool,
    coincidence_flag: bool,
    mode_flag: LcdMode,
}

impl Default for Stat {
    fn default() -> Stat {
        Stat {
            coincidence_selectable: false,
            mode_10: false,
            mode_01: false,
            mode_00: false,
            coincidence_flag: false,
            mode_flag: LcdMode::HBlank,
        }
    }
}

impl std::convert::From<&Stat> for u8 {
    fn from(stat: &Stat) -> u8 {
        (if stat.coincidence_selectable { 1 } else { 0 }) << COINCIDENCE_SELECTABLE_POS
            | (if stat.mode_10 { 1 } else { 0 }) << MODE_10_POS
            | (if stat.mode_01 { 1 } else { 0 }) << MODE_01_POS
            | (if stat.mode_00 { 1 } else { 0 }) << MODE_00_POS
            | (if stat.coincidence_flag { 1 } else { 0 }) << COINCIDENCE_FLAG_POS
            | u8::from(&stat.mode_flag)
    }
}

impl std::convert::From<&u8> for Stat {
    fn from(byte: &u8) -> Stat {
        let coincidence_selectable = (byte >> COINCIDENCE_SELECTABLE_POS) & 0b1 != 0;
        let mode_10 = (byte >> MODE_10_POS) & 0b1 != 0;
        let mode_01 = (byte >> MODE_01_POS) & 0b1 != 0;
        let mode_00 = (byte >> MODE_00_POS) & 0b1 != 0;
        let coincidence_flag= (byte >> COINCIDENCE_FLAG_POS) & 0b1 != 0;
        let mode_flag = LcdMode::from(*byte);
        
        Stat {
            coincidence_selectable,
            mode_10,
            mode_01,
            mode_00,
            coincidence_flag,
            mode_flag,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum LcdMode {
    HBlank,
    VBlank,
    SearchSpriteAttributes,
    TransferingDataToLCDDriver,
}

impl std::convert::From<&LcdMode> for u8 {
    fn from(mode: &LcdMode) -> u8 {
        match mode {
            LcdMode::HBlank => 0b00,
            LcdMode::VBlank => 0b01,
            LcdMode::SearchSpriteAttributes => 0b10,
            LcdMode::TransferingDataToLCDDriver => 0b11,
        }
    }
}

impl std::convert::From<u8> for LcdMode {
    fn from(byte: u8) -> LcdMode {
        match byte & 3 {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::SearchSpriteAttributes,
            3 => LcdMode::TransferingDataToLCDDriver,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }
}

#[test]
fn lcd_is_enabled() {
    let mut ppu = PPU::default();
    assert_eq!(ppu.lcd_enabled(), false);
    ppu.lcdc = 255;
    assert_eq!(ppu.lcd_enabled(), true);
}

#[test]
fn ly_inc() {
    let mut ppu = PPU::default();
    ppu.lcdc = 255;
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
fn u8_to_lcd_mode() {
    let eight = 0b0000_0011;
    assert_eq!(LcdMode::from(eight), LcdMode::TransferingDataToLCDDriver);
    let eight = 0b0000_0010;
    assert_eq!(LcdMode::from(eight), LcdMode::SearchSpriteAttributes);
    let eight = 0b0000_0001;
    assert_eq!(LcdMode::from(eight), LcdMode::VBlank);
    let eight = 0b0000_0000;
    assert_eq!(LcdMode::from(eight), LcdMode::HBlank);
}

#[test]
fn u8_to_stat() {
    let stat: u8 = 0b0110_0110;
    let expected = Stat {
        coincidence_selectable: true,
        mode_10: true,
        mode_01: false,
        mode_00: false,
        coincidence_flag: true,
        mode_flag: LcdMode::SearchSpriteAttributes,
    };
    assert_eq!(Stat::from(&stat), expected);
}

#[test]
fn lcd_to_u8() {
    let mode = LcdMode::TransferingDataToLCDDriver;
    assert_eq!(u8::from(&mode), 0b0000_0011);
    let mode = LcdMode::SearchSpriteAttributes;
    assert_eq!(u8::from(&mode), 0b0000_0010);
    let mode = LcdMode::VBlank;
    assert_eq!(u8::from(&mode), 0b0000_0001);
    let mode = LcdMode::HBlank;
    assert_eq!(u8::from(&mode), 0b0000_0000);
}
#[test]
fn stat_to_u8() {
    let expected: u8 = 0b0110_0110;
    let stat = Stat {
        coincidence_selectable: true,
        mode_10: true,
        mode_01: false,
        mode_00: false,
        coincidence_flag: true,
        mode_flag: LcdMode::SearchSpriteAttributes,
    };
    assert_eq!(u8::from(&stat), expected);
}

// TODO: Tests for the step function