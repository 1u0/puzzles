use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

trait Grid<T> {
    fn get_start(&self) -> Vec<T>;
    fn is_finish(&self, point: &T) -> bool;
    fn next(&self, point: &T) -> Vec<T>;
}
fn traverse<T>(grid: &dyn Grid<T>) -> usize
where
    T: std::hash::Hash + std::cmp::Eq + Copy,
{
    let mut visited = HashSet::new();
    let mut front = grid.get_start().into_iter().collect::<HashSet<T>>();
    let mut count = 0;
    while !front.is_empty() {
        for point in front.iter() {
            if grid.is_finish(point) {
                return count;
            }
            visited.insert(*point);
        }
        let mut new_front = HashSet::new();
        for point in front.iter() {
            for point in grid.next(point) {
                if !visited.contains(&point) {
                    new_front.insert(point);
                }
            }
        }
        front = new_front;
        count += 1;
    }
    0
}

fn load_maze() -> Vec<Vec<char>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

type Coord = (usize, usize);
struct Maze {
    data: Vec<Vec<char>>,
    label_ids: HashMap<String, usize>,
    labeled: HashMap<Coord, usize>,
    ports: Vec<Vec<Coord>>,
}

fn register_id(index: &mut HashMap<String, usize>, id: String) -> usize {
    let last_id = index.len();
    *index.entry(id).or_insert(last_id)
}

impl Maze {
    fn new(data: Vec<Vec<char>>) -> Self {
        let n = data.len();
        let m = data[0].len();
        let mut label_ids = HashMap::new();
        let mut labeled = HashMap::new();
        let mut ports = Vec::new();
        for i in 0..n - 1 {
            for j in 0..m - 1 {
                let ch0 = data[i][j];
                if !ch0.is_ascii_alphabetic() {
                    continue;
                }
                let mut entry = None;
                for (ch1, dir) in [(data[i][j + 1], 0), (data[i + 1][j], 1)].iter() {
                    if !ch1.is_ascii_alphabetic() {
                        continue;
                    }
                    let label = String::from_utf8(vec![ch0 as u8, *ch1 as u8]).unwrap();
                    let label_id = register_id(&mut label_ids, label);
                    if ports.len() <= label_id {
                        ports.push(Vec::new());
                    }
                    if *dir == 0 {
                        if j > 0 && data[i][j - 1] == '.' {
                            entry = Some(((i, j - 1), label_id));
                        }
                        if j + 2 < m && data[i][j + 2] == '.' {
                            entry = Some(((i, j + 2), label_id));
                        }
                    } else {
                        if i > 0 && data[i - 1][j] == '.' {
                            entry = Some(((i - 1, j), label_id));
                        }
                        if i + 2 < n && data[i + 2][j] == '.' {
                            entry = Some(((i + 2, j), label_id));
                        }
                    }
                }
                if let Some((point, label_id)) = entry {
                    if labeled.insert(point, label_id).is_none() {
                        ports[label_id].push(point);
                    }
                }
            }
        }
        Maze {
            data,
            label_ids,
            labeled,
            ports,
        }
    }

    fn is_outer_point(&self, point: &Coord) -> bool {
        let n = self.data.len();
        let m = self.data[0].len();
        let (i, j) = *point;
        (i == 2 || i == n - 3) || (j == 2 || j == m - 3)
    }
}

impl Grid<Coord> for Maze {
    fn get_start(&self) -> Vec<Coord> {
        self.ports[*self.label_ids.get("AA").unwrap()].to_vec()
    }

    fn is_finish(&self, point: &Coord) -> bool {
        let finish = self.ports[*self.label_ids.get("ZZ").unwrap()]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        assert!(!finish.is_empty());
        finish.contains(point)
    }

    fn next(&self, point: &Coord) -> Vec<Coord> {
        let mut next = Vec::new();
        if let Some(label) = self.labeled.get(&point) {
            for port in self.ports[*label].iter() {
                next.push(*port);
            }
        }
        let (i, j) = point;
        let points: [Coord; 4] = [(i - 1, *j), (i + 1, *j), (*i, j - 1), (*i, j + 1)];
        for point in points.iter() {
            if self.data[point.0][point.1] == '.' {
                next.push(*point);
            }
        }
        next
    }
}

impl Grid<(Coord, i32)> for Maze {
    fn get_start(&self) -> Vec<(Coord, i32)> {
        self.ports[*self.label_ids.get("AA").unwrap()]
            .iter()
            .cloned()
            .map(|coord| (coord, 0))
            .collect()
    }

    fn is_finish(&self, point: &(Coord, i32)) -> bool {
        if point.1 == 0 {
            let finish = self.ports[*self.label_ids.get("ZZ").unwrap()]
                .iter()
                .cloned()
                .collect::<HashSet<_>>();
            assert!(!finish.is_empty());
            finish.contains(&point.0)
        } else {
            false
        }
    }

    fn next(&self, point: &(Coord, i32)) -> Vec<(Coord, i32)> {
        let (point, level) = point;
        let mut next = Vec::new();
        if let Some(label) = self.labeled.get(point) {
            for port in self.ports[*label].iter() {
                if port == point {
                    continue;
                }
                if self.is_outer_point(point) {
                    if *level > 0 {
                        next.push((*port, level - 1));
                    }
                } else {
                    next.push((*port, level + 1));
                }
            }
        }
        let (i, j) = point;
        let points: [Coord; 4] = [(i - 1, *j), (i + 1, *j), (*i, j - 1), (*i, j + 1)];
        for point in points.iter() {
            if self.data[point.0][point.1] == '.' {
                next.push((*point, *level));
            }
        }
        next
    }
}

fn main() {
    let maze = Maze::new(load_maze());
    let result = traverse::<Coord>(&maze);
    println!("Result: {}", result);
    let result = traverse::<(Coord, i32)>(&maze);
    println!("Result: {}", result);
}
