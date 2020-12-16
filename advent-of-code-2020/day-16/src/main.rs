use std::collections::HashSet;
use std::io::{self, BufRead};

struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}

fn parse_ranges(string: &str) -> Vec<(i32, i32)> {
    // println!("Debug: {}", string);
    string
        .trim()
        .split(" or ")
        .map(|range| {
            let mut it = range.splitn(2, '-').map(|value| value.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn load_input() -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
    let mut state = 0;
    let mut rules = Vec::new();
    let mut my_ticket = Vec::new();
    let mut tickets = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            state += 1;
        } else {
            let tokens: Vec<&str> = line.splitn(2, ':').collect();
            if tokens.len() == 2 {
                if !tokens[1].is_empty() {
                    assert!(state == 0);
                    rules.push(Rule {
                        name: tokens[0].to_owned(),
                        ranges: parse_ranges(tokens[1]),
                    });
                }
            } else {
                let ticket = line
                    .split(',')
                    .map(|token| token.parse().unwrap())
                    .collect();
                if state == 1 {
                    my_ticket = ticket;
                } else {
                    tickets.push(ticket);
                }
            }
        }
    }
    // tickets.push(my_ticket.clone());
    (rules, my_ticket, tickets)
}

fn is_valid(rule: &Rule, field: i32) -> bool {
    rule.ranges
        .iter()
        .any(|&range| range.0 <= field && field <= range.1)
}

fn is_invalid_value(value: i32, rules: &[Rule]) -> bool {
    rules.iter().all(|rule| !is_valid(rule, value))
}

fn solve1(rules: &[Rule], tickets: &[Vec<i32>]) {
    let result: i64 = tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|&value| is_invalid_value(*value, rules))
                .map(|value| *value as i64)
                .sum::<i64>()
        })
        .sum();
    println!("Result: {}", result);
}

fn find_mapping_impl(
    combinations: &[(String, &HashSet<usize>)],
    i: usize,
    fields_mapping: &mut Vec<usize>,
) -> bool {
    if i >= fields_mapping.len() {
        return true;
    }
    for mapping in combinations[i].1.iter() {
        if fields_mapping[..i].contains(mapping) {
            continue;
        }
        fields_mapping[i] = *mapping;
        if find_mapping_impl(combinations, i + 1, fields_mapping) {
            return true;
        }
    }
    false
}

fn find_mapping(combinations: &[(String, &HashSet<usize>)]) -> Vec<usize> {
    let mut fields_mapping = vec![10000; combinations.len()];
    find_mapping_impl(&combinations, 0, &mut fields_mapping);
    fields_mapping
}

fn solve2(rules: &[Rule], tickets: &[Vec<i32>], my_ticket: &Vec<i32>) {
    let combinations = tickets
        .iter()
        // Remove invalid tickets
        .filter(|ticket| ticket.iter().all(|&value| !is_invalid_value(value, rules)))
        // For a valid ticket, for each rule, collect positions where a rule can be applied
        // without braking validation.
        .map(|ticket| {
            rules
                .iter()
                .map(|rule| {
                    ticket
                        .iter()
                        .enumerate()
                        .filter(|entry| is_valid(rule, *entry.1))
                        .map(|entry| entry.0)
                        .collect::<HashSet<usize>>()
                })
                .collect()
        })
        // Aggregate the results:
        //  for each rule, keep positions where a rule can be applied for all valid tickets.
        // TODO: use `fold_first`, aka `reduce` when the api is stabilised.
        .fold(Vec::new(), |acc, x| {
            if acc.is_empty() {
                x
            } else {
                acc.iter()
                    .zip(x)
                    .map(|pair| pair.0.intersection(&pair.1).cloned().collect())
                    .collect()
            }
        });
    let mut combinations = rules
        .iter()
        .map(|rule| rule.name.clone())
        .zip(combinations.iter())
        .collect::<Vec<_>>();
    combinations.sort_by_key(|entry| entry.1.len());
    // map: rule index -> field index
    let fields_mapping = find_mapping(&combinations);

    let result: i64 = combinations
        .iter()
        .enumerate()
        .filter(|entry| entry.1 .0.starts_with("departure"))
        .map(|entry| my_ticket[fields_mapping[entry.0]] as i64)
        .product();
    println!("Result: {}", result);
}

fn main() {
    let input = load_input();
    solve1(&input.0, &input.2);
    solve2(&input.0, &input.2, &input.1);
}
