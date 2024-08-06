#![allow(unused)]

use std::fmt;

struct ComplexNumber {
    x: i32,
    y: i32,
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

fn main() {
    let c = ComplexNumber { x: 1, y: -1 };
    println!("{}", c);
}
