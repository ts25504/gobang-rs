use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use chrono::*;
use common::*;

struct Step {
    color: char,
    x: usize,
    y: usize,
}

impl Step {
    fn to_string(&self) -> String {
        let color: String =
            if self.color == BLACK_STONE {
                String::from("Black")
            } else {
                String::from("White")
            };

        format!("{}: ({}, {})\n", color, self.x, self.y)
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

    pub fn record_step(&mut self, color: char, x: usize, y: usize) {
        self.steps.push(Step{ color: color, x: x, y: y });
    }

    pub fn write_manual(&self, winner: char) {
        let dt = Local::now();
        let filename = dt.format("%Y%m%d-%H%M").to_string() + "-game.txt";
        let path = Path::new(&filename);
        let mut file = File::create(&path).unwrap();
        for step in &self.steps {
            file.write_all(step.to_string().as_bytes()).unwrap();
        }
        let win =
            if winner == BLACK_STONE {
                String::from("Black Win!")
            } else {
                String::from("White Win!")
            };

        file.write_all(win.as_bytes()).unwrap();
    }
}
