// Build chain of cups in a circle
fn build_cups_mapping(prefix: &str, total_cups: usize) -> Vec<u32> {
    assert!(prefix.len() <= total_cups);
    let mut cups = vec![0; total_cups];
    let mut prev_cup = u32::MAX;
    for ch in prefix.chars() {
        let cup = ch as u32 - '1' as u32;
        if prev_cup != u32::MAX {
            cups[prev_cup as usize] = cup;
        }
        prev_cup = cup;
    }
    for cup in prefix.len()..total_cups {
        let cup = cup as u32;
        cups[prev_cup as usize] = cup;
        prev_cup = cup;
    }
    let first_cup = prefix.chars().next().unwrap() as u32 - '1' as u32;
    cups[prev_cup as usize] = first_cup;

    cups
}

fn iterate(cups: &mut Vec<u32>, start_cup: u32, iterations: u32) {
    let n = cups.len() as u32;
    let mut current_cup = start_cup;
    for _ in 0..iterations {
        let cup1 = cups[current_cup as usize];
        let cup2 = cups[cup1 as usize];
        let cup3 = cups[cup2 as usize];
        let next_cup = cups[cup3 as usize];
        // Determine the target cup
        let mut target_cup = (current_cup + n - 1) % n;
        while [cup1, cup2, cup3].contains(&target_cup) {
            target_cup = (target_cup + n - 1) % n;
        }
        // Update the chain
        let next_after_target_cup = cups[target_cup as usize];
        cups[target_cup as usize] = cup1;
        cups[cup3 as usize] = next_after_target_cup;
        cups[current_cup as usize] = next_cup;
        current_cup = next_cup;
    }
}

fn solve1(input: &str) {
    let mut cups = build_cups_mapping(input, input.len());
    let first_cup = input.chars().next().unwrap() as u32 - '1' as u32;
    iterate(&mut cups, first_cup, 100);

    // Build the result as a label.
    let mut label = Vec::new();
    let mut cup = cups[0];
    while cup != 0 {
        label.push(cup as u8 + '1' as u8);
        cup = cups[cup as usize];
    }
    println!("Result: {}", String::from_utf8(label).unwrap());
}

fn solve2(input: &str) {
    let mut cups = build_cups_mapping(input, 1000000);
    let first_cup = input.chars().next().unwrap() as u32 - '1' as u32;
    iterate(&mut cups, first_cup, 10000000);

    let cup1 = cups[0];
    let cup2 = cups[cup1 as usize];
    let result = (cup1 + 1) * (cup2 + 1);
    println!("Result: {}", result);
}

fn main() {
    let input = "739862541";
    solve1(input);
    solve2(input);
}
