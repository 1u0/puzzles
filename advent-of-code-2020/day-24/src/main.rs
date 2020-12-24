use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum HexDirection {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

static ALL_DIRECTIONS: [HexDirection; 6] = [
    HexDirection::E,
    HexDirection::SE,
    HexDirection::SW,
    HexDirection::W,
    HexDirection::NW,
    HexDirection::NE,
];

fn delta(direction: HexDirection) -> (i32, i32) {
    match direction {
        HexDirection::E => (1, 0),
        HexDirection::SE => (1, 1),
        HexDirection::SW => (0, 1),
        HexDirection::W => (-1, 0),
        HexDirection::NW => (-1, -1),
        HexDirection::NE => (0, -1),
    }
}

fn move_towards(coord: &(i32, i32), direction: HexDirection) -> (i32, i32) {
    let delta = delta(direction);
    (coord.0 + delta.0, coord.1 + delta.1)
}

fn load_directions() -> Vec<Vec<HexDirection>> {
    fn parse_path(line: &str) -> Vec<HexDirection> {
        let mut path = Vec::new();
        let mut chars = line.chars();
        let mut char = chars.next();
        while char.is_some() {
            let direction = match char.unwrap() {
                'e' => HexDirection::E,
                'w' => HexDirection::W,
                's' => match chars.next() {
                    Some('e') => HexDirection::SE,
                    Some('w') => HexDirection::SW,
                    _ => panic!("invalid input state"),
                },
                'n' => match chars.next() {
                    Some('e') => HexDirection::NE,
                    Some('w') => HexDirection::NW,
                    _ => panic!("invalid input state"),
                },
                _ => panic!("invalid input state"),
            };
            path.push(direction);
            char = chars.next();
        }
        path
    }
    io::stdin()
        .lock()
        .lines()
        .map(|line| parse_path(&line.unwrap()))
        .collect()
}

fn build_tiles(directions: &[Vec<HexDirection>]) -> HashMap<(i32, i32), i32> {
    let mut tiles = HashMap::new();
    for path in directions {
        let coord = path
            .iter()
            .fold((0, 0), |coord, direction| move_towards(&coord, *direction));
        *tiles.entry(coord).or_insert(0) += 1;
    }
    // Keep only black tiles: stepping on a tile twice has no effect in color change,
    // initially all tiles are white.
    tiles.into_iter().filter(|entry| entry.1 % 2 == 1).collect()
}

fn solve1(tiles: &HashMap<(i32, i32), i32>) {
    let result = tiles.len();
    println!("Result: {}", result);
}

fn solve2(tiles: &HashMap<(i32, i32), i32>) {
    let mut tiles = tiles.clone();
    for _iteration in 1..101 {
        let mut neighbours = HashMap::new();
        for tile in tiles.keys() {
            for direction in ALL_DIRECTIONS.iter() {
                let neighbour_tile = move_towards(tile, *direction);
                *neighbours.entry(neighbour_tile).or_insert(0) += 1;
            }
        }
        tiles = neighbours
            .into_iter()
            .filter(|entry| entry.1 == 2 || entry.1 == 1 && tiles.contains_key(&entry.0))
            .collect();
    }
    let result = tiles.len();
    println!("Result: {}", result);
}

fn main() {
    let directions = load_directions();
    let tiles = build_tiles(&directions);
    solve1(&tiles);
    solve2(&tiles);
}
