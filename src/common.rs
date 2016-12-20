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
        match *self {
            StoneType::None => '.',
            StoneType::Black => 'b',
            StoneType::White => 'w',
        }
    }
}

pub fn char_to_stonetype(c: char) -> Option<StoneType> {
    match c {
        '.' => Some(StoneType::None),
        'b' => Some(StoneType::Black),
        'w' => Some(StoneType::White),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_char() {
        assert_eq!(StoneType::None.as_char(), '.');
        assert_eq!(StoneType::Black.as_char(), 'b');
        assert_eq!(StoneType::White.as_char(), 'w');
    }

    #[test]
    fn test_char_to_stonetype() {
        assert_eq!(char_to_stonetype('.'), Some(StoneType::None));
        assert_eq!(char_to_stonetype('b'), Some(StoneType::Black));
        assert_eq!(char_to_stonetype('w'), Some(StoneType::White));
        assert_eq!(char_to_stonetype('a'), None);
    }
}
