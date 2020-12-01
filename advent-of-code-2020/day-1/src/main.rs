use std::io::BufRead;
use std::io;

fn read_numbers() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()

}

fn solve1(sorted_numbers: &[i32], target: i32) -> Result<i32, &'static str> {
    let mut i = 0;
    let mut j = sorted_numbers.len() - 1;
    while i < j {
        let sum = sorted_numbers[i] + sorted_numbers[j];
        if sum == target {
            return Ok(sorted_numbers[i] * sorted_numbers[j]);
        }
        if sum < target {
            i += 1;
        } else { // sum > target
            j -= 1;
        }
    }
    Err("no solution was found")
}

fn solve2(sorted_numbers: &[i32], target: i32) -> Result<i32, &'static str> {
    let mut i = sorted_numbers.len() - 1;
    while 0 < i {
        let n = sorted_numbers[i];
        match solve1(&sorted_numbers[..i], target - n) {
            Err(_err) => {
                i -= 1;
            }
            Ok(res) => {
                return Ok(res * n);
            }
        }
    }
    Err("no solution was found")
}

fn print_result(result: Result<i32, &'static str>) {
    match result {
        Err(err) => {
            println!("Error: {:?}", err);
        }
        Ok(res) => {
            println!("Result: {:?}", res);
        }
    }
}
fn main() {
    let mut numbers = read_numbers();
    numbers.sort();
    print_result(solve1(&numbers, 2020));
    print_result(solve2(&numbers, 2020));
}
