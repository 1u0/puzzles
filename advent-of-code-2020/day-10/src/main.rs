use std::io::{self, BufRead};

fn load_data() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.trim().parse()
        })
        .filter_map(Result::ok)
        .collect()
}

fn solve1(sorted_data: &[i32]) {
    let mut prev = 0;
    let mut counts = vec![0; 3 + 1];
    for &x in sorted_data.iter() {
        assert!(prev < x, "assume that all values are distinct");
        let diff = x - prev;
        prev = x;
        assert!(diff <= 3, "assume that the sequence is valid");
        counts[diff as usize] += 1;
    }
    counts[3] += 1;
    let result = counts[1] * counts[3];
    println!("Result {}", result);
}

fn solve2(sorted_data: &[i32]) {
    let mut counts: Vec<i64> = vec![0; (*sorted_data.last().unwrap() + 1) as usize];
    counts[0] = 1;
    for &x in sorted_data {
        for xi in x - 3..x {
            if xi >= 0 {
                counts[x as usize] += counts[xi as usize];
            }
        }
    }
    let result = counts.last().unwrap();
    println!("Result {}", result);
}

fn main() {
    let mut data = load_data();
    data.sort_unstable();
    solve1(&data);
    solve2(&data);
}
