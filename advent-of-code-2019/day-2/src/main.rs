use std::io;

fn reset(code: &mut Vec<i32>, noun: i32, verb: i32) {
    code[1] = noun;
    code[2] = verb;
}

fn load_code() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    intcode::parse_code(&input)
}

fn solve1() -> Result<i32, &'static str> {
    let mut code = load_code();
    reset(&mut code, 12, 2);
    let code = intcode::run_code(code)?;
    Ok(code[0])
}

fn solve2() -> Result<i32, &'static str> {
    let mut code = load_code();
    for noun in 0..100 {
        for verb in 0..100 {
            reset(&mut code, noun, verb);
            match intcode::run_code(code.to_vec()) {
                Err(_) => (),
                Ok(res) => {
                    if res[0] == 19690720 {
                        return Ok(100 * noun + verb);
                    }
                }
            }
        }
    }
    Err("no solution was found")
}

fn main() {
    match solve2() {
        Err(err) => {
            println!("Error: {:?}", err);
        }
        Ok(res) => {
            println!("Result: {:?}", res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_run_code() {
        assert_eq!(Ok(vec![2, 0, 0, 0, 99]), run_code(vec![1, 0, 0, 0, 99]));
        assert_eq!(Ok(vec![2, 3, 0, 6, 99]), run_code(vec![2, 3, 0, 3, 99]));
        assert_eq!(
            Ok(vec![2, 4, 4, 5, 99, 9801]),
            run_code(vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            Ok(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            run_code(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }
}
