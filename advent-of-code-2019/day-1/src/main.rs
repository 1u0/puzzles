use std::{io, cmp};
use std::io::BufRead;

fn get_required_fuel(mass: i32) -> i32 {
    let mut mass = mass;
    let mut fuel = 0;
    while mass > 0 {
        mass = cmp::max((mass / 3) - 2, 0);
        fuel += mass;
    }
    fuel
}

fn main() {
    let mut total_fuel = 0;
    let mut count = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let module_mass = line.unwrap()
            .parse()
            .unwrap();
        count += 1;
        total_fuel += get_required_fuel(module_mass);
    }

//    let mut fuel_mass = total_fuel;
//    while fuel_mass > 0 {
//        fuel_mass = get_required_fuel(fuel_mass);
//        total_fuel += fuel_mass;
//    }

    println!("Total fuel: {} ({})", total_fuel, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fuel_calculation() {
        assert_eq!(0, get_required_fuel(7));
        assert_eq!(0, get_required_fuel(8));
        assert_eq!(1, get_required_fuel(9));
        assert_eq!(1, get_required_fuel(10));
        assert_eq!(1, get_required_fuel(11));
        assert_eq!(2, get_required_fuel(12));
        assert_eq!(2, get_required_fuel(13));

//        assert_eq!(50346, get_required_fuel(100756));
    }
}
