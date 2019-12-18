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

fn load_grid() -> Grid {
    let code = intcode::load_code();
    let output = intcode::run_code_with_inputs(code, Vec::new());
    let grid = output
        .split(|v| *v == '\n' as intcode::Byte)
        .map(|row| row.iter().map(|v| *v as u8).collect::<Vec<_>>())
        .filter(|row| !row.is_empty())
        .collect();
    Grid::new(grid)
}

fn main() {
    let grid = load_grid();
    let (n, m) = grid.get_size();
    let mut result = 0;
    for i in 0..n {
        for j in 0..m {
            if grid.is_intersection((i as i32, j as i32)) {
                result += i * j;
            }
        }
    }
    println!("{}", result);
}
