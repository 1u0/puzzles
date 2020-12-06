use std::io::{self, BufRead};

fn decode(seat: &str) -> i32 {
    let mut code = 0;
    for ch in seat.chars() {
        code <<= 1;
        match ch {
            'B' | 'R' => code |= 1,
            'F' | 'L' => (),
            _ => panic!("invalid input"),
        }
    }
    code
}

fn load_entries() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| decode(&line.unwrap()))
        .collect()
}

fn solve1(sorted_entries: &[i32]) {
    let result = sorted_entries[sorted_entries.len() - 1];
    println!("Result: {}", result);
}

fn solve2(sorted_entries: &[i32]) {
    // Search using linear scan.
    // It's also possible to do binary search.
    let mut missing_entry = sorted_entries[0];
    for entry in sorted_entries {
        if missing_entry != *entry {
            // Means: missing_entry < entry
            break;
        }
        // missing_entry == entry
        missing_entry += 1;
    }
    println!("Result {}", missing_entry);
}

fn main() {
    let mut entries = load_entries();
    entries.sort_unstable();
    solve1(&entries);
    solve2(&entries);
}
