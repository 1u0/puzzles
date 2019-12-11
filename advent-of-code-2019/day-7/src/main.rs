use crossbeam_channel::{unbounded, Receiver, Sender};
use intcode::Byte;
use permutohedron::Heap;
use std::cmp;
use std::thread::JoinHandle;

struct ConcurrentIo {
    input: Receiver<Byte>,
    output: Sender<Byte>,
}

impl ConcurrentIo {
    fn new(input: Receiver<Byte>, output: Sender<Byte>) -> Self {
        ConcurrentIo { input, output }
    }
}

impl intcode::Io for ConcurrentIo {
    fn input(&mut self) -> Byte {
        self.input.recv().unwrap()
    }

    fn output(&mut self, value: Byte) {
        self.output.send(value).unwrap();
    }
}

fn run_circuit(code: &Vec<Byte>, phase_setting: &Vec<Byte>) -> Byte {
    let mut input = vec![0];
    for phase in phase_setting {
        input.push(*phase);
        input = intcode::run_code_with_inputs(code.to_vec(), input);
        assert_eq!(input.len(), 1);
    }
    input[0]
}

fn run_amp(code: Vec<Byte>, input: &Receiver<Byte>, output: &Sender<Byte>) -> JoinHandle<()> {
    let mut io = ConcurrentIo::new(input.clone(), output.clone());
    ::std::thread::spawn(move || {
        assert!(intcode::run_code(code, &mut io).is_ok());
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
    let code = intcode::load_code();
    let mut phases: Vec<Byte> = (0..5).collect();
    let all_phase_settings = Heap::new(&mut phases);
    let mut max_output = Byte::min_value();
    for phase_setting in all_phase_settings {
        max_output = cmp::max(max_output, run_circuit(&code, &phase_setting));
    }
    println!("Result: {}", max_output);
}

fn solve2() {
    let code = intcode::load_code();
    let mut phases: Vec<Byte> = (5..10).collect();
    let all_phase_settings = Heap::new(&mut phases);
    let mut max_output = Byte::min_value();
    for phase_setting in all_phase_settings {
        max_output = cmp::max(max_output, run_circuit_2(&code, &phase_setting));
    }
    println!("Result: {}", max_output);
}

fn main() {
    solve2();
}
