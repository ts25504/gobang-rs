use std::io;
use std::process::exit;
use board::Board;
use manual::Manual;
use archive::Archive;
use common::*;

pub struct Game {
    board: Board,
    manual: Manual,
    archive: Archive,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            manual: Manual::new(),
            archive: Archive::new(),
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
            self.manual.record_step(StoneType::Black, bx, by);
            self.archive.record_step(StoneType::Black, bx, by);

            if self.board.win(bx, by) {
                println!("Black win!");
                match self.manual.write_manual(StoneType::Black) {
                    Ok(_) => println!("Save manual success"),
                    Err(err) => println!("Save manual fail: err"),
                }
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
            self.manual.record_step(StoneType::White, wx, wy);
            self.archive.record_step(StoneType::White, wx, wy);

            if self.board.win(wx, wy) {
                println!("White win!");
                match self.manual.write_manual(StoneType::White) {
                    Ok(_) => println!("Save manual success"),
                    Err(err) => println!("Save manual fail: err"),
                }
                break;
            }
        }
    }

    fn is_coordinate_str_legal(&self, x: &str, y: &str) -> bool {
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

    fn input_coordinate(&mut self) -> (usize, usize) {
        let ux: usize;
        let uy: usize;
        loop {
            let mut input = String::new();

            println!("\nInput \"quit\" to quit this game");
            println!("Input \"save\" to save a game");
            println!("Input \"load 'filepath'\" to load a game");
            println!("\nMove(Format like 'A15'):");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if input == "quit\n".to_string() {
                println!("Quit game");
                exit(0);
            }

            if input == "save\n".to_string() {
                match self.archive.save_archive() {
                    Ok(_) => println!("Save game success"),
                    Err(err) => println!("Save game fail: {}", err),
                };
                continue;
            }

            if input.contains("load") {
                let split = input.split_whitespace();
                let filename = split.last().unwrap();
                let steps = match self.archive.load_archive(filename) {
                    Ok(steps) => {
                        println!("Load game {} success", filename);
                        steps
                    },
                    Err(err) => {
                        println!("Load game {} fail: {}", filename, err);
                        continue;
                    },
                };
                self.board.load_archive(steps);
                self.manual.load_archive(steps);
            }

            let (first, second) = input.split_at(1);

            if self.is_coordinate_str_legal(second, first) {
                ux = BOARD_SIZE - second.trim().parse::<usize>().unwrap();
                uy = (first.as_bytes()[0] - 65) as usize;
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
