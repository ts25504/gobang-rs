use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use chrono::*;
use archive;
use common::*;
use errors::*;

struct Step {
    color: StoneType,
    x: usize,
    y: usize,
}

impl Step {
    fn to_string(&self) -> String {
        let color: String =
            match self.color {
                StoneType::Black => String::from("Black"),
                StoneType::White => String::from("White"),
                StoneType::None => String::from("Error"),
            };

        let row_label = BOARD_SIZE - self.x;
        let col_label = ((self.y + 65) as u8) as char;
        format!("{}: {}{}\n", color, col_label, row_label)
    }
}

pub struct Manual {
    steps: Vec<Step>,
}

impl Manual {
    pub fn new() -> Manual {
        Manual {
            steps: Vec::new(),
        }
    }

    pub fn record_step(&mut self, color: StoneType, x: usize, y: usize) {
        self.steps.push(Step{ color: color, x: x, y: y });
    }

    pub fn write_manual(&self, winner: StoneType) -> Result<(), Error> {
        let dt = Local::now();
        let filename = dt.format("%Y%m%d-%H%M").to_string() + "-game.txt";
        let path = Path::new(&filename);
        let mut file = try!(File::create(&path));
        for step in &self.steps {
            try!(file.write_all(step.to_string().as_bytes()));
        }
        let win =
            match winner {
                StoneType::Black => String::from("Black Win!"),
                StoneType::White => String::from("White Win!"),
                StoneType::None => String::from("No winner"),
            };

        try!(file.write_all(win.as_bytes()));
    }

    pub fn load_archive(&mut self, steps: &Vec<archive::Step>) {
        self.steps.clear();
        for step in steps {
            self.steps.push(Step{ color: step.color, x: step.x, y: step.y });
        }
    }
}
