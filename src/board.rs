use std::process::Command;
use common::*;
use archive::Step;

fn clear_screen() {
    let mut child = Command::new("clear").spawn().unwrap();
    child.wait().unwrap();
}

pub struct Board {
    points: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board { points: Vec::new() };
        for i in 0..BOARD_SIZE {
            board.points.push(Vec::new());
            for _ in 0..BOARD_SIZE {
                board.points[i].push(POINT);
            }
        }
        board
    }

    pub fn print(&self) {
        clear_screen();
        println!("Welcome to Gobang game!");
        print!("   ");
        for i in 0..BOARD_SIZE {
            print!("{:>2} ", ((i+65) as u8) as char);
        }
        print!("\n");

        for i in 0..BOARD_SIZE {
            print!("{:02} ", BOARD_SIZE - i);
            for j in 0..BOARD_SIZE {
                print!(" {} ", self.points[i][j]);
            }
            print!("\n");
        }
    }

    pub fn move_black(&mut self, x: usize, y: usize) {
        self.points[x][y] = BLACK_STONE;
    }

    pub fn move_white(&mut self, x: usize, y: usize) {
        self.points[x][y] = WHITE_STONE;
    }

    fn clear(&mut self) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                self.points[i][j] = POINT;
            }
        }
    }

    pub fn load_archive(&mut self, steps: &Vec<Step>) {
        self.clear();
        for step in steps {
            if step.color == 'b' {
                self.move_black(step.x, step.y);
            } else if step.color == 'w' {
                self.move_white(step.x, step.y);
            }
        }
        self.print();
        println!("Load successl!");
    }

    pub fn win(&self, x: usize, y: usize) -> bool {
        self.win_horizontal(x, y) || self.win_vertical(x, y) || self.win_diagonal_a(x, y) || self.win_diagonal_b(x, y)
    }

    pub fn win_horizontal(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
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
        let color: char = self.points[x][y];
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
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut northeast: bool = true;
        let mut southwest: bool = true;
        while northeast || southwest {
            inc += 1;

            if x >= inc && y + inc < BOARD_SIZE && self.points[x-inc][y+inc] == color {
                serial_count += 1;
            } else {
                northeast = false;
            }

            if y >= inc && x + inc < BOARD_SIZE && self.points[x+inc][y-inc] == color {
                serial_count += 1;
            } else {
                southwest = false;
            }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    fn win_diagonal_b(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
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

            if x + inc < BOARD_SIZE && y + inc < BOARD_SIZE && self.points[x+inc][y+inc] == color {
                serial_count += 1;
            } else {
                southeast = false;
            }
        }

        serial_count == WIN_SERIAL_COUNT
    }

    pub fn has_stone(&self, x: usize, y: usize)  -> bool {
        self.points[x][y] != POINT
    }
}
