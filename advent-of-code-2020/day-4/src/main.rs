use std::collections::HashMap;
use std::io::{self, BufRead};

fn is_valid_attribute_value(field: &str, value: &str) -> bool {
    match field {
        // Birth year
        "byr" => match value.parse() {
            Ok(date) => 1920 <= date && date <= 2002,
            _ => false,
        },
        // Issue year
        "iyr" => match value.parse() {
            Ok(date) => 2010 <= date && date <= 2020,
            _ => false,
        },
        // Expiration year
        "eyr" => match value.parse() {
            Ok(date) => 2020 <= date && date <= 2030,
            _ => false,
        },
        // Height
        "hgt" => {
            if value.ends_with("cm") {
                match value.get(0..value.len() - 2).unwrap().parse() {
                    Ok(height) => 150 <= height && height <= 193,
                    _ => false,
                }
            } else if value.ends_with("in") {
                match value.get(0..value.len() - 2).unwrap().parse() {
                    Ok(height) => 59 <= height && height <= 76,
                    _ => false,
                }
            } else {
                false
            }
        }
        // Hair color
        "hcl" => {
            value.len() == 7
                && value.starts_with('#')
                && value.get(1..).unwrap().chars().all(|ch| match ch {
                    '0'..='9' | 'a'..='f' => true,
                    _ => false,
                })
        }
        // Eye color
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        // Passport ID
        "pid" => value.len() == 9 && value.chars().all(|ch| ch.is_ascii_digit()),
        // Country ID
        "cid" => true,
        _ => false,
    }
}

fn count_valid_passports(validate_attribute: &mut dyn FnMut(&str, &str) -> bool) -> i32 {
    let attribute_code: HashMap<&str, i32> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, attribute)| (attribute, 1 << i))
        .collect();
    let all_set = (1 << attribute_code.len()) - 1;
    let mut count = 0;
    let mut attrs = 0;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            if attrs == all_set {
                count += 1;
            }
            attrs = 0;
            continue;
        }
        for attribute in line.split_whitespace() {
            let (field, value) = attribute.split_at(3); // TODO: use attribute.split_once(':')
            match attribute_code.get(field) {
                Some(code) => {
                    if validate_attribute(field, value.get(1..).unwrap()) {
                        attrs |= code;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    if attrs == all_set {
        count += 1;
    }
    count
}

fn main() {
    // println!("Result: {}", count_valid_passports(&mut |_, _| true));
    println!("Result: {}", count_valid_passports(&mut is_valid_attribute_value));
}
