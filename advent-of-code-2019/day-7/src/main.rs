use crossbeam_channel::{unbounded, Receiver, Sender};
use permutohedron::Heap;
use std::cmp;
use std::io;
use std::thread::JoinHandle;
use intcode::Byte;

fn load_code() -> Vec<Byte> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    intcode::parse_code(&input)
}

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

fn run_circuit(code: &Vec<Byte>, phase_setting: &Vec<Byte>) -> Byte {
    let mut input = vec![0];
    for phase in phase_setting {
        input.push(*phase);
        input = run_code(code.to_vec(), input);
        assert_eq!(input.len(), 1);
    }
    input[0]
}

fn run_amp(code: Vec<Byte>, input: &Receiver<Byte>, output: &Sender<Byte>) -> JoinHandle<()> {
    let input = input.clone();
    let output = output.clone();
    ::std::thread::spawn(move || {
        assert!(
            intcode::run_code(code, &mut || { input.recv().unwrap() }, &mut |value| {
                output.send(value).unwrap()
            })
            .is_ok()
        );
    })
}

fn run_circuit_2(code: &Vec<Byte>, phase_setting: &Vec<Byte>) -> Byte {
    let n = phase_setting.len();
    let mut channels = Vec::new();
    for i in 0..n {
        let channel = unbounded();
        channel.0.send(phase_setting[i]).unwrap();
        if i == 0 {
            channel.0.send(0).unwrap();
        }
        channels.push(channel);
    }
    let mut threads = Vec::new();
    for i in 0..n {
        let handle = run_amp(code.to_vec(), &channels[i].1, &channels[(i + 1) % n].0);
        threads.push(handle);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    channels[0].1.recv().unwrap()
}

fn solve1() {
    let code = load_code();
    let mut phases: Vec<i32> = (0..5).collect();
    let all_phase_settings = Heap::new(&mut phases);
    let mut max_output = std::i32::MIN;
    for phase_setting in all_phase_settings {
        max_output = cmp::max(max_output, run_circuit(&code, &phase_setting));
    }
    println!("Result: {}", max_output);
}

fn solve2() {
    let code = load_code();
    let mut phases: Vec<Byte> = (5..10).collect();
    let all_phase_settings = Heap::new(&mut phases);
    let mut max_output = std::i32::MIN;
    for phase_setting in all_phase_settings {
        max_output = cmp::max(max_output, run_circuit_2(&code, &phase_setting));
    }
    println!("Result: {}", max_output);
}

fn main() {
    solve2();
}
