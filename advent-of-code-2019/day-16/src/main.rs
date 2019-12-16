use std::char;
use std::io;

struct Generator {
    repeat: usize,
    index: usize,
}

impl Generator {
    fn new(repeat: usize) -> Self {
        Generator { repeat, index: 0 }
    }

    fn next(&mut self) -> i32 {
        self.index += 1;
        let i = (self.index / self.repeat) % 4;
        [0, 1, 0, -1][i as usize]
    }
}

fn phase(input: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..input.len() {
        let mut phase = Generator::new(i + 1);
        let prod_sum = input.iter().map(|x| x * phase.next()).sum::<i32>().abs() % 10;
        res.push(prod_sum);
    }
    res
}

// Calculate the phase transform, assuming that the phase matrix is triangular
// (with 1's on the main diagonal and above, 0's below the main diagonal).
fn phase_simplified(input: &[i32]) -> Vec<i32> {
    let mut res = vec![0; input.len()];
    let mut sum = 0;
    for i in (0..input.len()).rev() {
        sum = (sum + input[i]) % 10;
        res[i] = sum;
    }
    res
}

fn load_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn convert_str_to_data(input: &str) -> Vec<i32> {
    input.chars().map(|ch| (ch as i32) - '0' as i32).collect()
}

fn get_prefix_code(data: &[i32]) -> String {
    data.iter()
        .take(8)
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect()
}

fn solve1(input: &str) -> String {
    let mut data = convert_str_to_data(input);
    for _ in 0..100 {
        data = phase(&data);
    }
    get_prefix_code(&data)
}

fn solve2(input: &str) -> String {
    let mut data = convert_str_to_data(&input.repeat(10_000));
    let mut index = 0;
    for d in data.iter().take(7) {
        index *= 10;
        index += d;
    }
    let offset = index as usize;
    // Optimization #1:
    // the calculation of i-th element doesn't use elements before i
    // (because their phase coefficients are 0's).
    // So, we can skip calculation of all elements before the offset (the prefix).
    data.drain(0..offset);

    // Optimization #2:
    // if the remaining part (tail) is less than the offset, the phase multiplier becomes simpler:
    // sequence of 0's and then 1's.
    assert!(
        data.len() < offset,
        "The optimization is implemented only for simple tails..."
    );
    for _ in 0..100 {
        data = phase_simplified(&data);
    }
    get_prefix_code(&data)
}

fn main() {
    let input = load_input();
    let result = solve1(&input);
    println!("Result for task 1: {}", result);
    let result = solve2(&input);
    println!("Result for task 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_1() {
        let mut phase = Generator::new(1, 0);
        for _ in 0..3 {
            assert_eq!(1, phase.next());
            assert_eq!(0, phase.next());
            assert_eq!(-1, phase.next());
            assert_eq!(0, phase.next());
        }
    }

    #[test]
    fn test_phase_2() {
        let mut phase = Generator::new(2, 0);
        for _ in 0..3 {
            assert_eq!(0, phase.next());
            assert_eq!(1, phase.next());
            assert_eq!(1, phase.next());
            assert_eq!(0, phase.next());
            assert_eq!(0, phase.next());
            assert_eq!(-1, phase.next());
            assert_eq!(-1, phase.next());
            assert_eq!(0, phase.next());
        }
    }
}
