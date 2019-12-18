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

    #[test]
    fn pos_eq_8_true() {
        let mut vm = VM::new(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn pos_eq_8_false() {
        let mut vm = VM::new(&[3,9,8,9,10,9,4,9,99,-1,8], &[10]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }
}
