use std::error::Error;
use std::convert::TryInto;
use super::opcode::{OpCode, Instruction, AddressMode};

pub struct VM {
    pub memory: Vec<i64>,

    pub ip: usize,
    pub halted: bool,

    pub input: Vec<i64>,
    pub input_index: usize,
    pub output: Vec<i64>
}

impl VM {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while !self.halted && self.ip < self.memory.len() {
            self.step()?;
        }
        Ok( () )
    }

    // TODO: write tests for this!
    fn read_operand(&mut self, mode: AddressMode) -> i64 {
        let index: usize = self.ip as usize;
        self.ip += 1;
        match mode {
            AddressMode::Immediate => self.memory[index],
            AddressMode::Absolute => self.memory[self.memory[index] as usize]
        }
    }

    fn write_operand(&mut self, value: i64) {
        let index: usize = self.ip;
        self.ip += 1;
        let dest_index: usize = self.memory[index] as usize;
        self.memory[dest_index] = value;
    }

    pub fn step(&mut self) -> Result<(), Box<dyn Error>> {
        // read opcode
        let opcode: OpCode = self.memory[self.ip].try_into()?;
        self.ip += 1;
        // dispatch
        
        match opcode.instruction {
            Instruction::Add => {
                let left = self.read_operand(opcode.left);
                let right = self.read_operand(opcode.right);
                self.write_operand(left + right);
            }
            Instruction::Multiply => {
                let left = self.read_operand(opcode.left);
                let right = self.read_operand(opcode.right);
                self.write_operand(left * right);
            }
            Instruction::Input => {
                if self.input_index >= self.input.len() {
                    panic!("Throw a real error here")
                }
                self.write_operand(self.input[self.input_index]);
                self.input_index += 1;
            }
            Instruction::Output => {
                let val = self.read_operand(opcode.left);
                self.output.push(val);
            }
            Instruction::Halt => self.halted = true
        }


        Ok( () )
    }

    pub fn new(memory: &[i64], input: &[i64]) -> Self {
        VM {
            halted: false,
            ip: 0,
            memory: memory.to_vec(),
            input_index: 0,
            input: input.to_vec(), // TODO: Fix input to be a real iterator
            output: Vec::<i64>::new()
        }
    }
}