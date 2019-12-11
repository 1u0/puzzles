fn main() {
    let code = intcode::load_code();
    println!(
        "Result for task 1: {:?}",
        intcode::run_code_with_inputs(code.to_vec(), vec![1])
    );
    println!(
        "Result for task 2: {:?}",
        intcode::run_code_with_inputs(code.to_vec(), vec![2])
    );
}
