use std::io::stdin;
use std::io::BufRead;

fn read_grid() -> Vec<Vec<char>> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn count_trees_on_slope(grid: &[Vec<char>], dx: usize, dy: usize) -> i64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while x < n {
        if grid[x][y] == '#' {
            count += 1;
        }
        x += dx;
        y = (y + dy) % m;
    }
    return count;
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
