const COINCIDENCE_SELECTABLE_POS: u8 = 6;
const MODE_10_POS: u8 = 5;
const MODE_01_POS: u8 = 4;
const MODE_00_POS: u8 = 3;
const COINCIDENCE_FLAG_POS: u8 = 2;

#[derive(Debug, PartialEq)]
pub struct Stat {
    pub coincidence_selectable: bool,
    pub mode_10: bool,
    pub mode_01: bool,
    pub mode_00: bool,
    pub coincidence_flag: bool,
    pub mode_flag: LcdMode,
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
pub enum LcdMode {
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