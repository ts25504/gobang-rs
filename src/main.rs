use std::process::Command;

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
        let mut child = Command::new("clear").spawn().unwrap();
        child.wait().unwrap();

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
}

struct User {
    board: Board,
}

impl User {
    fn new() -> User {
        User {
            board: Board::new(),
        }
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
}
