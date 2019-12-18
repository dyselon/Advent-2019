use std::convert::{TryFrom, TryInto};
use num_enum::TryFromPrimitive;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Instruction {
    Add = 01,
    Multiply = 02,
    Input = 03,
    Output = 04,
    Halt = 99
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AddressMode {
    Absolute,
    Immediate
}

#[derive(Debug, Eq, PartialEq)]
pub struct OpCode {
    pub left: AddressMode,
    pub right: AddressMode,
    pub dest: AddressMode,
    pub instruction: Instruction
}

#[derive(Debug)]
struct OpCodeParseError {
    pub opcode: i64
}

impl Error for OpCodeParseError {}
impl Display for OpCodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse opcode {}", self.opcode)
    }
}

impl TryFrom<i64> for OpCode {
    type Error = Box<dyn Error>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let mut op_remaining = value;
        let inst: Instruction = u8::try_from(op_remaining % 100)?.try_into()?;
        op_remaining = op_remaining / 100;
        let left: AddressMode = u8::try_from(op_remaining % 10)?.try_into()?;
        op_remaining = op_remaining / 10;
        let right: AddressMode = u8::try_from(op_remaining % 10)?.try_into()?;
        op_remaining = op_remaining / 10;
        let dest: AddressMode = u8::try_from(op_remaining % 10)?.try_into()?;
        //let left = 
        Ok(OpCode {
            instruction: inst,
            left: left,
            right: right,
            dest: dest
        })
    }
}

#[cfg(test)]
mod opcode_test {
    use super::*;

    #[test]
    fn opcode_00000_failes() {
        assert!(OpCode::try_from(0).is_err())
    }

    #[test]
    fn opcode_00001_succeeds() {
        assert_eq!(OpCode::try_from(1).unwrap(), OpCode {
            instruction: Instruction::Add,
            left: AddressMode::Absolute,
            right: AddressMode::Absolute,
            dest: AddressMode::Absolute
        })
    }

    #[test]
    fn opcode_00101_succeeds() {
        assert_eq!(OpCode::try_from(101).unwrap(), OpCode {
            instruction: Instruction::Add,
            left: AddressMode::Immediate,
            right: AddressMode::Absolute,
            dest: AddressMode::Absolute
        })
    }

    #[test]
    fn opcode_11101_succeeds() {
        assert_eq!(OpCode::try_from(11101).unwrap(), OpCode {
            instruction: Instruction::Add,
            left: AddressMode::Immediate,
            right: AddressMode::Immediate,
            dest: AddressMode::Immediate
        })
    }

    #[test]
    fn opcode_11102_succeeds() {
        assert_eq!(OpCode::try_from(11102).unwrap(), OpCode {
            instruction: Instruction::Multiply,
            left: AddressMode::Immediate,
            right: AddressMode::Immediate,
            dest: AddressMode::Immediate
        })
    }
}