use std::process::Command;
use common::*;
use archive::Step;

fn clear_screen() {
    let mut child = Command::new("clear").spawn().unwrap();
    child.wait().unwrap();
}

pub struct Board {
    points: Vec<Vec<StoneType>>,
}

impl Board {
    pub fn new() -> Board {
        let v = vec![vec![StoneType::None;BOARD_SIZE];BOARD_SIZE];
        Board { points: v }
    }

    pub fn print(&self) {
        clear_screen();
        println!("Welcome to Gobang game!");
        print!("   ");
        for i in 0..BOARD_SIZE {
            print!("{:>2} ", ((i+65) as u8) as char);
        }
        println!();

        for i in 0..BOARD_SIZE {
            print!("{:02} ", BOARD_SIZE - i);
            for j in 0..BOARD_SIZE {
                print!(" {} ", self.points[i][j].as_char());
            }
            println!();
        }
    }

    pub fn move_black(&mut self, x: usize, y: usize) {
        self.points[x][y] = StoneType::Black;
    }

    pub fn move_white(&mut self, x: usize, y: usize) {
        self.points[x][y] = StoneType::White;
    }

    fn clear(&mut self) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                self.points[i][j] = StoneType::None;
            }
        }
    }

    pub fn load_archive(&mut self, steps: &[Step]) {
        self.clear();
        for step in steps {
            match step.color {
                StoneType::Black => self.move_black(step.x, step.y),
                StoneType::White => self.move_white(step.x, step.y),
                StoneType::None => println!("Wrong step"),
            }
        }
        self.print();
        println!("Load success!");
    }

    pub fn win(&self, x: usize, y: usize) -> bool {
        self.win_horizontal(x, y)
            || self.win_vertical(x, y)
            || self.win_diagonal_a(x, y)
            || self.win_diagonal_b(x, y)
    }

    fn win_horizontal(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: StoneType = self.points[x][y];
        let mut inc: usize = 0;

        let mut east: bool = true;
        let mut west: bool = true;
        while east || west {
            inc += 1;

            if y >= inc && self.points[x][y-inc] == color {
                serial_count += 1;
            } else {
                east = false;
            }

            if y + inc < BOARD_SIZE && self.points[x][y+inc] == color {
                serial_count += 1;
            } else {
                west = false;
            }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    fn win_vertical(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: StoneType = self.points[x][y];
        let mut inc: usize = 0;

        let mut north: bool = true;
        let mut south: bool = true;
        while north || south {
            inc += 1;

            if x >= inc && self.points[x-inc][y] == color {
                serial_count += 1;
            } else {
                north = false;
            }

            if x + inc < BOARD_SIZE && self.points[x+inc][y] == color {
                serial_count += 1;
            } else {
                south = false;
            }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    fn win_diagonal_a(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: StoneType = self.points[x][y];
        let mut inc: usize = 0;

        let mut northeast: bool = true;
        let mut southwest: bool = true;
        while northeast || southwest {
            inc += 1;

            if x >= inc && y + inc < BOARD_SIZE &&
                self.points[x-inc][y+inc] == color {
                    serial_count += 1;
                } else {
                    northeast = false;
                }

            if y >= inc && x + inc < BOARD_SIZE &&
                self.points[x+inc][y-inc] == color {
                    serial_count += 1;
                } else {
                    southwest = false;
                }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    fn win_diagonal_b(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: StoneType = self.points[x][y];
        let mut inc: usize = 0;

        let mut northwest: bool = true;
        let mut southeast: bool = true;
        while northwest || southeast {
            inc += 1;

            if x >= inc && y >= inc && self.points[x-inc][y-inc] == color {
                serial_count += 1;
            } else {
                northwest = false;
            }

            if x + inc < BOARD_SIZE && y + inc < BOARD_SIZE &&
                self.points[x+inc][y+inc] == color {
                    serial_count += 1;
                } else {
                    southeast = false;
                }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    pub fn has_stone(&self, x: usize, y: usize) -> bool {
        self.points[x][y] != StoneType::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_horizontal() {
        let mut board = Board::new();
        board.move_black(0, 0);
        board.move_black(0, 1);
        board.move_black(0, 2);
        board.move_black(0, 3);
        board.move_black(0, 4);
        assert!(board.win_horizontal(0, 4));
    }

    #[test]
    fn test_win_vertical() {
        let mut board = Board::new();
        board.move_black(0, 0);
        board.move_black(1, 0);
        board.move_black(2, 0);
        board.move_black(3, 0);
        board.move_black(4, 0);
        assert!(board.win_vertical(4, 0));
    }

    #[test]
    fn test_win_digonal_a() {
        let mut board = Board::new();
        board.move_black(0, 4);
        board.move_black(1, 3);
        board.move_black(2, 2);
        board.move_black(3, 1);
        board.move_black(4, 0);
        assert!(board.win_diagonal_a(4, 0));
    }

    #[test]
    fn test_win_digonal_b() {
        let mut board = Board::new();
        board.move_black(0, 0);
        board.move_black(1, 1);
        board.move_black(2, 2);
        board.move_black(3, 3);
        board.move_black(4, 4);
        assert!(board.win_diagonal_b(4, 4));
    }

    #[test]
    fn test_move() {
        let mut board = Board::new();
        board.move_black(0, 0);
        assert!(!board.has_stone(1, 1));
        assert!(board.has_stone(0, 0));
        board.move_white(1, 1);
        assert!(board.has_stone(1, 1));
        assert!(board.has_stone(0, 0));
    }
}
