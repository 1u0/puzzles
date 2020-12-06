use std::io::{self, BufRead};

// Represent the string as bit set
// (the expected range of characters in the input string is 'a'..='z').
fn encode(entry: &str) -> u32 {
    let mut code = 0;
    for ch in entry.chars() {
        code |= 1 << (ch as u32 - 'a' as u32);
    }
    code
}

fn load_groups() -> Vec<Vec<u32>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
            continue;
        }
        group.push(encode(line));
    }
    if !group.is_empty() {
        groups.push(group);
    }
    groups
}

fn solve1(entries: &[Vec<u32>]) {
    let sum: u32 = entries
        .iter()
        .map(|group| group.iter().fold(group[0], |acc, x| acc | x).count_ones()) // TODO: use reduce/fold_first
        .sum();
    println!("Result: {}", sum);
}

fn solve2(entries: &[Vec<u32>]) {
    let sum: u32 = entries
        .iter()
        .map(|group| group.iter().fold(group[0], |acc, x| acc & x).count_ones()) // TODO: use reduce/fold_first
        .sum();
    println!("Result: {}", sum);
}

fn main() {
    let groups = load_groups();
    solve1(&groups);
    solve2(&groups);
}
