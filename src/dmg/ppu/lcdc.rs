const LCDC_ENABLED_POS: u8 = 7;
const WINDOW_DISPLAY_POS: u8 = 5;
const OBJ_DISPLAY_POS: u8 = 1;

#[derive(Debug, PartialEq)]
pub struct Lcdc {
    lcdc_enabled: bool,
    window_tile_map_display_select: TileMap,
    window_display: bool,
    bg_window_tile_data_select: TileData,
    bg_tile_map_data_select: TileMap,
    obj_size: ObjSize,
    obj_display: bool,
    bg_window_display: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjSize {
    S8x8,
    S8x16,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileMap {
    S9800, // 9800 - 9BFF
    S9C00, // 9C00 - 9FFF
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileData {
    S8800, // 8800 - 97FF
    S8000, // 8000 - 8FFF
}

impl Default for Lcdc {
    fn default() -> Lcdc {
        Lcdc {
            lcdc_enabled: false,
            window_tile_map_display_select: TileMap::S9800,
            window_display: false,
            bg_window_tile_data_select: TileData::S8000,
            bg_tile_map_data_select: TileMap::S9800,
            obj_size: ObjSize::S8x8,
            obj_display: false,
            bg_window_display: false,
        }
    }
}

impl std::convert::From<&Lcdc> for u8 {
    fn from(lcdc: &Lcdc) -> u8 {
        (if lcdc.lcdc_enabled { 1 } else { 0 }) << LCDC_ENABLED_POS
            | TileMap::u8_from_window(&lcdc.window_tile_map_display_select)
            | (if lcdc.window_display { 1 } else { 0 }) << WINDOW_DISPLAY_POS
            | u8::from(&lcdc.bg_window_tile_data_select)
            | TileMap::u8_from_bg(&lcdc.bg_tile_map_data_select)
            | u8::from(&lcdc.obj_size)
            | (if lcdc.obj_display { 1 } else { 0 }) << OBJ_DISPLAY_POS
            | (if lcdc.bg_window_display { 1 } else { 0 })
    }
}

impl std::convert::From<&u8> for Lcdc {
    fn from(byte: &u8) -> Lcdc {
        let lcdc_enabled = (byte >> LCDC_ENABLED_POS) & 0b1 == 1;
        let window_tile_map_display_select = TileMap::from_window(byte);
        let window_display = (byte >> WINDOW_DISPLAY_POS) & 0b1 == 1;
        let bg_window_tile_data_select = TileData::from(byte);
        let bg_tile_map_data_select = TileMap::from_bg(byte);
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
            ObjSize::S8x16 => 4,
        }
    }
}

impl std::convert::From<&u8> for ObjSize {
    fn from(byte: &u8) -> ObjSize {
        match byte & 4 {
            0 => ObjSize::S8x8,
            4 => ObjSize::S8x16,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }
}

impl std::convert::From<&TileData> for u8 {
    fn from(data: &TileData) -> u8 {
        match data {
            TileData::S8000 => 0,
            TileData::S8800 => 16,
        }
    }
}

impl std::convert::From<&u8> for TileData {
    fn from(byte: &u8) -> TileData {
        match byte & 16 {
            0 => TileData::S8000,
            16 => TileData::S8800,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }
}

impl ObjSize {
    pub fn vertical_size(&self) -> i16 {
        match self {
            ObjSize::S8x16 => 16,
            ObjSize::S8x8 => 8,
        }
    }
}

impl TileMap {
    fn from_window(byte: &u8) -> TileMap {
        match byte & 64 {
            0 => TileMap::S9800,
            64 => TileMap::S9C00,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }

    fn from_bg(byte: &u8) -> TileMap {
        match byte & 8 {
            0 => TileMap::S9800,
            8 => TileMap::S9C00,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }

    fn u8_from_window(tile: &TileMap) -> u8 {
        match tile {
            TileMap::S9800 => 0,
            TileMap::S9C00 => 64,
        }
    }

    fn u8_from_bg(tile: &TileMap) -> u8 {
        match tile {
            TileMap::S9800 => 0,
            TileMap::S9C00 => 8,
        }
    }

    pub fn address(&self) -> u16 {
        match self {
            TileMap::S9800 => 0x9800,
            TileMap::S9C00 => 0x9C00,
        }
    }
}

impl TileData {
    pub fn address(&self) -> u16 {
        match self {
            TileData::S8000 => 0x8000,
            TileData::S8800 => 0x8800,
        }
    }
}

impl Lcdc {
    pub fn obj_size(&self) -> ObjSize {
        self.obj_size
    }

    pub fn obj_display(&self) -> bool {
        self.obj_display
    }

    pub fn lcdc_enabled(&self) -> bool {
        self.lcdc_enabled
    }

    pub fn bg_window_display(&self) -> bool {
        self.bg_window_display
    }

    pub fn window_display(&self) -> bool {
        self.window_display
    }

    pub fn window_tile_map_display_select(&self) -> TileMap {
        self.window_tile_map_display_select
    }

    pub fn bg_window_tile_data_select(&self) -> TileData {
        self.bg_window_tile_data_select
    }

    pub fn bg_tile_map_data_select(&self) -> TileMap {
        self.bg_tile_map_data_select
    }
}

#[test]
fn lcdc_to_u8() {
    let lcdc = Lcdc {
        lcdc_enabled: true,
        window_tile_map_display_select: TileMap::S9C00,
        window_display: false,
        bg_window_tile_data_select: TileData::S8800,
        bg_tile_map_data_select: TileMap::S9800,
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
        window_tile_map_display_select: TileMap::S9C00,
        window_display: false,
        bg_window_tile_data_select: TileData::S8800,
        bg_tile_map_data_select: TileMap::S9800,
        obj_size: ObjSize::S8x8,
        obj_display: true,
        bg_window_display: true,
    };
    assert_eq!(Lcdc::from(&byte), expected);
}

#[test]
fn u8_to_objsize() {
    let byte = 0b0000_0100;
    assert_eq!(ObjSize::from(&byte), ObjSize::S8x16);
    let byte = 0b0000_0000;
    assert_eq!(ObjSize::from(&byte), ObjSize::S8x8);
}

#[test]
fn objsize_to_u8() {
    assert_eq!(u8::from(&ObjSize::S8x16), 0b0000_0100);
    assert_eq!(u8::from(&ObjSize::S8x8), 0b0000_0000);
}

#[test]
fn u8_to_tile_map() {
    let byte = 0b0100_0000;
    assert_eq!(TileMap::from_window(&byte), TileMap::S9C00);
    let byte = 0b0000_0000;
    assert_eq!(TileMap::from_window(&byte), TileMap::S9800);
}

#[test]
fn tile_map_to_u8() {
    assert_eq!(TileMap::u8_from_window(&TileMap::S9800), 0b0000_0000);
    assert_eq!(TileMap::u8_from_window(&TileMap::S9C00), 0b0100_0000);
}

#[test]
fn u8_to_tile_data() {
    let byte = 0b0000_0000;
    assert_eq!(TileData::from(&byte), TileData::S8000);
    let byte = 0b0001_0000;
    assert_eq!(TileData::from(&byte), TileData::S8800);
}

#[test]
fn tile_data_to_u8() {
    assert_eq!(u8::from(&TileData::S8000), 0b0000_0000);
    assert_eq!(u8::from(&TileData::S8800), 0b0001_0000);
}
