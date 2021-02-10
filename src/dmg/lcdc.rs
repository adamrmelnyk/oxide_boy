const LCDC_ENABLED_POS: u8 = 7;
const WINDOW_TMDS_POS: u8 = 6;
const WINDOW_DISPLAY_POS: u8 = 5;
const BG_WINDOW_TDS_POS: u8 = 4;
const BG_TMDS_POS: u8 = 3;
const OBJ_SIZE_POS: u8 = 2;
const OBJ_DISPLAY_POS: u8 = 1;

#[derive(Debug, PartialEq)]
pub struct Lcdc {
    pub lcdc_enabled: bool,
    window_tile_map_display_select: bool, // enum
    window_display: bool,
    bg_window_tile_data_select: bool, // enum
    bg_tile_map_data_select: bool, // enum
    obj_size: ObjSize, // enum
    obj_display: bool,
    pub bg_window_display: bool,
}

#[derive(Debug, PartialEq)]
enum ObjSize {
    S8x8,
    S16x16,
}

impl Default for Lcdc {
    fn default() -> Lcdc {
        Lcdc {
            lcdc_enabled: false,
            window_tile_map_display_select: false,
            window_display: false,
            bg_window_tile_data_select: false,
            bg_tile_map_data_select: false,
            obj_size: ObjSize::S8x8,
            obj_display: false,
            bg_window_display: false,
        }
    }
}

impl std::convert::From<&Lcdc> for u8 {
    fn from(lcdc: &Lcdc) -> u8 {
        (if lcdc.lcdc_enabled { 1 } else { 0 }) << LCDC_ENABLED_POS
           | (if lcdc.window_tile_map_display_select { 1 } else { 0 }) << WINDOW_TMDS_POS
           | (if lcdc.window_display { 1 } else { 0 }) << WINDOW_DISPLAY_POS
           | (if lcdc.bg_window_tile_data_select { 1 } else { 0 }) << BG_WINDOW_TDS_POS
           | (if lcdc.bg_tile_map_data_select { 1 } else { 0 }) << BG_TMDS_POS
           | u8::from(&lcdc.obj_size) << OBJ_SIZE_POS
           | (if lcdc.obj_display { 1 } else { 0 }) << OBJ_DISPLAY_POS
           | (if lcdc.bg_window_display { 1 } else { 0 })
    }
}

impl std::convert::From<&u8> for Lcdc {
    fn from(byte: &u8) -> Lcdc {
        let lcdc_enabled = (byte >> LCDC_ENABLED_POS) & 0b1 == 1; 
        let window_tile_map_display_select = (byte >> WINDOW_TMDS_POS) & 0b1 == 1;
        let window_display = (byte >> WINDOW_DISPLAY_POS) & 0b1 == 1;
        let bg_window_tile_data_select = (byte >> BG_WINDOW_TDS_POS) & 0b1 == 1;
        let bg_tile_map_data_select = (byte >> BG_TMDS_POS) & 0b1 == 1;
        let obj_size = ObjSize::from(byte);
        let obj_display = (byte >> OBJ_DISPLAY_POS) & 0b1 == 1;
        let bg_window_display = byte & 0b1 == 1;
        Lcdc {
            lcdc_enabled,
            window_tile_map_display_select,
            window_display,
            bg_window_tile_data_select,
            bg_tile_map_data_select,
            obj_size,
            obj_display,
            bg_window_display,
        }
    }
}

impl std::convert::From<&ObjSize> for u8 {
    fn from(size: &ObjSize) -> u8 {
        match size {
            ObjSize::S8x8 => 0,
            ObjSize::S16x16 => 4,
        }
    }
}

impl std::convert::From<&u8> for ObjSize {
    fn from(byte: &u8) -> ObjSize {
        match byte & 4 {
            0 => ObjSize::S8x8,
            4 => ObjSize::S16x16,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }
}

#[test]
fn lcdc_to_u8() {
    let lcdc = Lcdc {
        lcdc_enabled: true,
        window_tile_map_display_select: true,
        window_display: false,
        bg_window_tile_data_select: true,
        bg_tile_map_data_select: false,
        obj_size: ObjSize::S8x8,
        obj_display: true,
        bg_window_display: true,
    };
    assert_eq!(u8::from(&lcdc), 0b1101_0011);
}

#[test]
fn u8_to_lcdc() {
    let byte = 0b1101_0011;
    let expected = Lcdc {
        lcdc_enabled: true,
        window_tile_map_display_select: true,
        window_display: false,
        bg_window_tile_data_select: true,
        bg_tile_map_data_select: false,
        obj_size: ObjSize::S8x8,
        obj_display: true,
        bg_window_display: true,
    };
    assert_eq!(Lcdc::from(&byte), expected);
}

#[test]
fn u8_to_objsize() {
    let byte = 0b0000_0100;
    assert_eq!(ObjSize::from(&byte), ObjSize::S16x16);
    let byte = 0b0000_0000;
    assert_eq!(ObjSize::from(&byte), ObjSize::S8x8);
}

#[test]
fn objsize_to_u8() {
    assert_eq!(u8::from(&ObjSize::S16x16), 0b0000_0100);
    assert_eq!(u8::from(&ObjSize::S8x8), 0b0000_0000);
}