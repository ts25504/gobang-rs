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
        //clear_screen();
        println!("Welcome to Gobang game!");
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

    fn win(&self, x: usize, y: usize) -> bool {
        self.win_horizontal(x, y) || self.win_vertical(x, y) || self.win_diagonal_a(x, y) || self.win_diagonal_b(x, y)
    }

    fn win_horizontal(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut east: bool = true;
        let mut west: bool = true;
        while east || west {
            inc += 1;

            if y >= inc && self.points[x][y-inc] == color {
                serial_count += 1;
            } else {
                east = false;
            }

            if y + inc < 16 && self.points[x][y+inc] == color {
                serial_count += 1;
            } else {
                west = false;
            }
        }

        serial_count == 5
    }

    fn win_vertical(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut north: bool = true;
        let mut south: bool = true;
        while north || south {
            inc += 1;

            if x >= inc && self.points[x-inc][y] == color {
                serial_count += 1;
            } else {
                north = false;
            }

            if x + inc < 16 && self.points[x+inc][y] == color {
                serial_count += 1;
            } else {
                south = false;
            }
        }

        serial_count == 5
    }

    fn win_diagonal_a(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut northeast: bool = true;
        let mut southwest: bool = true;
        while northeast || southwest {
            inc += 1;

            if x >= inc && y + inc < 16 && self.points[x-inc][y+inc] == color {
                serial_count += 1;
            } else {
                northeast = false;
            }

            if y >= inc && x + inc < 16 && self.points[x+inc][y-inc] == color {
                serial_count += 1;
            } else {
                southwest = false;
            }
        }

        serial_count == 5
    }

    fn win_diagonal_b(&self, x: usize, y: usize) -> bool {
        let mut serial_count: i32 = 1;
        let color: char = self.points[x][y];
        let mut inc: usize = 0;

        let mut northwest: bool = true;
        let mut southeast: bool = true;
        while northwest || southeast {
            inc += 1;

            if x >= inc && y >= inc && self.points[x-inc][y-inc] == color {
                serial_count += 1;
            } else {
                northwest = false;
            }

            if x + inc < 16 && y + inc < 16 && self.points[x+inc][y+inc] == color {
                serial_count += 1;
            } else {
                southeast = false;
            }
        }

        serial_count == 5
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
        self.board.print();
        loop {
            let (bx, by) = self.input_coordinate();
            self.move_black(bx, by);

            if self.board.win(bx, by) {
                println!("Black win!");
                break;
            }

            let (wx, wy) = self.input_coordinate();
            self.move_white(wx, wy);

            if self.board.win(wx, wy) {
                println!("White win!");
                break;
            }
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
