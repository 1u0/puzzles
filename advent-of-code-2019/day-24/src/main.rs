use std::collections::HashSet;
use std::io;
use std::io::BufRead;

type GridData = u32;
const EMPTY_GRID: GridData = 0;

fn print_connectivity() {
    let n = 5;
    for i in 0..n * n {
        let mut connectivity: u32 = 0;
        if i >= n {
            connectivity |= 1 << (i - n);
        }
        if (i % n) > 0 {
            connectivity |= 1 << (i - 1);
        }
        if (i % n) < n - 1 {
            connectivity |= 1 << (i + 1);
        }
        if i + n < n * n {
            connectivity |= 1 << (i + n);
        }
        println!("    {:#027b},", connectivity);
    }
}

fn print_infinite_connectivity() {
    let n = 5;
    let center = n * n / 2;
    let mut connectivity = vec![(0, 0, 0); n * n];
    for i in 0..n * n {
        let mut value: u32 = 0;
        if i >= n {
            value |= 1 << (i - n);
        } else {
            connectivity[center - n].0 |= 1 << i;
            connectivity[i].2 |= 1 << (center - n);
        }
        if (i % n) > 0 {
            value |= 1 << (i - 1);
        } else {
            connectivity[center - 1].0 |= 1 << i;
            connectivity[i].2 |= 1 << (center - 1);
        }
        if (i % n) < n - 1 {
            value |= 1 << (i + 1);
        } else {
            connectivity[center + 1].0 |= 1 << i;
            connectivity[i].2 |= 1 << (center + 1);
        }
        if i + n < n * n {
            value |= 1 << (i + n);
        } else {
            connectivity[center + n].0 |= 1 << i;
            connectivity[i].2 |= 1 << (center + n);
        }
        if i != center {
            connectivity[i].1 = value;
        }
    }
    for (x, y, z) in connectivity {
        println!("    [{:#027b}, {:#027b}, {:#027b}], ", x, y, z);
    }
}

const CONNECTIVITY: [u32; 25] = [
    0b0000000000000000000100010,
    0b0000000000000000001000101,
    0b0000000000000000010001010,
    0b0000000000000000100010100,
    0b0000000000000001000001000,
    0b0000000000000010001000001,
    0b0000000000000100010100010,
    0b0000000000001000101000100,
    0b0000000000010001010001000,
    0b0000000000100000100010000,
    0b0000000001000100000100000,
    0b0000000010001010001000000,
    0b0000000100010100010000000,
    0b0000001000101000100000000,
    0b0000010000010001000000000,
    0b0000100010000010000000000,
    0b0001000101000100000000000,
    0b0010001010001000000000000,
    0b0100010100010000000000000,
    0b1000001000100000000000000,
    0b0001000001000000000000000,
    0b0010100010000000000000000,
    0b0101000100000000000000000,
    0b1010001000000000000000000,
    0b0100010000000000000000000,
];

const INFINITE_CONNECTIVITY: [[u32; 3]; 25] = [
    [
        0b0000000000000000000000000,
        0b0000000000000000000100010,
        0b0000000000000100010000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000000001000101,
        0b0000000000000000010000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000000010001010,
        0b0000000000000000010000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000000100010100,
        0b0000000000000000010000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000001000001000,
        0b0000000000010000010000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000010001000001,
        0b0000000000000100000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000100010100010,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000011111,
        0b0000000000001000101000100,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000010001010001000,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000100000100010000,
        0b0000000000010000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000001000100000100000,
        0b0000000000000100000000000,
    ],
    [
        0b0000100001000010000100001,
        0b0000000010001010001000000,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000000000000000000000000,
        0b0000000000000000000000000,
    ],
    [
        0b1000010000100001000010000,
        0b0000001000101000100000000,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000010000010001000000000,
        0b0000000000010000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0000100010000010000000000,
        0b0000000000000100000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0001000101000100000000000,
        0b0000000000000000000000000,
    ],
    [
        0b1111100000000000000000000,
        0b0010001010001000000000000,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0100010100010000000000000,
        0b0000000000000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b1000001000100000000000000,
        0b0000000000010000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0001000001000000000000000,
        0b0000000100000100000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0010100010000000000000000,
        0b0000000100000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0101000100000000000000000,
        0b0000000100000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b1010001000000000000000000,
        0b0000000100000000000000000,
    ],
    [
        0b0000000000000000000000000,
        0b0100010000000000000000000,
        0b0000000100010000000000000,
    ],
];

fn load_grid() -> GridData {
    let mut grid = EMPTY_GRID;
    let mut mask = 1;
    for line in io::stdin().lock().lines() {
        for ch in line.unwrap().chars() {
            if ch == '#' {
                grid |= mask;
            }
            mask <<= 1;
        }
    }
    grid
}

fn next(grid: GridData) -> GridData {
    let mut new_grid = 0;
    let mut mask = 1;
    for connectivity in &CONNECTIVITY {
        let count = (grid & connectivity).count_ones();
        let alive = if grid & mask != 0 {
            count == 1
        } else {
            count == 1 || count == 2
        };
        if alive {
            new_grid |= mask;
        }
        mask <<= 1;
    }
    new_grid
}

fn next_infinite(inner_grid: GridData, grid: GridData, outer_grid: GridData) -> GridData {
    let mut new_grid = 0;
    let mut mask = 1;
    for connectivity in &INFINITE_CONNECTIVITY {
        let count: u32 = [inner_grid, grid, outer_grid]
            .iter()
            .zip(connectivity)
            .map(|(grid, connectivity)| (grid & connectivity).count_ones())
            .sum();

        let alive = if grid & mask != 0 {
            count == 1
        } else {
            count == 1 || count == 2
        };
        if alive {
            new_grid |= mask;
        }
        mask <<= 1;
    }
    new_grid
}

fn solve1(grid: GridData) {
    let mut grid = grid;
    let mut tracked = HashSet::new();
    while tracked.insert(grid) {
        grid = next(grid);
    }
    println!("Result for task 1: {}", grid);
}

fn solve2(grid: GridData) {
    let iterations = 200;
    let mut layers = vec![grid];
    for _ in 0..iterations {
        let n = layers.len();
        let mut new_layers = Vec::new();

        let new_layer = next_infinite(EMPTY_GRID, EMPTY_GRID, layers[0]);
        if new_layer != EMPTY_GRID {
            new_layers.push(new_layer);
        }

        for (i, layer) in layers.iter().enumerate() {
            let inner_layer = if i > 0 { layers[i - 1] } else { EMPTY_GRID };
            let outer_layer = if i + 1 < n { layers[i + 1] } else { EMPTY_GRID };
            let new_layer = next_infinite(inner_layer, *layer, outer_layer);
            new_layers.push(new_layer);
        }

        let new_layer = next_infinite(layers[n - 1], EMPTY_GRID, EMPTY_GRID);
        if new_layer != EMPTY_GRID {
            new_layers.push(new_layer);
        }

        layers = new_layers;
    }
    let total_count: u64 = layers.iter().map(|grid| grid.count_ones() as u64).sum();
    println!("Result for task 2: {}", total_count);
}

fn main() {
    //    print_connectivity();
    //    print_infinite_connectivity();
    let grid = load_grid();
    solve1(grid);
    solve2(grid);
}
