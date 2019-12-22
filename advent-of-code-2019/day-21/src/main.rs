use intcode::Byte;
use std::char;

fn run_program(main_program: &Vec<Byte>, springscript: &str) -> Vec<Byte> {
    intcode::run_code_with_inputs(
        main_program.to_vec(),
        springscript.chars().rev().map(|ch| ch as Byte).collect(),
    )
}

fn display_output(output: &Vec<Byte>) {
    for v in output.iter() {
        if let Some(ch) = char::from_u32(*v as u32) {
            print!("{}", ch);
        } else {
            print!("{}", v);
        }
    }
    println!();
}

fn run(script: &str) {
    let main_program = intcode::load_code();
    let output = run_program(&main_program, script);
    display_output(&output);
}

fn solve1() {
    // The script: jump if there is any hole at A or B or C and D is not a hole:
    // (not A || not B || not C) && D === not (A and B and C) and D
    run("\
NOT T T
AND A T
AND B T
AND C T
NOT T J
AND D J
WALK
");
}

fn solve2() {
    // The modified script: jump if there is any hole at A, B or C and D is not a hole and
    // E (next step after D) or H (next jump after D) is not a hole (this way we have an additional
    // next move)
    run("\
OR E J
OR H J
AND D J
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND T J
RUN
");
}

fn main() {
    solve2();
}
