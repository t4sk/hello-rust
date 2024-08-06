#![allow(unused)]

use std::convert::{From, Into};

#[derive(Debug)]
struct ComplexNumber {
    x: i32,
    y: i32,
}

// impl Into<ComplexNumber> for (i32, i32) {
//     fn into(self) -> ComplexNumber {
//         ComplexNumber {
//             x: self.0,
//             y: self.1,
//         }
//     }
// }

impl From<(i32, i32)> for ComplexNumber {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

fn main() {
    let c = ComplexNumber::from((1, 2));
    let c: ComplexNumber = (1, 2).into();
    println!("{:?}", c);
}
