fn get_digits(num: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = num;
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }
    digits
}

fn is_non_decreasing(digits: &[u8]) -> bool {
    let mut d0 = 10;
    for d in digits.iter() {
        if d0 < *d {
            return false;
        }
        d0 = *d;
    }
    true
}

fn check_consecutive_duplicates(digits: &[u8], is_standalone_pair: bool) -> bool {
    let mut d0 = 10;
    let mut count = 0;
    for d in digits.iter() {
        if d0 != *d {
            if count >= 2 && (!is_standalone_pair || count == 2) {
                return true;
            }
            count = 0;
        }
        d0 = *d;
        count += 1;
    }
    if count >= 2 && (!is_standalone_pair || count == 2) {
        return true;
    }
    false
}

fn is_password(num: u64, is_standalone_pair: bool) -> bool {
    let digits = get_digits(num);
    digits.len() == 6
        && is_non_decreasing(&digits)
        && check_consecutive_duplicates(&digits, is_standalone_pair)
}

fn solve(is_standalone_pair: bool) -> i32 {
    let mut count = 0;
    for num in 367479..=893698 {
        if is_password(num, is_standalone_pair) {
            count += 1;
        }
    }
    count
}

fn solve1() -> i32 {
    solve(false)
}

fn solve2() -> i32 {
    solve(true)
}

fn main() {
    println!("Result: {}", solve2());
}
