use std::io::BufRead;
use std::{cmp, io};

fn get_required_fuel(mass: i32) -> i32 {
    let mut mass = mass;
    let mut fuel = 0;
    while mass > 0 {
        mass = cmp::max((mass / 3) - 2, 0);
        fuel += mass;
    }
    fuel
}

fn read_modules() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

fn main() {
    let modules = read_modules();
    let mut total_fuel: i64 = 0;
    for module_mass in modules.iter() {
        total_fuel += get_required_fuel(*module_mass) as i64;
    }
    println!("Total fuel: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fuel_calculation() {
        assert_eq!(2, get_required_fuel(14));
        assert_eq!(966, get_required_fuel(1969));
        assert_eq!(50346, get_required_fuel(100756));
    }
}
