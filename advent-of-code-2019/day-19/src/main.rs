use intcode::Byte;
use std::cmp::Ordering;

fn check(program: &Vec<Byte>, pos: (i32, i32)) -> bool {
    let output =
        intcode::run_code_with_inputs(program.to_vec(), vec![pos.1 as Byte, pos.0 as Byte]);
    assert_eq!(1, output.len());
    output[0] == 1
}

fn solve1() {
    let program = intcode::load_code();
    let mut count = 0;
    for i in 0..50i32 {
        for j in 0..50i32 {
            let ch = if check(&program, (i, j)) {
                count += 1;
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
    println!("Result for task 1: {}", count);
}

enum Dir {
    X,
    Y,
}

fn get_range(program: &Vec<Byte>, pos: (i32, i32), dir: Dir) -> i32 {
    let mut pos = pos;
    let mut len = 0;
    loop {
        if check(program, pos) {
            len += 1;
        } else {
            if len != 0 {
                break;
            }
        }
        match dir {
            Dir::X => {
                pos.0 += 1;
            }
            Dir::Y => {
                pos.1 += 1;
            }
        }
    }
    len
}

fn solve2() {
    let program = intcode::load_code();
    let mut pos = (0, 0);
    for i in 0..100 {
        for j in 0..100 {
            if check(&program, (i, j)) {
                pos = (i, j);
            }
        }
    }
    let min_len = 100;
    let mut iter = 0;
    loop {
        if iter % 100 == 0 {
            println!("Current {}: {:?}", iter, pos);
            iter += 1;
        }
        let x_len = get_range(&program, pos, Dir::X);
        let y_len = get_range(&program, pos, Dir::Y);
        if x_len >= min_len && y_len >= min_len {
            break;
        }
        match x_len.cmp(&y_len) {
            Ordering::Less => {
                pos.1 += 1;
            }
            _ => {
                pos.0 += 1;
            }
        }
    }
    println!("Result for task 2: {:?}", pos.0 * 10000 + pos.1);
}

fn main() {
    solve2();
}
