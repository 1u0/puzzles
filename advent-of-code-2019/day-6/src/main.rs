use std::collections::HashMap;
use std::io::stdin;
use std::io::BufRead;

const INVALID_IDX: usize = std::usize::MAX;

struct OrbitMap {
    index: HashMap<String, usize>,
    direct_orbits: Vec<Vec<usize>>,
    centers: Vec<usize>,
}

impl OrbitMap {
    fn new(orbits: Vec<(String, String)>) -> Self {
        let mut map = OrbitMap {
            index: HashMap::new(),
            direct_orbits: Vec::new(),
            centers: Vec::new(),
        };
        for (center, orbiter) in orbits.iter() {
            let center_idx = map.register(center);
            let orbiter_idx = map.register(orbiter);
            map.direct_orbits[center_idx].push(orbiter_idx);
            map.centers[orbiter_idx] = center_idx;
        }
        map
    }

    fn register(&mut self, body: &str) -> usize {
        let free_idx = self.direct_orbits.len();
        assert_eq!(free_idx, self.centers.len());
        let idx = *self.index.entry(body.to_string()).or_insert(free_idx);
        if idx == free_idx {
            self.direct_orbits.push(Vec::new());
            self.centers.push(INVALID_IDX);
        }
        idx
    }

    fn lookup(&self, body: &str) -> Option<usize> {
        self.index.get(body).cloned()
    }
}

fn read_map() -> Vec<(String, String)> {
    stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let pair: Vec<_> = line.trim().split(')').collect();
            assert!(pair.len() == 2);
            (pair[0].to_string(), pair[1].to_string())
        })
        .collect()
}

fn solve1() {
    let map = OrbitMap::new(read_map());
    let mut depth = 0;
    let mut total_orbits_count = 0;

    let idx = map.lookup("COM").unwrap();
    let mut horizon = map.direct_orbits[idx].to_vec();
    while horizon.len() > 0 {
        depth += 1;
        total_orbits_count += depth * horizon.len();
        horizon = horizon
            .iter()
            .flat_map(|idx| map.direct_orbits[*idx].iter())
            .cloned()
            .collect();
    }
    println!("Result: {}", total_orbits_count);
}

fn solve2() {
    let map = OrbitMap::new(read_map());
    let mut path: HashMap<usize, i32> = HashMap::new();
    let mut count = 0;
    let mut idx = map.lookup("YOU").unwrap();
    while idx != INVALID_IDX {
        idx = map.centers[idx];
        path.insert(idx, count);
        count += 1;
    }
    count = 0;
    idx = map.lookup("SAN").unwrap();
    while idx != INVALID_IDX {
        idx = map.centers[idx];
        if let Some(count0) = path.get(&idx) {
            count += count0;
            break;
        }
        count += 1;
    }
    println!("Result: {}", count);
}

fn main() {
    solve2();
}
