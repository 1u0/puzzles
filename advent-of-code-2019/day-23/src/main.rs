use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use intcode::Byte;
use std::thread::JoinHandle;

const NAT_ADDR: Byte = 255;

struct Node {
    input_count: i32,
    input: Receiver<Byte>,
    output: Sender<Byte>,
}

impl Node {
    fn new(input: Receiver<Byte>, output: Sender<Byte>) -> Self {
        Node {
            input_count: 1,
            input,
            output,
        }
    }
}

impl intcode::Io for Node {
    fn input(&mut self) -> Byte {
        if self.input_count == 0 && self.input.is_empty() {
            -1
        } else {
            self.input_count = (self.input_count + 1) % 2;
            self.input.recv().unwrap()
        }
    }

    fn output(&mut self, value: Byte) {
        self.output.send(value).unwrap();
    }
}

fn run_node(code: Vec<Byte>, input: &Receiver<Byte>, output: &Sender<Byte>) -> JoinHandle<()> {
    let mut io = Node::new(input.clone(), output.clone());
    ::std::thread::spawn(move || {
        assert!(intcode::run_code(code, &mut io).is_ok());
    })
}

fn run_network(program: &[Byte]) {
    let number_of_computer: usize = 50;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for i in 0..number_of_computer {
        let input = unbounded();
        let output = unbounded();
        input.0.send(i as Byte).unwrap();
        run_node(program.to_vec(), &input.1, &output.0);
        outputs.push(output.1);
        inputs.push(input.0);
    }

    let mut output_buffers = vec![Vec::new(); number_of_computer];
    let mut idle_count = 0;
    let mut nat_buffer = Vec::new();
    let mut nat_first_recv_y = false;
    let mut nat_last_sent_y = None;
    loop {
        let mut has_message = false;
        for i in 0..number_of_computer {
            let output = &outputs[i];
            match output.try_recv() {
                Ok(byte) => {
                    has_message = true;
                    let buffer = &mut output_buffers[i];
                    buffer.push(byte);
                    if buffer.len() == 3 {
                        let addr = buffer[0];
                        let x = buffer[1];
                        let y = buffer[2];

                        if addr < number_of_computer as Byte {
                            let input = &inputs[addr as usize];
                            input.send(x).unwrap();
                            input.send(y).unwrap();
                        } else if addr == NAT_ADDR {
                            nat_buffer.clear();
                            nat_buffer.push(x);
                            nat_buffer.push(y);
                            if !nat_first_recv_y {
                                nat_first_recv_y = true;
                                println!("Result for task 1: {}", y);
                            }
                        }
                        buffer.clear();
                    }
                }
                Err(TryRecvError::Empty) => {}
                Err(err) => {
                    println!("Error on output receive ({}): {:?}", i, err);
                }
            }
        }
        if has_message {
            idle_count = 0;
        } else {
            idle_count += 1;
        }
        if idle_count > 100
            && !nat_buffer.is_empty()
            && inputs.iter().all(|input| input.is_empty())
            && output_buffers.iter().all(|input| input.is_empty())
        {
            println!("Sending nat packet {:?}", nat_buffer);
            let x = nat_buffer[0];
            let y = nat_buffer[1];
            if nat_last_sent_y == Some(y) {
                println!("Result for task 2: {}", y);
            }
            nat_last_sent_y = Some(y);
            inputs[0].send(x).unwrap();
            inputs[0].send(y).unwrap();
            nat_buffer.clear();
        }
    }
}

fn main() {
    run_network(&intcode::load_code());
}
