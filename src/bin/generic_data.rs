#![allow(unused)]

// Enum
enum Option<T> {
    Some(T),
    None,
}

// Enum - generic over 2 types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Struct - default type is u32
struct Point<T = u32> {
    x: T,
    y: T,
}

fn main() {
    // Vector
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = vec![-1, 2, -3];
    // Let Rust infer the type
    let v: Vec<_> = vec![-1, 0, 1];

    let p0: Point<u32> = Point { x: 1, y: 0 };
    let p1: Point<f32> = Point { x: 1.23, y: 0.123 };
}
