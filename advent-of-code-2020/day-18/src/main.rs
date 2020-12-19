use std::io::{self, BufRead};

#[derive(PartialEq, Debug)]
enum Token {
    LPar,
    RPar,
    Plus,
    Mult,
    Number(i64),
}

fn load_expressions() -> Vec<Vec<Token>> {
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut expression = Vec::new();
        let mut num = -1;
        for ch in line.chars() {
            match ch {
                '(' => expression.push(Token::LPar),
                '+' => expression.push(Token::Plus),
                '*' => expression.push(Token::Mult),
                ')' => {
                    if num != -1 {
                        expression.push(Token::Number(num));
                        num = -1;
                    }
                    expression.push(Token::RPar)
                }
                ' ' => {
                    if num != -1 {
                        expression.push(Token::Number(num));
                        num = -1;
                    }
                }
                '0'..='9' => {
                    if num == -1 {
                        num = 0;
                    }
                    num *= 10;
                    num += ch as i64 - '0' as i64;
                }
                _ => panic!("invalid input"),
            }
        }
        if num != -1 {
            expression.push(Token::Number(num));
        }
        result.push(expression);
    }
    result
}

fn eval1(expression: &[Token]) -> i64 {
    fn eval_rec(expression: &[Token], i: &mut usize) -> i64 {
        let mut result = -1;
        let mut op = None;
        while *i < expression.len() {
            *i += 1;
            match expression[*i - 1] {
                Token::LPar => {
                    let num = eval_rec(expression, i);
                    *i += 1;
                    if result == -1 {
                        result = num;
                    } else {
                        match op {
                            Some(Token::Plus) => result += num,
                            Some(Token::Mult) => result *= num,
                            _ => panic!("invalid state"),
                        }
                        op = None;
                    }
                }
                Token::RPar => {
                    *i -= 1;
                    break;
                }
                Token::Number(num) => {
                    if result == -1 {
                        result = num;
                    } else {
                        match op {
                            Some(Token::Plus) => result += num,
                            Some(Token::Mult) => result *= num,
                            _ => panic!("invalid state"),
                        }
                        op = None;
                    }
                }
                Token::Plus => op = Some(Token::Plus),
                Token::Mult => op = Some(Token::Mult),
            }
        }
        result
    }
    let mut i = 0;
    eval_rec(expression, &mut i)
}

fn solve1(expressions: &[Vec<Token>]) {
    let result: i64 = expressions
        .iter()
        .map(|expression| eval1(&expression))
        .sum();
    println!("Result {}", result);
}

fn eval2(expression: &[Token]) -> i64 {
    fn eval_rec(expression: &[Token], i: &mut usize) -> i64 {
        let mut values = Vec::new();
        let mut op = None;
        while *i < expression.len() {
            *i += 1;
            match expression[*i - 1] {
                Token::LPar => {
                    let num = eval_rec(expression, i);
                    *i += 1;
                    if values.is_empty() {
                        values.push(num);
                    } else {
                        let i = values.len() - 1;
                        match op {
                            Some(Token::Plus) => values[i] += num,
                            Some(Token::Mult) => values.push(num),
                            _ => panic!("invalid state"),
                        }
                        op = None;
                    }
                }
                Token::RPar => {
                    *i -= 1;
                    break;
                }
                Token::Number(num) => {
                    if values.is_empty() {
                        values.push(num);
                    } else {
                        let i = values.len() - 1;
                        match op {
                            Some(Token::Plus) => values[i] += num,
                            Some(Token::Mult) => values.push(num),
                            _ => panic!("invalid state"),
                        }
                        op = None;
                    }
                }
                Token::Plus => op = Some(Token::Plus),
                Token::Mult => op = Some(Token::Mult),
            }
        }
        values.iter().product()
    }
    let mut i = 0;
    eval_rec(expression, &mut i)
}

fn solve2(expressions: &[Vec<Token>]) {
    let result: i64 = expressions
        .iter()
        .map(|expression| eval2(&expression))
        .sum();
    println!("Result {}", result);
}

fn main() {
    let expressions = load_expressions();
    solve1(&expressions);
    solve2(&expressions);
}
