use intcode::Byte;
use std::char;

struct Grid {
    data: Vec<Vec<u8>>,
}
impl Grid {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Grid { data }
    }

    fn is_intersection(&self, pos: (i32, i32)) -> bool {
        let (i, j) = pos;
        self.is_scaffold(&pos)
            && [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                .iter()
                .filter(|pos| self.is_scaffold(pos))
                .count()
                > 2
    }

    fn is_scaffold(&self, pos: &(i32, i32)) -> bool {
        if self.is_in_grid(pos) {
            if pos.1 >= self.data[pos.0 as usize].len() as i32 {
                println!("i, j - out of bounds: {:?}", pos);
            }
            self.data[pos.0 as usize][pos.1 as usize] != '.' as u8
        } else {
            false
        }
    }

    fn is_in_grid(&self, pos: &(i32, i32)) -> bool {
        (0 <= pos.0 && (pos.0 as usize) < self.data.len())
            && (0 <= pos.1 && pos.1 < self.data[0].len() as i32)
    }

    fn get_size(&self) -> (usize, usize) {
        let n = self.data.len();
        let m = self.data[0].len();
        (n, m)
    }
}

fn solve1(code: &[Byte]) {
    let output = intcode::run_code_with_inputs(code.to_vec(), Vec::new());
    let grid = output
        .split(|v| *v == '\n' as intcode::Byte)
        .map(|row| row.iter().map(|v| *v as u8).collect::<Vec<_>>())
        .filter(|row| !row.is_empty())
        .collect();

    let grid = Grid::new(grid);
    let (n, m) = grid.get_size();
    let mut result = 0;
    for i in 0..n {
        for j in 0..m {
            if grid.is_intersection((i as i32, j as i32)) {
                result += i * j;
            }
        }
    }
    println!("Result for task 1: {}", result);
}

fn solve2(code: &[Byte]) {
    let mut code = code.to_vec();
    code[0] = 2;
    let instructions = "\
A,B,A,B,C,C,B,A,B,C
L,12,L,10,R,8,L,12
R,8,R,10,R,12
L,10,R,12,R,8
n
    ";
    let output = intcode::run_code_with_inputs(
        code,
        instructions.chars().rev().map(|ch| ch as Byte).collect(),
    );
    for v in output.iter() {
        if let Some(ch) = char::from_u32(*v as u32) {
            print!("{}({})", ch, v);
        } else {
            println!("{}", v);
        }
    }
}

fn main() {
    let code = intcode::load_code();
    solve1(&code);
    solve2(&code);
}
