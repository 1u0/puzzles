use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    InitMask {
        bitmask_0s: u64,
        bitmask_1s: u64,
        bitmask_xs: u64,
    },
    SetMem {
        addr: u64,
        value: u64,
    },
}

fn load_instructions() -> Vec<Instruction> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tokens = line.splitn(2, " = ").collect::<Vec<&str>>();
            match tokens[0] {
                "mask" => {
                    // let mut
                    let mut bitmask_0s = 0;
                    let mut bitmask_1s = 0;
                    let mut bitmask_xs = 0;
                    for ch in tokens[1].chars() {
                        bitmask_0s <<= 1;
                        bitmask_1s <<= 1;
                        bitmask_xs <<= 1;
                        match ch {
                            '0' => bitmask_0s |= 1,
                            '1' => bitmask_1s |= 1,
                            'X' => bitmask_xs |= 1,
                            _ => panic!("wrong input in mask"),
                        }
                    }
                    Instruction::InitMask {
                        bitmask_0s,
                        bitmask_1s,
                        bitmask_xs,
                    }
                }
                _ => {
                    assert!(tokens[0].starts_with("mem["));
                    assert!(tokens[0].ends_with("]"));
                    let addr = tokens[0][4..tokens[0].len() - 1].parse().unwrap();
                    let value = tokens[1].parse().unwrap();
                    Instruction::SetMem { addr, value }
                }
            }
        })
        .collect()
}

fn solve1(instructions: &[Instruction]) {
    let mut mask = (0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::InitMask {
                bitmask_0s,
                bitmask_1s,
                bitmask_xs,
            } => mask = (*bitmask_0s, *bitmask_1s, *bitmask_xs),
            Instruction::SetMem { addr, value } => {
                let masked_value = (value & mask.2) | mask.1;
                memory.insert(*addr, masked_value);
            }
        }
    }
    let sum: u64 = memory.values().sum();
    println!("Result {}", sum);
}

fn solve2(instructions: &[Instruction]) {
    let mut mask = (0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    fn set_memory(
        memory: &mut HashMap<u64, u64>,
        value: u64,
        base_addr: u64,
        bitmask_xs: u64,
        i_mask: u64,
    ) {
        if bitmask_xs == 0 {
            memory.insert(base_addr, value);
            return;
        }
        if bitmask_xs & i_mask != 0 {
            set_memory(
                memory,
                value,
                base_addr | i_mask,
                bitmask_xs & !i_mask,
                i_mask << 1,
            );
            set_memory(memory, value, base_addr, bitmask_xs & !i_mask, i_mask << 1);
        } else {
            set_memory(memory, value, base_addr, bitmask_xs, i_mask << 1);
        }
    }
    for instruction in instructions {
        match instruction {
            Instruction::InitMask {
                bitmask_0s,
                bitmask_1s,
                bitmask_xs,
            } => mask = (*bitmask_0s, *bitmask_1s, *bitmask_xs),
            Instruction::SetMem { addr, value } => {
                set_memory(&mut memory, *value, (mask.0 & addr) | mask.1, mask.2, 1)
            }
        }
    }
    let sum: u64 = memory.values().sum();
    println!("Result {}", sum);
}

fn main() {
    let instructions = load_instructions();
    solve1(&instructions);
    solve2(&instructions);
}
