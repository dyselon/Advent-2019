use std::fs;
use std::env;

fn main() {
    println!("Hello, world!!!");

    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let file_contents = fs::read_to_string(path).expect("Couldn't find the file");

    let initial_memory_state: Vec<usize> = file_contents.trim().split(',').map(|line| line.parse::<usize>()).filter_map(Result::ok).collect();

    let mut nounverb = 0000;
    while run(&initial_memory_state, nounverb) != 19690720 {
        nounverb += 1;
    }
    println!("final noun verb combo: {}", nounverb);
}

fn run(initial_memory: &Vec<usize>, nounverb: usize) -> usize {

    let mut memory = initial_memory.clone();
    // restoring the program to the 1202 alarm state
    // i don't know what that means but it seems pretty bad
    memory[1] = nounverb / 100;
    memory[2] = nounverb % 100;

    let mut ip = 0;

    while ip < memory.len() {
        // get the opcode (or bail if it's HALT)
        let opcode = memory[ip];
        if opcode == 99 { break; }
        let op = if opcode == 1 { add } else { mul };

        // get the operands and calculate a result
        let left = memory[memory[ip+1]];
        let right = memory[memory[ip+2]];
        let result = op(left, right);

        // stash the result in the destination
        let dest = memory[ip+3];
        memory[dest] = result;

        // advance the ip
        ip += 4;
    }
    println!("Result at 0 for {}: {}", nounverb, memory[0]);
    memory[0]
}

fn add(left: usize, right: usize) -> usize {
    left + right
}

fn mul(left: usize, right: usize) -> usize {
    left * right
}
