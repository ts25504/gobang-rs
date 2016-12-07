pub const BOARD_SIZE: usize = 15;
pub const WIN_SERIAL_COUNT: i32 = 5;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StoneType {
    None,
    Black,
    White,
}

impl StoneType {
    pub fn as_char(&self) -> char {
        match self {
            &StoneType::None => '.',
            &StoneType::Black => 'b',
            &StoneType::White => 'w',
        }
    }
}

pub fn char_to_stonetype(c: char) -> StoneType {
    match c {
        '.' => StoneType::None,
        'b' => StoneType::Black,
        'w' => StoneType::White,
        _ => StoneType::None,
    }
}
