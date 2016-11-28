use board::Board;
use chessmanual::ChessManual;
use common::*;

pub struct Game {
    board: Board,
    manual: ChessManual,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            manual: ChessManual::new(),
        }
    }

    pub fn start(&mut self) {
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
            self.manual.record_step(BLACK_STONE, bx, by);

            if self.board.win(bx, by) {
                self.manual.write_manual(BLACK_STONE);
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
            self.manual.record_step(WHITE_STONE, wx, wy);

            if self.board.win(wx, wy) {
                self.manual.write_manual(WHITE_STONE);
                println!("White win!");
                break;
            }
        }
    }

    fn is_coordinate_str_legal(&self, x: &String, y: &String) -> bool {
        match x.trim().parse::<usize>() {
            Ok(u) => {
                if u >= BOARD_SIZE {
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
                if u >= BOARD_SIZE {
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
