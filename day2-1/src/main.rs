use std::fs;
use std::env;

fn main() {
    println!("Hello, world!!!");

    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let file_contents = fs::read_to_string(path).expect("Couldn't find the file");

    let mut byte_vec: Vec<usize> = file_contents.trim().split(',').map(|line| line.parse::<usize>()).filter_map(Result::ok).collect();

    // restoring the program to the 1202 alarm state
    // i don't know what that means but it seems pretty bad
    byte_vec[1] = 12;
    byte_vec[2] = 02;

    let mut ip = 0;

    while ip < byte_vec.len() {
        let opcode = byte_vec[ip];
        if opcode == 99 { break; }
        let left_index = byte_vec[ip+1];
        let left = byte_vec[left_index];
        let right_index = byte_vec[ip+2];
        let right = byte_vec[right_index];
        let op = if opcode == 1 { add } else { mul };
        let op_str = if opcode == 1 { "+" } else { "*" };
        let dest = byte_vec[ip+3];
        let result = op(left, right);
        byte_vec[dest] = result;
        ip += 4;
        println!("{}, {}, {}, {} -> {} {} {} = {}", opcode, left_index, right_index, dest, left, op_str, right, result);

    }
    println!("Result at 0: {}", byte_vec[0]);
}

fn add(left: usize, right: usize) -> usize {
    left + right
}

fn mul(left: usize, right: usize) -> usize {
    left * right
}
