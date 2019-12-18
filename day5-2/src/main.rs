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

    let mut vm = VM::new(&initial_memory_state, &[5]);
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

    #[test]
    fn pos_lt_8_true() {
        let mut vm = VM::new(&[3,9,7,9,10,9,4,9,99,-1,8], &[4]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn pos_lt_8_false() {
        let mut vm = VM::new(&[3,9,7,9,10,9,4,9,99,-1,8], &[10]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }
    
    #[test]
    fn imm_eq_8_true() {
        let mut vm = VM::new(&[3,3,1108,-1,8,3,4,3,99], &[8]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn imm_eq_8_false() {
        let mut vm = VM::new(&[3,3,1108,-1,8,3,4,3,99], &[10]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn imm_lt_8_true() {
        let mut vm = VM::new(&[3,3,1107,-1,8,3,4,3,99], &[4]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn imm_lt_8_false() {
        let mut vm = VM::new(&[3,3,1107,-1,8,3,4,3,99], &[10]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }

    #[test]
    fn pos_jmp_0() {
        let mut vm = VM::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }
    #[test]
    fn pos_jmp_1() {
        let mut vm = VM::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }
    

    #[test]
    fn imm_jmp_0() {
        let mut vm = VM::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 0, "1101, 2, 3, 0 => 5");
    }
    #[test]
    fn imm_jmp_1() {
        let mut vm = VM::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1, "1101, 2, 3, 0 => 5");
    }
    
    #[test]
    fn cmp_to_8_lt() {
        let mut vm = VM::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[3]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 999, "1101, 2, 3, 0 => 5");
    }    

    #[test]
    fn cmp_to_8_eq() {
        let mut vm = VM::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[8]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1000, "1101, 2, 3, 0 => 5");
    }    

    #[test]
    fn cmp_to_8_gt() {
        let mut vm = VM::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[10]);
        vm.run().expect("ran");
        println!("{:?}", &vm.memory);
        assert!(vm.output[0] == 1001, "1101, 2, 3, 0 => 5");
    }
}
