const OBJ_TO_BG_PRIORITY_POS: u8 = 7;
const Y_FLIP_POS: u8 = 6;
const X_FLIP_POS: u8 = 5;
const PALETTE_NUMBER_POS: u8 = 4;
const CGB_TILE_VRAM_BLANK_POS: u8 = 3;

pub struct OamEntry {
    pub y_pos: i16,
    pub x_pos: i16,
    pub tile_location: u8,
    pub attributes: OamEntryFlag,
}

#[derive(Debug, PartialEq)]
pub enum Palette {
    Obp0,
    Obp1,
}

impl std::convert::From<&Palette> for u16 {
    fn from(palette: &Palette) -> u16 {
        match palette {
            Palette::Obp0 => 0xFF48,
            Palette::Obp1 => 0xFF49,
        }
    }
}

impl OamEntry {
    pub fn new(oam: [u8; 160], sprite: usize) -> OamEntry {
        let index: usize = sprite * 4;
        OamEntry {
            y_pos: oam[index] as i16 - 16,
            x_pos: oam[index + 1] as i16 - 8,
            tile_location: oam[index + 2],
            attributes: OamEntryFlag::from(oam[index + 3]),
        }
    }
}

pub struct OamEntryFlag {
    pub obj_to_bg_priority: bool,
    pub y_flip: bool, // vertically mirrored
    pub x_flip: bool, // horizontally mirrored
    pub palette_number: Palette,

    /// These Flags are only required for use with
    /// the Color Gameboy
    _cgb_tile_vram_bank: bool,
    _cgb_palette_number: u8,
}

impl std::convert::From<u8> for OamEntryFlag {
    fn from(byte: u8) -> OamEntryFlag {
        let obj_to_bg_priority = (byte >> OBJ_TO_BG_PRIORITY_POS) & 0b1 != 0;
        let y_flip = (byte >> Y_FLIP_POS) & 0b1 != 0;
        let x_flip = (byte >> X_FLIP_POS) & 0b1 != 0;
        let palette_number = Palette::from(byte);
        let _cgb_tile_vram_bank = (byte >> CGB_TILE_VRAM_BLANK_POS) & 0b1 != 0; // Required for CGB only
        let _cgb_palette_number = 0; // Required for CGB only
        OamEntryFlag {
            obj_to_bg_priority,
            y_flip,
            x_flip,
            palette_number,
            _cgb_tile_vram_bank,
            _cgb_palette_number,
        }
    }
}

impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Palette {
        match (byte >> PALETTE_NUMBER_POS) & 0b1 {
            0 => Palette::Obp0,
            1 => Palette::Obp1,
            _ => panic!("We've defied a law of mathematics!!"),
        }
    }
}

#[test]
fn convert_4_bytes_to_oam_entry() {
    let mut oam = [0u8; 160];
    oam[0] = 0x10;
    oam[1] = 0x20;
    oam[2] = 0x30;
    oam[3] = 0b1101_0000;
    let res = OamEntry::new(oam, 0);
    assert_eq!(res.y_pos, 0x10 - 16);
    assert_eq!(res.x_pos, 0x20 - 8);
    assert_eq!(res.tile_location, 0x30);
    assert_eq!(res.attributes.obj_to_bg_priority, true);
    assert_eq!(res.attributes.y_flip, true);
    assert_eq!(res.attributes.x_flip, false);
    assert_eq!(res.attributes.palette_number, Palette::Obp1);

    // Check the rest of the attributes for CGB
}

#[test]
fn convert_byte_to_oamentryflag() {
    let byte = 0b1011_0000;
    let res = OamEntryFlag::from(byte);
    assert_eq!(res.obj_to_bg_priority, true);
    assert_eq!(res.y_flip, false);
    assert_eq!(res.x_flip, true);
    assert_eq!(res.palette_number, Palette::Obp1);
}

#[test]
fn convert_byte_to_palette_number() {
    let byte = 0b0001_0000;
    assert_eq!(Palette::from(byte), Palette::Obp1);
    let byte = 0;
    assert_eq!(Palette::from(byte), Palette::Obp0);
}
