use intcode::Byte;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

struct Discovery {
    maze: HashMap<(i32, i32), char>,
    pos: (i32, i32),
    dir: Dir,
    checked: i32,
    iter: i64,
}

impl Discovery {
    fn new() -> Self {
        let mut maze = HashMap::new();
        let pos = (0, 0);
        maze.insert(pos, 'x');
        Discovery {
            maze,
            pos,
            dir: Dir::East,
            checked: 0,
            iter: 0,
        }
    }

    fn get_next_pos(&self) -> (i32, i32) {
        let mut dx = 0;
        let mut dy = 0;
        match self.dir {
            Dir::North => {
                dy = -1;
            }
            Dir::South => {
                dy = 1;
            }
            Dir::West => {
                dx = -1;
            }
            Dir::East => {
                dx = 1;
            }
        };
        (self.pos.0 + dx, self.pos.1 + dy)
    }

    fn get_left_turn(&self) -> Dir {
        match self.dir {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    fn get_right_turn(&self) -> Dir {
        match self.dir {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn display(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for ((x, y), _) in self.maze.iter() {
            min_x = cmp::min(min_x, *x);
            max_x = cmp::max(max_x, *x);
            min_y = cmp::min(min_y, *y);
            max_y = cmp::max(max_y, *y);
        }
        let rows_count = max_y - min_y + 1;
        let cols_count = max_x - min_x + 1;

        let mut data = (0..rows_count)
            .map(|_| vec!['?'; cols_count as usize])
            .collect::<Vec<_>>();
        for (k, v) in self.maze.iter() {
            let i = (k.1 - min_y) as usize;
            let j = (k.0 - min_x) as usize;
            data[i][j] = *v;
        }
        for row in data.iter() {
            row.iter().for_each(|value| print!("{}", value));
            println!();
        }
    }
}

impl intcode::Io for Discovery {
    // This implements a simple maze traversal (right-hand rule)
    // that may not reveal the whole maze in a general case, but enough for the given problem.
    fn input(&mut self) -> Byte {
        match self.dir {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }

    fn output(&mut self, status: Byte) {
        match status {
            0 => {
                // last move was a hit to a wall
                self.maze.entry(self.get_next_pos()).or_insert('#');
                self.checked += 1;
                self.dir = self.get_left_turn();
            }
            1 => {
                let new_pos = self.get_next_pos();
                self.maze.entry(self.get_next_pos()).or_insert('.');
                self.dir = self.get_right_turn();
                self.pos = new_pos;
            }
            2 => {
                let new_pos = self.get_next_pos();
                self.maze.entry(self.get_next_pos()).or_insert('O');
                self.dir = self.get_right_turn();
                self.pos = new_pos;
                self.display();
            }
            _ => panic!("Unhandled status code {}", status),
        }
        self.iter += 1;
        if self.iter % 1000 == 0 {
            println!("Iteration: {}", self.iter);
            self.display();
        }
    }
}

// Run this function for a while on the original input (intcode program) to discover actual maze.
fn discover_maze() {
    let code = intcode::load_code();
    let mut discovery = Discovery::new();
    let res = intcode::run_code(code, &mut discovery).is_ok();
    assert!(res);
}

type Maze = Vec<Vec<char>>;

fn read_maze() -> Maze {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect()
}

fn find_oxygen(maze: &Maze) -> (i32, i32) {
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'O' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("The Maze has no oxygen!")
}

fn can_visit(maze: &Maze, pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        false
    } else {
        let i = pos.0 as usize;
        let j = pos.1 as usize;
        if i < maze.len() && j < maze[i].len() {
            match maze[i][j] {
                '.' | 'x' => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

fn display(maze: &Maze) {
    for row in maze.iter() {
        row.iter().for_each(|ch| print!("{}", ch));
        println!();
    }
}

fn count_bfs_iterations<Predicate>(maze: &Maze, predicate: &mut Predicate) -> i32
where
    Predicate: FnMut((i32, i32)) -> bool,
{
    let mut visited = HashSet::new();
    let mut front = vec![find_oxygen(&maze)];
    for pos in front.iter() {
        visited.insert(*pos);
    }

    let mut iter = 0;
    'search: while !front.is_empty() {
        iter += 1;
        let previous = front;
        front = Vec::new();
        for (i, j) in previous.iter() {
            let i = *i;
            let j = *j;
            for pos in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)].iter() {
                if visited.contains(pos) || !can_visit(&maze, *pos) {
                    continue;
                }
                if predicate(*pos) {
                    break 'search;
                }
                visited.insert(*pos);
                front.push(*pos);
            }
        }
    }
    iter - 1
}

fn count_until_origin(maze: &Maze) -> i32 {
    count_bfs_iterations(maze, &mut |pos| maze[pos.0 as usize][pos.1 as usize] == 'x')
}

fn count_until_fill(maze: &Maze) -> i32 {
    count_bfs_iterations(maze, &mut |_| false)
}

fn main() {
    let maze = read_maze();
    let result = count_until_origin(&maze) + 1;
    println!("Result for task 1: {:?}", result);
    let result = count_until_fill(&maze);
    println!("Result for task 2: {:?}", result);
}
