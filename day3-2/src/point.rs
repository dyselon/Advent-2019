use std::ops;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn dist_raw(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}