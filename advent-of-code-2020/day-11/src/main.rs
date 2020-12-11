use std::io::{self, BufRead};

fn load_grid() -> Vec<Vec<char>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect()
        })
        .collect()
}

fn count_occupied_neighbours(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let max_i = grid.len() as i32;
    let max_j = grid[i].len() as i32;
    let mut count = 0;
    let i = i as i32;
    let j = j as i32;
    for &di in [-1, 0, 1].iter() {
        let i = i + di;
        if !(0 <= i && i < max_i) {
            continue;
        }
        for &dj in [-1, 0, 1].iter() {
            let j = j + dj;
            if !(0 <= j && j < max_j) {
                continue;
            }
            if !(di == 0 && dj == 0) && grid[i as usize][j as usize] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn count_occupied_in_view(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let max_i = grid.len() as i32;
    let max_j = grid[i].len() as i32;
    let is_occupied_in_direction = |di: i32, dj: i32| {
        let mut i = i as i32 + di;
        let mut j = j as i32 + dj;
        while 0 <= i && i < max_i && 0 <= j && j < max_j {
            match grid[i as usize][j as usize] {
                '#' => return true,
                'L' => return false,
                _ => (),
            }
            i += di;
            j += dj;
        }
        false
    };
    let mut count = 0;
    for &di in [-1, 0, 1].iter() {
        for &dj in [-1, 0, 1].iter() {
            if !(di == 0 && dj == 0) && is_occupied_in_direction(di, dj) {
                count += 1;
            }
        }
    }
    count
}

fn count_occupied(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|&&s| s == '#').count())
        .sum()
}

fn iterate(
    grid: &[Vec<char>],
    can_be_occupied: &mut dyn FnMut(&[Vec<char>], usize, usize) -> bool,
    should_be_freed: &mut dyn FnMut(&[Vec<char>], usize, usize) -> bool,
) -> Vec<Vec<char>> {
    let mut grid = grid.to_vec();
    let mut iteration = 0;
    loop {
        let mut changes = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                match x {
                    '.' => (),
                    'L' => {
                        if can_be_occupied(&grid, i, j) {
                            changes.push((i, j, '#'));
                        }
                    }
                    '#' => {
                        if should_be_freed(&grid, i, j) {
                            changes.push((i, j, 'L'));
                        }
                    }
                    _ => panic!("invalid grid value"),
                }
            }
        }
        if changes.is_empty() {
            break;
        }
        for &(i, j, v) in changes.iter() {
            grid[i][j] = v;
        }
        iteration += 1;
    }
    println!("iterations: {}", iteration);
    grid
}

fn solve1(grid: &[Vec<char>]) {
    let result = count_occupied(&iterate(
        grid,
        &mut |grid, i, j| count_occupied_neighbours(&grid, i, j) == 0,
        &mut |grid, i, j| count_occupied_neighbours(&grid, i, j) >= 4,
    ));
    println!("Result {}", result);
}

fn solve2(grid: &[Vec<char>]) {
    let result = count_occupied(&iterate(
        grid,
        &mut |grid, i, j| count_occupied_in_view(&grid, i, j) == 0,
        &mut |grid, i, j| count_occupied_in_view(&grid, i, j) >= 5,
    ));
    println!("Result {}", result);
}

fn main() {
    let grid = load_grid();
    solve1(&grid);
    solve2(&grid);
}
