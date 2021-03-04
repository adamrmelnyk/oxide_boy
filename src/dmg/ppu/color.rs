#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

impl Color {
    pub fn rgb(self) -> u8 {
        match self {
            Color::White => 255,
            Color::LightGrey => 0xCC,
            Color::DarkGrey => 0x77,
            Color::Black => 0x00,
        }
    }
}

impl std::convert::From<u8> for Color {
    fn from(byte: u8) -> Color {
        match byte {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            3 => Color::Black,
            _ => panic!("This should never happen!")
        }
    }
}