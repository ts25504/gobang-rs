use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use chrono::*;
use common::*;

pub struct Step {
    pub color: StoneType,
    pub x: usize,
    pub y: usize,
}

impl Step {
    fn to_string(&self) -> String {
        format!("{} {} {}\n", self.color.as_char(), self.x, self.y)
    }
}

pub struct Archive {
    steps: Vec<Step>,
}

impl Archive {
    pub fn new() -> Archive {
        Archive {
            steps: Vec::new(),
        }
    }

    pub fn record_step(&mut self, color: StoneType, x: usize, y: usize) {
        self.steps.push(Step{ color: color, x: x, y: y});
    }

    pub fn save_archive(&mut self) {
        let dt = Local::now();
        let filename = dt.format("%Y%m%d-%H%M").to_string() + "-archive.txt";
        let path = Path::new(&filename);
        let mut file = File::create(&path).unwrap();
        for step in &self.steps {
            file.write_all(step.to_string().as_bytes()).unwrap();
        }
    }

    pub fn load_archive(&mut self, filename: String) -> &Vec<Step> {
        let path = Path::new(&filename);
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}",
                               path.display(),
                               why.description()),
            Ok(file) => file,
        };

        self.steps.clear();
        let buffer = BufReader::new(file);
        for line in buffer.lines().filter_map(|result| result.ok()) {
            let mut elems = line.split_whitespace();
            let step = Step {
                color: char_to_stonetype(
                           elems.next().unwrap().chars().next().unwrap()),
                x: elems.next().unwrap().trim().parse::<usize>().unwrap(),
                y: elems.next().unwrap().trim().parse::<usize>().unwrap()
            };
            self.steps.push(step);
        }

        &self.steps
    }
}
