#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

impl Color {
    pub fn rgb(self) -> u32 {
        match self {
            Color::White => 0x9bbc0f,
            Color::LightGrey => 0x8bac0f,
            Color::DarkGrey => 0x306230,
            Color::Black => 0x0f380f,
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
            _ => panic!("This should never happen!"),
        }
    }
}
