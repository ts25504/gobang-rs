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

    pub fn save_archive(&mut self) -> Result<String, Error> {
        let dt = Local::now();
        let filename = dt.format("%Y%m%d-%H%M").to_string() + "-archive.txt";
        let path = Path::new(&filename);
        let mut file = try!(File::create(&path));
        for step in &self.steps {
            try!(file.write_all(step.to_string().as_bytes()));
        }
        Ok(path.to_str().unwrap().to_string())
    }

    pub fn load_archive(&mut self, filename: &str) -> Result<&Vec<Step>, Error> {
        let path = Path::new(filename);
        let file = try!(File::open(&path));

        self.steps.clear();
        let buffer = BufReader::new(file);
        for line in buffer.lines().filter_map(|result| result.ok()) {
            let mut elems = line.split_whitespace();
            let s = try!(elems.next().ok_or(Error::ArchiveParse));
            let color = try!(s.chars().next().ok_or(Error::ArchiveParse));
            let x = try!(elems.next().ok_or(Error::ArchiveParse));
            let y = try!(elems.next().ok_or(Error::ArchiveParse));

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

#[test]
fn test_archive() {
    let mut archive = Archive::new();
    archive.record_step(StoneType::Black, 0, 0);
    archive.record_step(StoneType::White, 1, 1);
    let path = archive.save_archive().unwrap();
    let steps = archive.load_archive(&path).unwrap();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].color, StoneType::Black);
    assert_eq!(steps[0].x, 0);
    assert_eq!(steps[0].y, 0);
    assert_eq!(steps[1].color, StoneType::White);
    assert_eq!(steps[1].x, 1);
    assert_eq!(steps[1].y, 1);
}
