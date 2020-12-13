use gcd::Gcd;
use std::io::{self, BufRead};

fn load_puzzle() -> (i32, Vec<Option<i32>>) {
    let mut timestamp = 0;
    let mut schedule = Vec::new();
    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        match i {
            0 => timestamp = line.parse().unwrap(),
            1 => {
                schedule = line
                    .split(',')
                    .map(|str| match str {
                        "x" => None,
                        _ => Some(str.parse().unwrap()),
                    })
                    .collect()
            }
            _ => break,
        }
    }
    (timestamp, schedule)
}

fn solve1(timestamp: i32, schedule: &[Option<i32>]) {
    let result = schedule
        .iter()
        .filter_map(|period| period.as_ref())
        .map(|period| ((timestamp + period - 1) / period * period, period))
        .min_by_key(|entry| entry.0)
        .map(|entry| (entry.0 - timestamp) * entry.1);
    println!("Result: {:?}", result);
}

fn solve2(schedule: &[Option<i32>]) {
    // Recursive solution:
    // Invariant:
    //  result[i] := (a[i], p[i]), where:
    //  a[i] is minimum value: (a[i] + sj) % pj == 0
    //  p[i] is minimum value: p[i] % pj == 0
    //  for j in 0..i
    // Recursion step:
    //  result[i+1] := (a[i+1], p[i+1]), where:
    //  a[i+1] := (a[i] + x * p[i]) % pi == 0, min by x
    //  p[i+1] := p[i] * pi / gcd(p[i], pi)

    fn solve(result: (u64, u64), t: (u64, u64)) -> (u64, u64) {
        let s = (t.1 - t.0 % t.1) % t.1;
        if result.1 == 0 {
            (s, t.1)
        } else {
            // solve(&result, &t)
            let d = (result.1 as u64).gcd(t.1 as u64);
            let p = result.1 / d * t.1;
            let mut a = result.0;
            for _i in 0..t.1 {
                if a % t.1 == s {
                    break;
                }
                a += result.1;
            }
            (a, p)
        }
    }
    let result = schedule
        .iter()
        .enumerate()
        .filter_map(|(i, period)| period.map(|p| (i as u64, p as u64)))
        .fold((0u64, 0u64), solve); // TODO: use fold_first (aka reduce) after the api is stabilized.
    println!("Result: {}", result.0);
}

fn main() {
    let puzzle = load_puzzle();
    solve1(puzzle.0, &puzzle.1);
    solve2(&puzzle.1);
}
