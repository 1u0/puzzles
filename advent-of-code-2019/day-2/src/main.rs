use std::io;

const OPCODE_ADD: u32 = 1;
const OPCODE_MULTIPLY: u32 = 2;
const OPCODE_STOP: u32 = 99;

struct Code(Vec<u32>);

impl Code {
    fn read_ref(&self, ip: usize) -> Result<u32, &'static str> {
        let pos = self.0[ip] as usize;
        if pos >= self.0.len() {
            Err("Index out of bound")
        } else {
            Ok(self.0[pos])
        }
    }

    fn write_ref(&mut self, ip: usize, value: u32) -> Result<(), &'static str> {
        let pos = self.0[ip] as usize;
        if pos >= self.0.len() {
            Err("Index out of bound")
        } else {
            self.0[pos] = value;
            Ok(())
        }
    }

    fn run(mut self) -> Result<Vec<u32>, &'static str> {
        let mut i: usize = 0;
        let l = self.0.len();
        while i < l {
            match self.0[i] {
                OPCODE_ADD => {
                    if i + 3 >= l {
                        return Err("End of input.");
                    }
                    let val0 = self.read_ref(i + 1)?;
                    let val1 = self.read_ref(i + 2)?;
                    self.write_ref(i + 3, val0 + val1)?;
                    i += 4;
                }
                OPCODE_MULTIPLY => {
                    if i + 3 >= l {
                        return Err("End of input.");
                    }
                    let val0 = self.read_ref(i + 1)?;
                    let val1 = self.read_ref(i + 2)?;
                    self.write_ref(i + 3, val0 * val1)?;
                    i += 4;
                }
                OPCODE_STOP => {
                    break;
                }
                _ => {
                    return Err("Unknown opcode");
                }
            }
        }
        Ok(self.0)
    }
}

fn run_code(code: Vec<u32>) -> Result<Vec<u32>, &'static str> {
    Code(code).run()
}

fn reset(code: &mut Vec<u32>, noun: u32, verb: u32) {
    code[1] = noun;
    code[2] = verb;
}
fn reset_12_02(code: &mut Vec<u32>) {
    reset(code, 12, 2);
}

fn load_code() -> Vec<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect()
}

fn solve1() -> Result<u32, &'static str> {
    let mut code = load_code();
    reset_12_02(&mut code);
    let code = run_code(code)?;
    Ok(code[0])
}

fn solve2() -> Result<u32, &'static str> {
    let mut code = load_code();
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
