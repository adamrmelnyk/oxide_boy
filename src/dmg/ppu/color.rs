#[derive(Debug, PartialEq)]
enum Color {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

impl Color {
    fn rgb(self) -> u8 {
        match self {
            Color::White => 255,
            Color::LightGrey => 0xCC,
            Color::DarkGrey => 0x77,
            Color::Black => 0x00,
        }
    }
}