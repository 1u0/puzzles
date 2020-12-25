use std::collections::HashMap;
use std::io::{self, BufRead};

fn load_input() -> (u64, u64) {
    let nums: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.parse().unwrap()
        })
        .collect();
    (nums[0], nums[1])
}

fn build_inverse_pow(num: u64, modulo: u64) -> HashMap<u64, u64> {
    let mut cache = HashMap::new();
    let mut i = 1;
    let mut result = num;
    while cache.insert(result, i) == None {
        i += 1;
        result = (result * num) % modulo;
    }
    cache
}

fn pow(num: u64, p: u64, modulo: u64) -> u64 {
    match p {
        0 => 1,
        1 => num,
        _ => {
            let num2 = (num * num) % modulo;
            let mut result = pow(num2, p / 2, modulo);
            if p % 2 == 1 {
                result = (result * num) % modulo
            }
            result
        }
    }
}

fn solve1(a: u64, b: u64) {
    let modulo = 20201227;
    let pows = build_inverse_pow(7, modulo);
    let pow_a = pows.get(&a).unwrap();
    println!("Result: {}", pow(b, *pow_a, modulo));
}

fn main() {
    let (a, b) = load_input();
    solve1(a, b);
}
