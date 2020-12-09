use std::collections::{hash_map::Entry, HashMap};
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

fn solve1(data: &[i32]) -> i32 {
    let preamble_len: usize = 25;
    let mut preamble = HashMap::<i32, i32>::new();
    for i in 0..preamble_len {
        let a0 = data[i];
        for &a1 in data[i..preamble_len].iter() {
            if a0 != a1 {
                *preamble.entry(a0 + a1).or_default() += 1;
            }
        }
    }
    for i in preamble_len..data.len() {
        let next = data[i];
        if !preamble.contains_key(&next) {
            return next;
        }
        let prev = data[i - preamble_len];
        for &a in data[i - preamble_len + 1..i].iter() {
            if next != a {
                *preamble.entry(next + a).or_default() += 1;
            }
            if prev != a {
                match preamble.entry(prev + a) {
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() -= 1;
                        if *entry.get() == 0 {
                            entry.remove();
                        }
                    }
                    _ => unreachable!("previous sum must exist!"),
                }
            }
        }
    }
    unreachable!("result not found");
}

fn solve2(data: &[i32], sum: i32) -> i32 {
    // Note: this solution assumes that all values are positive (>= 0)
    //  and that exists (otherwise, we can reach out of bounds of the array).
    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut asum = data[i]; // sum of elements in half open sub range [i, j)
    loop {
        if asum > sum {
            asum -= data[i];
            i += 1;
        }
        if asum < sum || (j - i) < 2 {
            asum += data[j]; // TODO: check if we out of range
            j += 1;
        } else if asum == sum {
            return data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap();
        }
    }
}

fn main() {
    let data = load_data();
    let n = solve1(&data);
    println!("Result {}", n);
    let n = solve2(&data, n);
    println!("Result {}", n);
}
