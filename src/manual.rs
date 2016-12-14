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

    pub fn write_manual(&self, winner: StoneType) -> Result<String, Error> {
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
        Ok(path.to_str().unwrap().to_string())
    }

    pub fn load_archive(&mut self, steps: &Vec<archive::Step>) {
        self.steps.clear();
        for step in steps {
            self.steps.push(Step{ color: step.color, x: step.x, y: step.y });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let step1 = Step{ color: StoneType::Black, x: 0, y: 0};
        assert_eq!(step1.to_string(), "Black: A15\n".to_string());
        let step2 = Step{ color: StoneType::White, x: 1, y: 1};
        assert_eq!(step2.to_string(), "White: B14\n".to_string());
        let step3 = Step{ color: StoneType::None, x: 2, y: 2};
        assert_eq!(step3.to_string(), "Error: C13\n".to_string());
    }

    #[test]
    fn test_record_step() {
        let mut manual = Manual::new();
        manual.record_step(StoneType::Black, 0, 0);
        manual.record_step(StoneType::White, 1, 1);
        assert_eq!(manual.steps[0].color, StoneType::Black);
        assert_eq!(manual.steps[0].x, 0);
        assert_eq!(manual.steps[0].y, 0);

        assert_eq!(manual.steps[1].color, StoneType::White);
        assert_eq!(manual.steps[1].x, 1);
        assert_eq!(manual.steps[1].y, 1);
    }

    #[test]
    fn test_load_archive() {
        let mut manual = Manual::new();
        let mut archive = archive::Archive::new();
        archive.record_step(StoneType::Black, 0, 0);
        archive.record_step(StoneType::White, 1, 1);
        manual.load_archive(&archive.get_steps());

        assert_eq!(manual.steps.len(), 2);
        assert_eq!(manual.steps[0].color, StoneType::Black);
        assert_eq!(manual.steps[0].x, 0);
        assert_eq!(manual.steps[0].y, 0);
        assert_eq!(manual.steps[1].color, StoneType::White);
        assert_eq!(manual.steps[1].x, 1);
        assert_eq!(manual.steps[1].y, 1);
    }
}
