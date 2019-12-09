use intcode::Byte;

fn input_func() -> Byte {
    panic!("unexpected input request");
}

fn output_func(_value: Byte) {
    panic!("unexpected output request");
}

fn reset(code: &mut Vec<Byte>, noun: Byte, verb: Byte) {
    code[1] = noun;
    code[2] = verb;
}

fn run_code(code: Vec<Byte>) -> Result<Vec<Byte>, &'static str> {
    intcode::run_code(code, &mut input_func, &mut output_func)
}

fn solve1() -> Result<Byte, &'static str> {
    let mut code = intcode::load_code();
    reset(&mut code, 12, 2);
    let code = run_code(code)?;
    Ok(code[0])
}

fn solve2() -> Result<Byte, &'static str> {
    let mut code = intcode::load_code();
    for noun in 0..100 {
        for verb in 0..100 {
            reset(&mut code, noun, verb);
            match run_code(code.to_vec()) {
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
