use std::io;
use std::process::exit;
use board::Board;
use manual::Manual;
use common::*;

pub struct Game {
    board: Board,
    manual: Manual,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            manual: Manual::new(),
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
                if u > BOARD_SIZE || u == 0 {
                    println!("row error: overflow");
                    return false
                }
            },
            Err(error) => {
                println!("row error: {}", error);
                return false
            },
        }

        if y.as_bytes()[0] < 65 || y.as_bytes()[0] > (65 + BOARD_SIZE - 1) as u8 {
            println!("column error: wrong column label");
            return false;
        }

        true
    }

    fn input_coordinate(&self) -> (usize, usize) {
        let ux: usize;
        let uy: usize;
        loop {
            let mut input = String::new();

            println!("Input \"quit\" to quit this game");
            println!("Move(Format like 'A19'):");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            if input == "quit".to_string() {
                println!("Quit game");
                exit(0);
            }

            let (first, second) = input.split_at(1);
            let y = first.to_string();
            let x = second.to_string();


            if self.is_coordinate_str_legal(&x, &y) {
                ux = BOARD_SIZE - x.trim().parse::<usize>().unwrap();
                uy = (y.as_bytes()[0] - 65) as usize;
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
