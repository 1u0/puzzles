fn main() {
    let code = intcode::load_code();
    let outputs = intcode::run_code_with_inputs(code.to_vec(), vec![1]);
    println!("Result for task 1: {:?}", outputs.last());

    let outputs = intcode::run_code_with_inputs(code.to_vec(), vec![5]);
    println!("Result for task 2: {:?}", outputs.last());
}
