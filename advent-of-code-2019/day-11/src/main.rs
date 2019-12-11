use intcode::Byte;
use std::cmp;
use std::collections::HashMap;

type Grid = HashMap<(i32, i32), u8>;

struct State {
    current_position: (i32, i32),
    current_direction: i32,
    grid: Grid,
    output_state: i8,
}

impl State {
    fn new() -> Self {
        State {
            current_position: (0, 0),
            current_direction: 0,
            grid: HashMap::new(),
            output_state: 0,
        }
    }
}

impl intcode::Io for State {
    fn input(&mut self) -> Byte {
        *self.grid.entry(self.current_position).or_insert(0) as Byte
    }

    fn output(&mut self, value: Byte) {
        if self.output_state == 0 {
            self.grid.insert(self.current_position, value as u8);
        } else {
            let dp;
            let dd;
            match value {
                0 => {
                    // turn left
                    dp = -1;
                    dd = 3;
                }
                1 => {
                    // turn right
                    dp = 1;
                    dd = 1;
                }
                _ => panic!("unhandled program value {:?}", value),
            }
            let mut x = self.current_position.0;
            let mut y = self.current_position.1;
            match self.current_direction {
                0 => {
                    x += dp;
                }
                1 => {
                    y += dp;
                }
                2 => {
                    x -= dp;
                }
                3 => {
                    y -= dp;
                }
                x => panic!("invalid state (direction): {:?}", x),
            };
            self.current_position = (x, y);
            self.current_direction = (self.current_direction + dd) % 4;
        }
        self.output_state = (self.output_state + 1) % 2;
    }
}

fn print_grid(grid: &Grid) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for ((x, y), _) in grid.iter() {
        min_x = cmp::min(min_x, *x);
        max_x = cmp::max(max_x, *x);
        min_y = cmp::min(min_y, *y);
        max_y = cmp::max(max_y, *y);
    }
    let rows_count = max_y - min_y + 1;
    let cols_count = max_x - min_x + 1;

    let mut data = (0..rows_count)
        .map(|_| vec![0u8; cols_count as usize])
        .collect::<Vec<_>>();
    for (k, v) in grid.iter() {
        let i = (k.1 - min_y) as usize;
        let j = (k.0 - min_x) as usize;
        data[i][j] = *v;
    }
    for row in data.iter() {
        row.iter().for_each(|value| match value {
            0 => print!("."),
            1 => print!("#"),
            _ => panic!(),
        });
        println!();
    }
}

fn run(code: &Vec<Byte>, init: bool) -> Grid {
    let mut state = State::new();

    if init {
        state.grid.insert((0, 0), 1);
    }
    assert!(intcode::run_code(code.to_vec(), &mut state).is_ok());
    state.grid
}

fn main() {
    let code = intcode::load_code();
    let grid = run(&code, false);
    println!("Result for task 1: {:?}", grid.len());

    let grid = run(&code, true);
    println!("Result for task 2:");
    print_grid(&grid);
}
