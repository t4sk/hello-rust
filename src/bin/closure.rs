#![allow(unused)]

// Closures intro
fn main() {
    // Closures can capture variables
    let v = 1;
    // Annotated
    let f = |i: i32| -> i32 { i + v };
    // Inferred
    let f = |i| i + v;

    println!("closure inferred: {}", f(-1));

    // Inferred type is locked
    let f = |x| x;
    f(-1);
    // This will not compile - f is lockedto type i32 -> i32
    // f("hello");

    // Map example
    let vals = vec![1, 2, 3];
    let x = 1;
    println!("{:?}", vals.iter().map(|v| v + x));
}
