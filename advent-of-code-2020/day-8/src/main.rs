use std::io::{self, BufRead};

#[derive(Clone)]
enum Operation {
    Acc { value: i32 },
    Jmp { offset: i32 },
    Nop { value: i32 },
}

fn load_code() -> Vec<Operation> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let cmd_value: Vec<&str> = line.splitn(2, ' ').collect(); // TODO: use split_once()
            match cmd_value[0] {
                "acc" => Operation::Acc {
                    value: cmd_value[1].parse().unwrap(),
                },
                "jmp" => Operation::Jmp {
                    offset: cmd_value[1].parse().unwrap(),
                },
                "nop" => Operation::Nop {
                    value: cmd_value[1].parse().unwrap(),
                },
                _ => panic!("unknown instruction {}", cmd_value[0]),
            }
        })
        .collect()
}

fn run(code: &[Operation]) -> (i32, i32) {
    let mut pos = 0;
    let mut accumulator = 0;
    let mut visited_instructions = vec![false; code.len()];
    while 0 <= pos && pos < code.len() as i32 && !visited_instructions[pos as usize] {
        visited_instructions[pos as usize] = true;
        match code[pos as usize] {
            Operation::Acc { value } => {
                accumulator += value;
                pos += 1;
            }
            Operation::Jmp { offset } => {
                pos += offset;
            }
            Operation::Nop { .. } => {
                pos += 1;
            }
        }
    }
    (pos, accumulator)
}

fn solve1(code: &[Operation]) {
    let result = run(code).1;
    println!("Result: {}", result);
}

fn swap_nop_and_jmp_at_pos(code: &mut Vec<Operation>, i: usize) -> bool {
    match code[i] {
        Operation::Nop { value } => {
            code[i] = Operation::Jmp { offset: value };
            true
        }
        Operation::Jmp { offset } => {
            code[i] = Operation::Nop { value: offset };
            true
        }
        Operation::Acc { .. } => false,
    }
}

fn solve2(code: Vec<Operation>) {
    let mut code = code;
    for i in 0..code.len() {
        if swap_nop_and_jmp_at_pos(&mut code, i) {
            let result = run(&code);
            if result.0 == code.len() as i32 {
                println!("Result: {}", result.1);
                break;
            }
            swap_nop_and_jmp_at_pos(&mut code, i);
        }
    }
}

fn main() {
    // solve1(&vec![
    //     Operation::Nop { value: 0 },
    //     Operation::Acc { value: 1 },
    //     Operation::Jmp { offset: 4 },
    //     Operation::Acc { value: 3 },
    //     Operation::Jmp { offset: -3 },
    //     Operation::Acc { value: -99 },
    //     Operation::Acc { value: 1 },
    //     Operation::Jmp { offset: -4 },
    //     Operation::Acc { value: 6 },
    // ]);
    let code = load_code();
    solve1(&code);
    solve2(code);
}
