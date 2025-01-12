#![allow(unused)]

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/*
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p0: Point<f32> = Point { x: 1.0, y: 2.0 };
    let p1: Point<f32> = Point { x: 1.0, y: 3.0 };
    let p2 = p0 + p1;
    println!("{:?}", p2);
}
