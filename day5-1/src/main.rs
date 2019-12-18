mod intcode;
use intcode::vm::VM;
use std::fs;
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let file_contents = fs::read_to_string(path).expect("Couldn't find the file");

    let initial_memory_state: Vec<i64> = file_contents.trim().split(',').map(|line| line.parse::<i64>()).filter_map(Result::ok).collect();

    let mut vm = VM::new(&initial_memory_state, &[1]);
    vm.run().expect("Program failed");
    println!("{:?}", &vm.output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_prg() {
        let mut vm = VM::new(&[1101, 2, 3, 0], &[]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.memory[0] == 5, "1101, 2, 3, 0 => 5");
    }
}
