use std::io;

fn load_code() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    intcode::parse_code(&input)
}

fn solve1() -> Result<(), &'static str> {
    // Run the program and see the last non-0 output as the result.
    let code = load_code();
    let mut inputs = vec![1];
    let code = intcode::run_code(code, &mut || {
        let input = inputs.pop().unwrap();
        println!(">>> {}", input);
        input
    })?;
    Ok(())
}

fn main() {
    match solve1() {
        Err(err) => {
            println!("Error: {:?}", err);
        }
        Ok(res) => {
            println!("Result: {:?}", res);
        }
    }
}
