use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use chrono::*;
use common::*;
use errors::Error;

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

    pub fn save_archive(&mut self) -> Result<(), Error> {
        let dt = Local::now();
        let filename = dt.format("%Y%m%d-%H%M").to_string() + "-archive.txt";
        let path = Path::new(&filename);
        let mut file = try!(File::create(&path));
        for step in &self.steps {
            try!(file.write_all(step.to_string().as_bytes()));
        }
        Ok(())
    }

    pub fn load_archive(&mut self, filename: &str) -> Result<&Vec<Step>, Error> {
        let path = Path::new(filename);
        let file = try!(File::open(&path));

        self.steps.clear();
        let buffer = BufReader::new(file);
        for line in buffer.lines().filter_map(|result| result.ok()) {
            let mut elems = line.split_whitespace();
            let s = try!(elems.next().ok_or(Error::ArchiveParseError));
            let color = try!(s.chars().next().ok_or(Error::ArchiveParseError));
            let x = try!(elems.next().ok_or(Error::ArchiveParseError));
            let y = try!(elems.next().ok_or(Error::ArchiveParseError));

            let step = Step {
                color: char_to_stonetype(color),
                x: try!(x.trim().parse::<usize>()),
                y: try!(y.trim().parse::<usize>()),
            };
            self.steps.push(step);
        }

        Ok(&self.steps)
    }
}
