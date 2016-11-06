use std::process::Command;
use std::io;

fn clear_screen() {
    let mut child = Command::new("clear").spawn().unwrap();
    child.wait().unwrap();
}

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
        clear_screen();
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

struct Game {
    board: Board,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    fn start(&mut self) {
        clear_screen();
        println!("Welcome to Gobang game!");
        loop {
            let (bx, by) = self.input_coordinate();
            self.move_black(bx, by);

            let (wx, wy) = self.input_coordinate();
            self.move_white(wx, wy);
        }
    }

    fn input_coordinate(&self) -> (usize, usize) {
        let mut x = String::new();
        println!("Move x");
        io::stdin().read_line(&mut x).unwrap();
        let ux = x.trim().parse().unwrap();

        let mut y = String::new();
        println!("Move y");
        io::stdin().read_line(&mut y).unwrap();
        let uy = y.trim().parse().unwrap();

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

fn main() {
    let mut game = Game::new();
    game.start();
}
