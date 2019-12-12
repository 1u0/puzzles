use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Dimension {
    positions: Vec<i32>,
    velocities: Vec<i32>,
}

impl Dimension {
    fn new(positions: Vec<i32>, velocities: Vec<i32>) -> Self {
        Dimension {
            positions,
            velocities,
        }
    }

    fn update(&self) -> Self {
        let mut positions = self.positions.to_vec();
        let mut velocities = self.velocities.to_vec();
        let n = positions.len();
        for i in 0..n {
            let position = positions[i];
            for pos in positions.iter() {
                velocities[i] += match position.cmp(&pos) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
            }
        }
        for i in 0..n {
            positions[i] += velocities[i];
        }
        Dimension::new(positions, velocities)
    }
}

type Vector = [i32; 3];

struct Celestians {
    n: usize,
    dimensions: [Dimension; 3],
}

impl Celestians {
    fn new(positions: Vec<Vector>) -> Self {
        let n = positions.len();
        let dimension = |d| {
            Dimension::new(
                positions.iter().map(|pos| pos[d]).collect::<Vec<_>>(),
                vec![0; n],
            )
        };

        let dimensions = [dimension(0), dimension(1), dimension(2)];

        Celestians { n, dimensions }
    }

    fn get_dimension(&self, dimension: usize) -> Dimension {
        self.dimensions[dimension].clone()
    }

    fn update(&mut self) {
        for d in 0..3 {
            self.dimensions[d] = self.dimensions[d].update();
        }
    }

    fn get_total_energy(&self) -> i32 {
        (0..self.n).map(|i| self.get_energy(i)).sum()
    }

    fn get_energy(&self, i: usize) -> i32 {
        let mut potential = 0;
        let mut kinetic = 0;
        for d in 0..3 {
            potential += self.dimensions[d].positions[i].abs();
            kinetic += self.dimensions[d].velocities[i].abs();
        }
        potential * kinetic
    }
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

//<x=19, y=-10, z=7>
//<x=1, y=2, z=-3>
//<x=14, y=-4, z=1>
//<x=8, y=7, z=-6>
fn get_initial_positions() -> Vec<Vector> {
    vec![[19, -10, 7], [1, 2, -3], [14, -4, 1], [8, 7, -6]]
}

fn solve1() {
    let mut celestians = Celestians::new(get_initial_positions());
    for _ in 0..1000 {
        celestians.update();
    }
    println!("Result for task 1: {:?}", celestians.get_total_energy());
}

fn solve2() {
    let celestians = Celestians::new(get_initial_positions());
    let mut periods = Vec::new();
    for i in 0..3 {
        let mut seen = HashMap::new();
        let mut iter = 0;
        let mut state = celestians.get_dimension(i);
        loop {
            if let Some(_) = seen.insert(state.clone(), iter) {
                periods.push(iter as i64);
                break;
            }
            iter += 1;
            state = state.update();
        }
    }
    let period = periods
        .iter()
        .fold(1, |res, period| res / gcd(res, *period) * period);
    println!("Result for task 2: {:?}", period);
}

fn main() {
    solve1();
    solve2();
}
