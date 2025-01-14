#![allow(unused)]

// Vector
fn main() {
    // Vec<T>
    let v: Vec<i32> = vec![1, 2, 3];

    // Vec<u8>
    let v = vec![1u8, 2, 3, 4];
    // Initialize with all 5 elements equal to 1
    let v = vec![1u8; 5];
    println!("vec = {:?}, length = {}", v, v.len());

    // Get
    let x = v[1];
    println!("{}", x);

    let x = v.get(1);
    match x {
        Some(val) => println!("get: {:?}", val),
        None => println!("get: value doesn't exist"),
    }

    // Update
    let mut v = vec![1, 2, 3];
    v[1] = 10;

    // push - append to vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("vec = {:?}", v);

    // pop - remove last element
    let mut v = vec![1, 2];
    match v.pop() {
        Some(val) => println!("pop: {val}"),
        None => println!("pop: none"),
    }

    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..3];
    println!("slice = {:?}", s);
}
