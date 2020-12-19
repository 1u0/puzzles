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

enum Op {
    Plus,
    Mult,
}

trait NumericEval: Default {
    fn set(&mut self, value: i64);
    fn push_op(&mut self, op: Op, value: i64);
    fn get_result(&self) -> i64;
}

#[derive(Default)]
struct Eval1 {
    result: i64,
}

impl NumericEval for Eval1 {
    fn set(&mut self, value: i64) {
        self.result = value;
    }

    fn push_op(&mut self, op: Op, value: i64) {
        match op {
            Op::Plus => self.result += value,
            Op::Mult => self.result *= value,
        };
    }

    fn get_result(&self) -> i64 {
        self.result
    }
}

#[derive(Default)]
struct Eval2 {
    values: Vec<i64>,
}

impl NumericEval for Eval2 {
    fn set(&mut self, value: i64) {
        assert!(self.values.is_empty());
        self.values.push(value);
    }

    fn push_op(&mut self, op: Op, value: i64) {
        let i = self.values.len() - 1;
        match op {
            Op::Plus => self.values[i] += value,
            Op::Mult => self.values.push(value),
        }
    }

    fn get_result(&self) -> i64 {
        self.values.iter().product()
    }
}

fn eval<State: NumericEval>(expression: &[Token]) -> i64 {
    fn eval_rec<State: NumericEval>(expression: &[Token], i: &mut usize) -> i64 {
        let mut state = State::default();
        let mut last_op = None;
        while *i < expression.len() {
            *i += 1;
            match expression[*i - 1] {
                Token::LPar => {
                    let value = eval_rec::<State>(expression, i);
                    *i += 1;
                    match last_op {
                        Some(op) => {
                            state.push_op(op, value);
                            last_op = None;
                        }
                        None => state.set(value),
                    }
                }
                Token::RPar => {
                    *i -= 1;
                    break;
                }
                Token::Number(value) => match last_op {
                    Some(op) => {
                        state.push_op(op, value);
                        last_op = None;
                    }
                    None => state.set(value),
                },
                Token::Plus => last_op = Some(Op::Plus),
                Token::Mult => last_op = Some(Op::Mult),
            }
        }
        state.get_result()
    }
    let mut i = 0;
    eval_rec::<State>(expression, &mut i)
}

fn solve<State: NumericEval>(expressions: &[Vec<Token>]) {
    let result: i64 = expressions
        .iter()
        .map(|expression| eval::<State>(&expression))
        .sum();
    println!("Result {}", result);
}

fn main() {
    let expressions = load_expressions();
    solve::<Eval1>(&expressions);
    solve::<Eval2>(&expressions);
}
