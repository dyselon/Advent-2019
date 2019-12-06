use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use super::point::Point;

#[derive(Debug)]
pub struct InstParseError {
    desc: String
}

impl InstParseError {
    fn new(desc: String) -> Box<InstParseError> {
        Box::new(InstParseError { desc: desc })
    }
}

impl Error for InstParseError {
 /// A short description of the error.
  fn description(&self) -> &str { &self.desc }

  /// The lower level cause of this error, if any.
  fn cause(&self) -> Option<&dyn Error> { None }
}

impl fmt::Display for InstParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub dir: Point,
    pub mag: i32
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn Error>;
    fn try_from(str_rep: &str) -> Result<Self, Self::Error> {
        let dir = match str_rep.chars().nth(0) {
            Some('R') => Point { x: 1, y: 0 },
            Some('L') => Point { x: -1, y: 0 },
            Some('U') => Point { x: 0, y: 1 },
            Some('D') => Point { x: 0, y: -1 },
            _ => return Err(InstParseError::new(format!("Invalid direction from instruction {}", str_rep)))
        };
        let mag: i32 = str_rep[1..].parse::<i32>()?;
        Ok(Self {
            dir: dir,
            mag: mag
        })
    }
}