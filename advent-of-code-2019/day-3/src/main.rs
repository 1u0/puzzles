use std::iter::repeat_with;
use std::{cmp, io};

struct Bound {
    min: i32,
    max: i32,
}

impl Bound {
    fn extend_to(&self, point: i32) -> Bound {
        Bound {
            min: cmp::min(self.min, point),
            max: cmp::max(self.max, point),
        }
    }
}

fn parse_token(astr: &str) -> (Dir, i32) {
    let steps = astr[1..].parse().unwrap();
    match astr.chars().next().unwrap() {
        'R' => (Dir::Horizontal, steps),
        'L' => (Dir::Horizontal, -steps),
        'D' => (Dir::Vertical, steps),
        'U' => (Dir::Vertical, -steps),
        chr => panic!("Invalid path direction: {}", chr),
    }
}

enum Dir {
    Horizontal,
    Vertical,
}

struct Path {
    path: Vec<(Dir, i32)>,
}

impl Path {
    fn read_path() -> Self {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let path = input
            .trim()
            .split(",")
            .map(|str| parse_token(str))
            .collect();

        Path { path }
    }

    fn get_bounds(&self) -> (Bound, Bound) {
        let mut bounds = (Bound { min: 0, max: 0 }, Bound { min: 0, max: 0 });
        let mut pos = (0, 0);
        for (dir, steps) in self.path.iter() {
            match dir {
                Dir::Horizontal => {
                    pos.0 += steps;
                    bounds.0 = bounds.0.extend_to(pos.0);
                }
                Dir::Vertical => {
                    pos.1 += steps;
                    bounds.1 = bounds.1.extend_to(pos.1);
                }
            }
        }
        bounds
    }
}

fn get_common_bounds(bounds0: (Bound, Bound), bounds1: (Bound, Bound)) -> (Bound, Bound) {
    let mut common_bounds = bounds0;
    common_bounds.0 = common_bounds
        .0
        .extend_to(bounds1.0.min)
        .extend_to(bounds1.0.max);
    common_bounds.1 = common_bounds
        .1
        .extend_to(bounds1.1.min)
        .extend_to(bounds1.1.max);
    common_bounds
}

struct Grid {
    origin: (i32, i32),
    data: Vec<Vec<u32>>,
}

impl Grid {
    fn new(bounds: (Bound, Bound)) -> Self {
        let origin = (-bounds.0.min, -bounds.1.min);

        let x_len = (bounds.0.max - bounds.0.min + 1) as usize;
        let y_len = (bounds.1.max - bounds.1.min + 1) as usize;
        let data = repeat_with(|| vec![0; y_len]).take(x_len).collect();

        Grid { origin, data }
    }

    fn trace_path(&mut self, i: usize, path: &Path) -> Vec<(i32, i32)> {
        assert!(i < 32);
        let path_mask = 1 << i;

        let mut intersections: Vec<(i32, i32)> = Vec::new();
        let mut i = self.origin.0;
        let mut j = self.origin.1;
        for (dir, steps) in path.path.iter() {
            match dir {
                Dir::Horizontal => {
                    let range = if *steps >= 0 {
                        (1..steps + 1)
                    } else {
                        (*steps..0)
                    };
                    for di in range {
                        let i1 = (i + di) as usize;
                        let j1 = j as usize;
                        intersections.extend(self.mark_point(i1, j1, path_mask).iter());
                    }
                    i += steps;
                }
                Dir::Vertical => {
                    let range = if *steps >= 0 {
                        (1..steps + 1)
                    } else {
                        (*steps..0)
                    };
                    for dj in range {
                        let i1 = i as usize;
                        let j1 = (j + dj) as usize;
                        intersections.extend(self.mark_point(i1, j1, path_mask).iter());
                    }
                    j += steps;
                }
            }
        }
        intersections
    }

    fn mark_point(&mut self, i: usize, j: usize, mask: u32) -> Option<(i32, i32)> {
        self.data[i][j] |= mask;
        let res_mask = self.data[i][j];
        if res_mask & !mask != 0 {
            Some((i as i32 - self.origin.0, j as i32 - self.origin.1))
        } else {
            None
        }
    }
}

fn solve1() -> Option<i32> {
    let path0 = Path::read_path();
    let path1 = Path::read_path();
    let bounds = get_common_bounds(path0.get_bounds(), path1.get_bounds());
    let mut grid = Grid::new(bounds);
    grid.trace_path(0, &path0);
    let min_distance = grid
        .trace_path(1, &path1)
        .iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min();
    return min_distance;
}

fn main() {
    println!("Result: {:?}", solve1());
}
