#[macro_use]
extern crate text_io;

use std::process::Command;

fn clear_screen() {
    let mut child = Command::new("clear").spawn().unwrap();
    child.wait().unwrap();
}

struct Board {
    points: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Board {
        let mut board = Board { points: Vec::new() };
        for i in 0..16 {
            board.points.push(Vec::new());
            for _ in 0..16 {
                board.points[i].push('*');
            }
        }
        board
    }

    fn print(&self) {
        clear_screen();
        println!("Welcome to Gobang game!");
        for i in 0..16 {
            for j in 0..16 {
                print!("{} ", self.points[i][j]);
            }
            print!("\n");
        }
    }

    fn move_black(&mut self, x: usize, y: usize) {
        self.points[x][y] = 'b';
    }

    fn move_white(&mut self, x: usize, y: usize) {
        self.points[x][y] = 'w';
    }

    fn win(&self, x: usize, y: usize) -> bool {
        self.win_horizontal(x, y) || self.win_vertical(x, y) || self.win_diagonal_a(x, y) || self.win_diagonal_b(x, y)
    }

    fn win_horizontal(&self, x: usize, y: usize) -> bool {
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

            if y + inc < 16 && self.points[x][y+inc] == color {
                serial_count += 1;
            } else {
                west = false;
            }
        }

        serial_count == 5
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

            if x + inc < 16 && self.points[x+inc][y] == color {
                serial_count += 1;
            } else {
                south = false;
            }
        }

        serial_count == 5
    }

    fn win_diagonal_a(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut northeast: bool = true;
        let mut southwest: bool = true;
        while northeast || southwest {
            inc += 1;

            if x >= inc && y + inc < 16 && self.points[x-inc][y+inc] == color {
                serial_count += 1;
            } else {
                northeast = false;
            }

            if y >= inc && x + inc < 16 && self.points[x+inc][y-inc] == color {
                serial_count += 1;
            } else {
                southwest = false;
            }
        }

        serial_count == 5
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

            if x + inc < 16 && y + inc < 16 && self.points[x+inc][y+inc] == color {
                serial_count += 1;
            } else {
                southeast = false;
            }
        }

        serial_count == 5
    }

    fn has_stone(&self, x: usize, y: usize)  -> bool {
        self.points[x][y] != '*'
    }
}

struct Game {
    board: Board,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    fn start(&mut self) {
        self.board.print();
        loop {
            let (mut bx, mut by) = self.input_coordinate();
            while self.board.has_stone(bx, by) {
                println!("error: this point already has a stone.");
                let (x, y) = self.input_coordinate();
                bx = x;
                by = y;
            }
            self.move_black(bx, by);

            if self.board.win(bx, by) {
                println!("Black win!");
                break;
            }

            let (mut wx, mut wy) = self.input_coordinate();
            while self.board.has_stone(wx, wy) {
                println!("error: this point already has a stone.");
                let (x, y) = self.input_coordinate();
                wx = x;
                wy = y;
            }
            self.move_white(wx, wy);

            if self.board.win(wx, wy) {
                println!("White win!");
                break;
            }
        }
    }

    fn is_coordinate_str_legal(&self, x: &String, y: &String) -> bool {
        match x.trim().parse::<usize>() {
            Ok(u) => {
                if u >= 16 {
                    println!("error: overflow");
                    return false
                }
            },
            Err(error) => {
                println!("error: {}", error);
                return false
            },
        }

        match y.trim().parse::<usize>() {
            Ok(u) => {
                if u >= 16 {
                    println!("error: overflow");
                    return false
                }
            },
            Err(error) => {
                println!("error: {}", error);
                return false
            },
        }

        true
    }

    fn input_coordinate(&self) -> (usize, usize) {
        let mut ux: usize = 0;
        let mut uy: usize = 0;
        loop {
            let mut x = String::new();
            let mut y = String::new();

            println!("Move(Format: x y):");
            scan!("{} {}", x, y);
            if self.is_coordinate_str_legal(&x, &y) {
                ux = x.trim().parse::<usize>().unwrap();
                uy = y.trim().parse::<usize>().unwrap();
                break;
            }
        }

        (ux, uy)
    }

    fn move_black(&mut self, x: usize, y: usize) {
        self.board.move_black(x, y);
        self.board.print();
    }

    fn move_white(&mut self, x: usize, y: usize) {
        self.board.move_white(x, y);
        self.board.print();
    }
}

fn main() {
    let mut game = Game::new();
    game.start();
}
