use intcode::Byte;
use std::cmp::Ordering;
use std::collections::HashMap;

fn solve1() {
    let code = intcode::load_code();
    let output = intcode::run_code_with_inputs(code, Vec::new());

    let mut count = 0;
    for (i, value) in output.iter().enumerate() {
        if i % 3 == 2 && *value == 2 {
            count += 1;
        }
    }
    println!("Result for task 1: {:?}", count);
}

struct Game {
    iter: i64,
    count: i32,
    last_x: i32,
    last_y: i32,
    score: i64,
    display: HashMap<(i32, i32), char>,
    ball_x: i32,
    paddle_x: i32,
}

impl Game {
    fn new() -> Self {
        Game {
            iter: 0,
            count: 0,
            last_x: -1,
            last_y: -1,
            display: HashMap::new(),
            score: 0,
            ball_x: 0,
            paddle_x: 0,
        }
    }

    fn display(&self) {
        println!("Score: {}", self.score);
        for j in 0..24 {
            for i in 0..44 {
                let ch = self.display.get(&(i, j)).unwrap_or(&'?');
                print!("{}", ch)
            }
            println!();
        }
    }
}

impl intcode::Io for Game {
    fn input(&mut self) -> Byte {
        //        if self.iter % 1000 == 0 {
        //            self.display();
        //        }
        self.iter += 1;

        // Auto play: just follow the ball.
        match self.paddle_x.cmp(&self.ball_x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    fn output(&mut self, value: Byte) {
        match self.count {
            0 => {
                self.last_x = value as i32;
            }
            1 => {
                self.last_y = value as i32;
            }
            2 => {
                if self.last_x == -1 {
                    self.score = value;
                } else {
                    let ch = match value {
                        0 => ' ', // empty space
                        1 => '#', // a wall
                        2 => '+', // a block
                        3 => {
                            // paddle
                            self.paddle_x = self.last_x;
                            '='
                        }
                        4 => {
                            // ball
                            self.ball_x = self.last_x;
                            '@'
                        }
                        _ => panic!("unexpected output"),
                    };
                    self.display.insert((self.last_x, self.last_y), ch);
                }
            }
            _ => panic!("Unexpected state"),
        }
        self.count = (self.count + 1) % 3;
    }
}

fn play_game() {
    let mut code = intcode::load_code();
    code[0] = 2; // play for free
    let mut game = Game::new();
    let res = intcode::run_code(code, &mut game).is_ok();
    assert!(res);
    println!("Result for task 2: {}", game.score);
    game.display();
}

fn main() {
    play_game();
}
