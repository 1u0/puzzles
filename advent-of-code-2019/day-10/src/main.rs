use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::stdin;
use std::io::BufRead;

struct Image {
    width: usize,
    height: usize,
    frame: Vec<u8>,
}

impl Image {
    fn read() -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut frame = Vec::new();
        for line in stdin().lock().lines() {
            let line = line.unwrap().trim().to_string();
            if width == 0 {
                width = line.len();
            }
            assert_eq!(width, line.len());
            height += 1;
            frame.extend(line.chars().map(|ch| match ch {
                '.' => 0,
                '#' => 1,
                _ => panic!("Unknown element on the map"),
            }));
        }

        Image {
            width,
            height,
            frame,
        }
    }

    fn print_frame(&self) {
        for (i, px) in self.frame.iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                println!();
            }
            let px = match px {
                0 => '.',
                1 => '#',
                2 => ' ',
                _ => panic!("unknown pixel value"),
            };
            print!("{}", px);
        }
    }

    fn get_coordinates(&self, pos: usize) -> (usize, usize) {
        (pos % self.width, pos / self.width)
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn get_direction(coord0: &(usize, usize), coord1: &(usize, usize)) -> (i32, i32) {
    let dx = coord1.0 as i32 - coord0.0 as i32;
    let dy = coord1.1 as i32 - coord0.1 as i32;
    if dx == 0 {
        (0, dy / dy.abs())
    } else if dy == 0 {
        (dx / dx.abs(), 0)
    } else {
        let gcd = gcd(dx.abs(), dy.abs());
        (dx / gcd, dy / gcd)
    }
}

fn get_distance(coord0: &(usize, usize), coord1: &(usize, usize)) -> i64 {
    let dx = coord0.0 as i64 - coord1.0 as i64;
    let dy = coord0.1 as i64 - coord1.1 as i64;
    dx.pow(2) + dy.pow(2)
}

fn cross_product(vec0: &(i32, i32), vec1: &(i32, i32)) -> i32 {
    vec0.1 * vec1.0 - vec0.0 * vec1.1
}

fn find_max_asteroids_monitored(image: &Image) -> (usize, usize) {
    let size = image.frame.len();
    let mut counts = vec![0; size];
    for pos0 in 0..size {
        if image.frame[pos0] == 0 {
            continue;
        }
        let coord0 = image.get_coordinates(pos0);
        let mut directions = HashSet::new();
        for pos1 in pos0 + 1..size {
            if image.frame[pos1] == 0 {
                continue;
            }
            let coord1 = image.get_coordinates(pos1);
            if directions.insert(get_direction(&coord0, &coord1)) {
                counts[pos0] += 1; // pos0 asteroid can monitor pos1 asteroid
                counts[pos1] += 1; // same, but in reverse direction
            }
        }
    }
    counts
        .iter()
        .cloned()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn get_nth_evaporated(image: &Image, center_pos: usize, n: usize) -> Option<usize> {
    let mut asteroids = Vec::new();
    let center_coord = image.get_coordinates(center_pos);
    for pos in 0..image.frame.len() {
        if image.frame[pos] == 0 || pos == center_pos {
            continue;
        }
        let coord = image.get_coordinates(pos);
        let dir = get_direction(&center_coord, &coord);
        let dist = get_distance(&center_coord, &coord);
        asteroids.push((pos, dir, dist));
    }
    asteroids.sort_by(|&(_, dir0, dist0), &(_, dir1, dist1)| {
        if dir0 == dir1 {
            dist0.cmp(&dist1)
        } else {
            let n0 = cross_product(&(0, -1), &dir0);
            let n1 = cross_product(&(0, -1), &dir1);

            let is_less = if (n0 > 0 && n1 > 0) || (n0 < 0 && n1 < 0) {
                // both on the same side relative to y-axis
                cross_product(&dir0, &dir1) < 0
            } else if n0 != 0 && n1 != 0 {
                // both not 0 and on different sides
                n0 < 0
            } else {
                if n0 == 0 {
                    dir0.1 < 0 || n1 > 0
                } else {
                    !(dir1.1 < 0 || n0 > 0)
                }
            };
            if is_less {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    });

    let mut count = 0;
    while !asteroids.is_empty() {
        let mut new_asteroids = Vec::new();
        let mut last_dir = None;
        for asteroid in asteroids {
            let dir = asteroid.1;
            if last_dir == Some(dir) {
                new_asteroids.push(asteroid);
            } else {
                last_dir = Some(dir);
                count += 1;
                if count == n {
                    return Some(asteroid.0);
                }
            }
        }
        asteroids = new_asteroids;
    }
    None
}

fn main() {
    let image = Image::read();
    let (pos, monitored_count) = find_max_asteroids_monitored(&image);
    println!("Result for task 1: {:?}", monitored_count);

    let pos = get_nth_evaporated(&image, pos, 200).unwrap();
    let coord = image.get_coordinates(pos);
    println!("Result for task 2: {:?}", 100 * coord.0 + coord.1);
}
