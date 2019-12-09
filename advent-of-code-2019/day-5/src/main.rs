use intcode::Byte;
use std::io;

fn load_code() -> Vec<Byte> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    intcode::parse_code(&input)
}

fn run_code(code: Vec<Byte>, inputs: Vec<Byte>) -> Vec<Byte> {
    let mut inputs = inputs;
    let mut outputs = Vec::new();
    assert!(intcode::run_code(
        code,
        &mut || {
            let input = inputs.pop().unwrap();
            println!(">>> {}", input);
            input
        },
        &mut |value| {
            println!("#: {}", value);
            outputs.push(value);
        }
    )
    .is_ok());
    outputs
}

fn solve1() {
    let outputs = run_code(load_code(), vec![1]);
    println!("Result: {:?}", outputs.last());
}

fn solve2() {
    let outputs = run_code(load_code(), vec![5]);
    println!("Result: {:?}", outputs.last());
}

fn main() {
    solve2();
}
