#![allow(unused)]

use std::convert::{From, Into};
use std::ops::{Add, Mul};

#[derive(Debug)]
struct Complex {
    x: i32,
    y: i32,
}

// From (i32, i32) to Complex
impl From<(i32, i32)> for Complex {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

// (i32, i32) into Complex
// impl Into<Complex> for (i32, i32) {
//     fn into(self) -> Complex {
//         Complex {
//             x: self.0,
//             y: self.1,
//         }
//     }
// }

impl Add<(i32, i32)> for Complex {
    // TODO: what this?
    type Output = Complex;

    fn add(self, t: (i32, i32)) -> Complex {
        Complex {
            x: self.x + t.0,
            y: self.y + t.1,
        }
    }
}

impl Mul<(i32, i32)> for Complex {
    type Output = Complex;

    fn mul(self, t: (i32, i32)) -> Complex {
        // (a + bi) * (c + di) = (a*c - b*d) + (a*d + b*c)i
        let a = self.x;
        let b = self.y;
        let c = t.0;
        let d = t.1;

        Complex {
            x: a * c - b * d,
            y: a * d + b * c,
        }
    }
}

fn main() {
    let c = Complex::from((1, 2));
    let c: Complex = (1, 2).into();
    println!("{:?}", c);

    let c = Complex::from((1, 2));
    let s = c + (1i32, 2i32);
    println!("{:?}", s);

    let c = Complex::from((1, 2));
    let p = c * (2i32, 3i32);
    println!("{:?}", p);
}
