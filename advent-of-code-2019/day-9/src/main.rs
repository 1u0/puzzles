use intcode::Byte;

fn run_code(code: Vec<Byte>, inputs: Vec<Byte>) -> Vec<Byte> {
    let mut inputs = inputs;
    let mut outputs = Vec::new();
    assert!(
        intcode::run_code(code, &mut || { inputs.pop().unwrap() }, &mut |value| {
            outputs.push(value)
        })
        .is_ok()
    );
    outputs
}

fn main() {
    let code = intcode::load_code();
    println!("Result for task 1: {:?}", run_code(code.to_vec(), vec![1]));
    println!("Result for task 2: {:?}", run_code(code, vec![2]));
}
