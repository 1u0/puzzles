const OPCODE_ADD: i32 = 1;
const OPCODE_MULTIPLY: i32 = 2;
const OPCODE_STOP: i32 = 99;

pub struct Code(Vec<i32>);

impl Code {
    fn read_ref(&self, ip: usize) -> Result<i32, &'static str> {
        let pos = self.0[ip] as usize;
        if pos >= self.0.len() {
            Err("Index out of bound")
        } else {
            Ok(self.0[pos])
        }
    }

    fn write_ref(&mut self, ip: usize, value: i32) -> Result<(), &'static str> {
        let pos = self.0[ip] as usize;
        if pos >= self.0.len() {
            Err("Index out of bound")
        } else {
            self.0[pos] = value;
            Ok(())
        }
    }

    pub fn run(mut self) -> Result<Vec<i32>, &'static str> {
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

pub fn run_code(code: Vec<i32>) -> Result<Vec<i32>, &'static str> {
    Code(code).run()
}

pub fn parse_code(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect()
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
