#![allow(unused)]

// derive - compiler is capable of providing basic
// implementation for some traits

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("point = {:?}", p);

    let p1 = p.clone();
    println!("clone = {:?}", p1);
}
