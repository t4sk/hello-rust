#![allow(unused)]

use std::convert::{From, Into};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

/*
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
*/

// (u32, u32) -> Point
impl From<(u32, u32)> for Point {
    fn from(val: (u32, u32)) -> Self {
        Self { x: val.0, y: val.1 }
    }
}

/*
pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}
*/

// u32 -> Point
impl Into<Point> for u32 {
    fn into(self) -> Point {
        Point { x: self, y: self }
    }
}

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;

    let p0 = Point::from((x, y));
    let p1: Point = x.into();

    println!("p0 {:?}", p0);
    println!("p1 {:?}", p1);
}
