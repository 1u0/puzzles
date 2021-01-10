use std::io::{self, BufRead};

fn read_grid() -> Vec<Vec<char>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn count_trees_on_slope(grid: &[Vec<char>], dx: usize, dy: usize) -> i64 {
    let m = grid[0].len();
    grid.iter()
        .step_by(dx)
        .enumerate()
        .filter(|(x, row)| row[x * dy % m] == '#')
        .count() as i64
}

fn solve1(grid: &[Vec<char>]) {
    let result = count_trees_on_slope(grid, 1, 3);
    println!("Result: {}", result);
}

fn solve2(grid: &[Vec<char>]) {
    let result = count_trees_on_slope(grid, 1, 1)
        * count_trees_on_slope(grid, 1, 3)
        * count_trees_on_slope(grid, 1, 5)
        * count_trees_on_slope(grid, 1, 7)
        * count_trees_on_slope(grid, 2, 1);
    println!("Result: {}", result);
}

fn main() {
    let grid = read_grid();
    solve1(&grid);
    solve2(&grid);
}
