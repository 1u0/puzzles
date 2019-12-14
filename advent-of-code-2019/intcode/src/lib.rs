use std::io;

pub type Byte = i64;

pub trait Io {
    fn input(&mut self) -> Byte;
    fn output(&mut self, value: Byte);
}

enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    UpdateRelativeBase,
    Stop,
}

impl Opcode {
    fn from_code(code: Byte) -> Opcode {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::UpdateRelativeBase,
            99 => Opcode::Stop,
            _ => panic!("Unknown opcode: {}", code),
        }
    }
}

// Parameter mode
#[derive(PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Modes {
    value: Byte,
}

impl Modes {
    fn new(value: Byte) -> Self {
        Modes { value }
    }

    fn next(&mut self) -> Mode {
        let code = self.value % 10;
        self.value /= 10;
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode: {}", code),
        }
    }
}

fn parse_instruction(instruction: Byte) -> (Opcode, Modes) {
    (
        Opcode::from_code(instruction % 100),
        Modes::new(instruction / 100),
    )
}

type RuntimeError = &'static str;

pub struct Intcode {
    code: Vec<Byte>,
    relative_base: Byte,
}

impl Intcode {
    fn new(code: Vec<Byte>) -> Self {
        Intcode {
            code,
            relative_base: 0,
        }
    }

    fn ensure_memory(&mut self, i: usize) {
        let min_len = i + 1;
        if min_len > self.code.len() {
            println!(
                "Resizing the memory from {} to {}",
                self.code.len(),
                min_len
            );
            self.code.resize(min_len, 0);
        }
    }

    fn is_valid_index(&self, i: Byte) -> bool {
        0 <= i
    }

    fn get_pos(&mut self, ip: usize, mode: Mode) -> Result<usize, RuntimeError> {
        match mode {
            Mode::Immediate => {
                self.ensure_memory(ip);
                Ok(ip)
            }
            Mode::Position | Mode::Relative => {
                self.ensure_memory(ip);
                let mut pos = self.code[ip];
                if mode == Mode::Relative {
                    pos += self.relative_base;
                }
                if !self.is_valid_index(pos) {
                    Err("Index out of bound")
                } else {
                    Ok(pos as usize)
                }
            }
        }
    }

    fn read_ref(&mut self, ip: usize, mode: Mode) -> Result<Byte, RuntimeError> {
        let pos = self.get_pos(ip, mode)?;
        self.ensure_memory(pos);
        Ok(self.code[pos])
    }

    fn write_ref(&mut self, ip: usize, mode: Mode, value: Byte) -> Result<(), RuntimeError> {
        let pos = self.get_pos(ip, mode)?;
        self.ensure_memory(pos);
        self.code[pos] = value;
        Ok(())
    }

    pub fn run(mut self, io: &mut dyn Io) -> Result<Vec<Byte>, RuntimeError> {
        let mut i: usize = 0;
        loop {
            self.ensure_memory(i);
            let (opcode, mut modes) = parse_instruction(self.code[i]);
            match opcode {
                Opcode::Add => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    self.write_ref(i + 3, modes.next(), val0 + val1)?;
                    i += 4;
                }
                Opcode::Multiply => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    let val1 = self.read_ref(i + 2, modes.next())?;
                    self.write_ref(i + 3, modes.next(), val0 * val1)?;
                    i += 4;
                }
                Opcode::Input => {
                    let input = io.input();
                    self.write_ref(i + 1, modes.next(), input)?;
                    i += 2;
                }
                Opcode::Output => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    io.output(val0);
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
                Opcode::UpdateRelativeBase => {
                    let val0 = self.read_ref(i + 1, modes.next())?;
                    self.relative_base += val0;
                    i += 2;
                }
                Opcode::Stop => {
                    return Ok(self.code);
                }
            }
        }
    }
}

pub fn run_code(code: Vec<Byte>, io: &mut dyn Io) -> Result<Vec<Byte>, RuntimeError> {
    Intcode::new(code).run(io)
}

struct NoIo {}

impl Io for NoIo {
    fn input(&mut self) -> Byte {
        panic!("unexpected input request")
    }

    fn output(&mut self, _: Byte) {
        panic!("unexpected output request");
    }
}

pub fn run_code_without_io(code: Vec<Byte>) -> Result<Vec<Byte>, RuntimeError> {
    let mut io = NoIo {};
    run_code(code, &mut io)
}

struct SimpleIo {
    input: Vec<Byte>,
    output: Vec<Byte>,
}

impl SimpleIo {
    fn new(input: Vec<Byte>) -> Self {
        SimpleIo {
            input,
            output: Vec::new(),
        }
    }
}

impl Io for SimpleIo {
    fn input(&mut self) -> Byte {
        self.input.pop().unwrap()
    }

    fn output(&mut self, value: Byte) {
        self.output.push(value);
    }
}

pub fn run_code_with_inputs(code: Vec<Byte>, inputs: Vec<Byte>) -> Vec<Byte> {
    let mut io = SimpleIo::new(inputs);
    let ok = run_code(code, &mut io).is_ok();
    assert!(ok);
    io.output
}

pub fn parse_code(input: &str) -> Vec<Byte> {
    input
        .trim()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect()
}

pub fn load_code() -> Vec<Byte> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    parse_code(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_run_code() {
        assert_eq!(
            Ok(vec![2, 0, 0, 0, 99]),
            run_code_without_io(vec![1, 0, 0, 0, 99])
        );
        assert_eq!(
            Ok(vec![2, 3, 0, 6, 99]),
            run_code_without_io(vec![2, 3, 0, 3, 99])
        );
        assert_eq!(
            Ok(vec![2, 4, 4, 5, 99, 9801]),
            run_code_without_io(vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            Ok(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            run_code_without_io(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }

    #[test]
    fn check_run_code_with_immediate_mode() {
        assert_eq!(
            Ok(vec![1002, 4, 3, 4, 99]),
            run_code_without_io(vec![1002, 4, 3, 4, 33])
        );
    }

    fn test_run(code: &Vec<Byte>, input: Vec<Byte>) -> Vec<Byte> {
        run_code_with_inputs(code.to_vec(), input)
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

    #[test]
    fn test_big_numbers_support() {
        let code = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert_eq!(vec![1219070632396864], test_run(&code, Vec::new()));
        let code = vec![104, 1125899906842624, 99];
        assert_eq!(vec![code[1]], test_run(&code, Vec::new()));
    }

    #[test]
    fn test_a_quine_program() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(code, test_run(&code, Vec::new()));
    }
}
