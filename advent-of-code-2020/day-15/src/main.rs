use std::collections::HashMap;
use std::io::{self, BufRead};

fn load_input() -> Vec<i32> {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    line.split(',')
        .map(|token| token.parse().unwrap())
        .collect()
}

fn calculate_ith(numbers: &[i32], n: usize) -> i32 {
    let mut memory = HashMap::new();
    let mut last_number = -1;
    let mut next_number = -1;
    for i in 0..n {
        last_number = if i < numbers.len() {
            numbers[i]
        } else {
            next_number
        };
        let last_number_age = memory.get(&last_number).map(|j| i - j).unwrap_or(0);
        next_number = last_number_age as i32;
        memory.insert(last_number, i);
    }
    last_number
}

fn solve1(numbers: &[i32]) {
    let result = calculate_ith(numbers, 2020);
    println!("Result {}", result);
}

fn solve2(numbers: &[i32]) {
    let result = calculate_ith(numbers, 30000000);
    println!("Result {}", result);
}

fn main() {
    let numbers = load_input();
    solve1(&numbers);
    solve2(&numbers);
}
