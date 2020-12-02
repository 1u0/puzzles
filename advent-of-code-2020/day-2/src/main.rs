use std::io::BufRead;
use std::io;

struct Rule {
    min: usize,
    max: usize,
    char: char,
}

struct Entry {
    rule: Rule,
    password: String,
}

fn load_entries() -> Vec<Entry> {
    io::stdin().lock().lines().map(|line| {
        let line = line.unwrap();
        let min_line: Vec<&str> = line.splitn(2, '-').collect();
        let max_line: Vec<&str> = min_line[1].splitn(2, ' ').collect();
        let char_password: Vec<&str> = max_line[1].splitn(2, ": ").collect();
        Entry {
            rule: Rule {
                min: min_line[0].parse().unwrap(),
                max: max_line[0].parse().unwrap(),
                char: char_password[0].chars().next().unwrap(),
            },
            password: char_password[1].to_string()
        }
    })
    .collect()
}

fn solve1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|&entry| {
            let count = entry.password.matches(entry.rule.char).count();
            entry.rule.min <= count && count <= entry.rule.max
        })
        .count()
}

fn solve2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .filter(|&entry| {
            let password: Vec<char> = entry.password.chars().collect();
            let ch1 = password.get(entry.rule.min - 1);
            let ch2 = password.get(entry.rule.max - 1);
            let ch = Some(&entry.rule.char);
            (ch1 == ch) ^ (ch2 == ch)
        })
        .count()
}

fn main() {
    let entries = load_entries();
    println!("Result: {}", solve1(&entries));
    println!("Result: {}", solve2(&entries));
}
