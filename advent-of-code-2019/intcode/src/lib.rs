enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl Opcode {
    fn from_code(code: i32) -> Opcode {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Stop,
            _ => panic!("Unknown opcode: {}", code),
        }
    }
}

// Parameter mode
enum Mode {
    Position,
    Immediate,
}

struct Modes {
    value: i32,
}

impl Modes {
    fn new(value: i32) -> Self {
        Modes { value }
    }

    fn next(&mut self) -> Mode {
        let code = self.value % 10;
        self.value /= 10;
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode: {}", code),
        }
    }
}

fn parse_instruction(instruction: i32) -> (Opcode, Modes) {
    (
        Opcode::from_code(instruction % 100),
        Modes::new(instruction / 100),
    )
}

type RuntimeError = &'static str;

pub struct Code(Vec<i32>);

impl Code {
    fn is_valid_index(&self, i: i32) -> bool {
        0 <= i && i < self.0.len() as i32
    }

    fn get_pos(&self, ip: usize, mode: Mode) -> Result<usize, RuntimeError> {
        match mode {
            Mode::Position => {
                let pos = self.0[ip];
                if !self.is_valid_index(pos) {
                    Err("Index out of bound")
                } else {
                    Ok(pos as usize)
                }
            }
            Mode::Immediate => Ok(ip),
        }
    }

    fn read_ref(&self, ip: usize, mode: Mode) -> Result<i32, RuntimeError> {
        let pos = self.get_pos(ip, mode)?;
        Ok(self.0[pos])
    }

    fn write_ref(&mut self, ip: usize, mode: Mode, value: i32) -> Result<(), RuntimeError> {
        let pos = self.get_pos(ip, mode)?;
        self.0[pos] = value;
        Ok(())
    }

    pub fn run(
        mut self,
        input_func: &mut dyn FnMut() -> i32,
        output_func: &mut dyn FnMut(i32),
    ) -> Result<Vec<i32>, RuntimeError> {
        let mut i: usize = 0;
        let l = self.0.len();
        while i < l {
            let (opcode, mut modes) = parse_instruction(self.0[i]);
            match opcode {
                Opcode::Add => {
                    if i + 3 >= l {
                        return Err("End of input.");
                    }
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    self.write_ref(i + 3, modes.next(), val0 + val1)?;
                    i += 4;
                }
                Opcode::Multiply => {
                    if i + 3 >= l {
                        return Err("End of input.");
                    }
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    self.write_ref(i + 3, modes.next(), val0 * val1)?;
                    i += 4;
                }
                Opcode::Input => {
                    let input = input_func();
                    self.write_ref(i + 1, modes.next(), input)?;
                    i += 2;
                }
                Opcode::Output => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    output_func(val0);
                    i += 2;
                }
                Opcode::JumpIfTrue => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    if val0 != 0 {
                        let ip = self.read_ref(i + 2, modes.next())?;
                        if !self.is_valid_index(ip) {
                            return Err("Jump out of boundaries");
                        }
                        i = ip as usize;
                        continue;
                    }
                    i += 3;
                }
                Opcode::JumpIfFalse => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    if val0 == 0 {
                        let ip = self.read_ref(i + 2, modes.next())?;
                        if !self.is_valid_index(ip) {
                            return Err("Jump out of boundaries");
                        }
                        i = ip as usize;
                        continue;
                    }
                    i += 3;
                }
                Opcode::LessThan => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    let res = if val0 < val1 { 1 } else { 0 };
                    self.write_ref(i + 3, modes.next(), res)?;
                    i += 4;
                }
                Opcode::Equals => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    let res = if val0 == val1 { 1 } else { 0 };
                    self.write_ref(i + 3, modes.next(), res)?;
                    i += 4;
                }
                Opcode::Stop => {
                    return Ok(self.0);
                }
            }
        }
        Err("Unexpected program termination")
    }
}

pub fn run_code(
    code: Vec<i32>,
    input_func: &mut dyn FnMut() -> i32,
    output_func: &mut dyn FnMut(i32),
) -> Result<Vec<i32>, RuntimeError> {
    Code(code).run(input_func, output_func)
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

    fn unexpected_input() -> i32 {
        assert!(false, "unexpected input request");
        unreachable!()
    }

    fn unexpected_output(_value: i32) {
        assert!(false, "unexpected output request");
    }

    fn run_code_without_input(code: Vec<i32>) -> Result<Vec<i32>, RuntimeError> {
        run_code(code, &mut unexpected_input, &mut unexpected_output)
    }

    #[test]
    fn check_run_code() {
        assert_eq!(
            Ok(vec![2, 0, 0, 0, 99]),
            run_code_without_input(vec![1, 0, 0, 0, 99])
        );
        assert_eq!(
            Ok(vec![2, 3, 0, 6, 99]),
            run_code_without_input(vec![2, 3, 0, 3, 99])
        );
        assert_eq!(
            Ok(vec![2, 4, 4, 5, 99, 9801]),
            run_code_without_input(vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            Ok(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            run_code_without_input(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }

    #[test]
    fn check_run_code_with_immediate_mode() {
        assert_eq!(
            Ok(vec![1002, 4, 3, 4, 99]),
            run_code_without_input(vec![1002, 4, 3, 4, 33])
        );
    }

    fn test_run(code: &Vec<i32>, input: Vec<i32>) -> Vec<i32> {
        let mut input = input;
        let mut output = Vec::new();
        assert!(run_code(
            code.to_vec(),
            &mut || { input.pop().unwrap() },
            &mut |value| {
                output.push(value);
            }
        )
        .is_ok());
        output
    }

    #[test]
    fn test_jump() {
        // The programs: ask for input, if input == 0, output 0; else output 1
        let codes = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], // position mode
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],         // position mode
        ];
        for code in codes.iter() {
            assert_eq!(vec![0], test_run(code, vec![0])); // == 0
            assert_eq!(vec![1], test_run(code, vec![42])); // any value != 0
        }
    }

    #[test]
    fn test_less_than() {
        // The programs: ask for input, if input is < 8, output 1; else output 0
        let codes = vec![
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], // position mode
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],     // immediate mode
        ];
        for code in codes.iter() {
            assert_eq!(vec![1], test_run(code, vec![7])); // any value < 8
            assert_eq!(vec![0], test_run(code, vec![8])); // any value >= 8
        }
    }

    #[test]
    fn test_equal_to() {
        // The programs: ask for input, if input is == 8, output 1 else output 0
        let codes = vec![
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], // position mode
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],     // immediate mode
        ];
        for code in codes.iter() {
            assert_eq!(vec![1], test_run(code, vec![8])); // == 8
            assert_eq!(vec![0], test_run(code, vec![42])); // any value != 8
        }
    }
}
