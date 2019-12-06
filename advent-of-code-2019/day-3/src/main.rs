use std::iter::repeat_with;
use std::iter::Iterator;
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
    (
        bounds0.0.extend_to(bounds1.0.min).extend_to(bounds1.0.max),
        bounds0.1.extend_to(bounds1.1.min).extend_to(bounds1.1.max),
    )
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

    fn point_data(&mut self, point: (i32, i32)) -> &mut u32 {
        &mut self.data[(point.0 + self.origin.0) as usize][(point.1 + self.origin.1) as usize]
    }

    fn iterate<Iter: Iterator<Item = (i32, i32)>>(
        &mut self,
        func: &mut dyn FnMut((i32, i32), &mut u32),
        iter: Iter,
    ) {
        for point in iter {
            func(point, &mut self.point_data(point));
        }
    }

    fn trace_path(&mut self, path: &Path, func: &mut dyn FnMut((i32, i32), &mut u32)) {
        let mut i = 0;
        let mut j = 0;
        for (dir, steps) in path.path.iter() {
            match dir {
                Dir::Horizontal => {
                    if *steps >= 0 {
                        self.iterate(func, (1..steps + 1).map(|di| (i + di, j)));
                    } else {
                        self.iterate(func, (-1..steps - 1).map(|di| (i + di, j)));
                    }
                    i += steps;
                }
                Dir::Vertical => {
                    if *steps >= 0 {
                        self.iterate(func, (1..steps + 1).map(|dj| (i, j + dj)));
                    } else {
                        self.iterate(func, (*steps..0).map(|dj| (i, j + dj)));
                    };
                    j += steps;
                }
            }
        }
    }
}

fn solve1() -> Option<i32> {
    let path0 = Path::read_path();
    let path1 = Path::read_path();
    let bounds = get_common_bounds(path0.get_bounds(), path1.get_bounds());
    let mut grid = Grid::new(bounds);

    let mut counter: u32 = 0;
    grid.trace_path(&path0, &mut |_, value| {
        counter += 1;
        if *value == 0 {
            *value = counter;
        }
    });
    let mut min_distance = None;
    grid.trace_path(&path1, &mut |point, &mut value| {
        if value != 0 {
            let dist = point.0.abs() + point.1.abs();
            min_distance = match min_distance {
                None => Some(dist),
                Some(dist0) => Some(cmp::min(dist0, dist)),
            };
        }
    });
    return min_distance;
}

fn main() {
    println!("Result: {:?}", solve1());
}
