use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Clone)]
enum Rule {
    Str(String),
    Ref(Vec<i32>),
    Or(Vec<i32>, Vec<i32>),
}

fn load_input() -> (HashMap<i32, Rule>, Vec<String>) {
    fn parse(list: &str) -> Vec<i32> {
        list.split(' ')
            .map(|token| token.parse().unwrap())
            .collect()
    }
    fn parse_rule(rule: &str) -> Rule {
        if rule.starts_with('"') {
            Rule::Str(rule.trim_matches('"').to_owned())
        } else if rule.contains('|') {
            let rules: Vec<&str> = rule.splitn(2, " | ").collect();
            Rule::Or(parse(rules[0]), parse(rules[1]))
        } else {
            Rule::Ref(parse(rule))
        }
    }

    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    let mut state = 0;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if state == 0 {
            if line.is_empty() {
                state = 1;
                continue;
            }
            let tokens: Vec<&str> = line.splitn(2, ": ").collect();
            rules.insert(tokens[0].parse().unwrap(), parse_rule(tokens[1]));
        } else {
            messages.push(line.to_owned());
        }
    }
    (rules, messages)
}

fn matches_rule(rules: &HashMap<i32, Rule>, rule_id: i32, message: &str) -> bool {
    fn matches_rule_impl(
        rules: &HashMap<i32, Rule>,
        message: &str,
        ix: &[usize],
        rule_id: i32,
    ) -> Vec<usize> {
        match rules.get(&rule_id).unwrap() {
            Rule::Str(str) => ix
                .iter()
                .filter(|&i| message[*i..].starts_with(str))
                .map(|i| i + str.len())
                .collect(),
            Rule::Ref(rule_list) => matches_rule_list(rules, message, ix, rule_list),
            Rule::Or(rule_list1, rule_list2) => {
                let mut res = matches_rule_list(rules, message, ix, rule_list1);
                res.extend(matches_rule_list(rules, message, ix, rule_list2));
                res
            }
        }
    }
    fn matches_rule_list(
        rules: &HashMap<i32, Rule>,
        message: &str,
        ix: &[usize],
        list: &[i32],
    ) -> Vec<usize> {
        let mut jx = ix.to_vec();
        for &rule_id in list {
            jx = matches_rule_impl(rules, message, &jx, rule_id);
            if jx.is_empty() {
                break;
            }
        }
        jx
    }

    let result = matches_rule_impl(rules, message, &[0], rule_id);
    result.iter().any(|end| *end == message.len())
}

fn solve1(rules: &HashMap<i32, Rule>, messages: &[String]) {
    let result = messages
        .iter()
        .filter(|message| matches_rule(rules, 0, message))
        .count();
    println!("Result: {}", result);
}

fn solve2(rules: &HashMap<i32, Rule>, messages: &[String]) {
    let mut rules = (*rules).clone();
    rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));
    solve1(&rules, messages);
}

fn main() {
    let (rules, messages) = load_input();
    solve1(&rules, &messages);
    solve2(&rules, &messages);
}
