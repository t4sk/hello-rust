#![allow(unused)]

// Memory
// Stack
// - Stores data of fixed size at compile time
// - Fast
// - LIFO
// Heap
// - Stores data of unknown size at compile time
// - Slower than stack
// - Data managed by ownership and borrowing rules

// Ownership rules
// 1. Each value has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn take(s: String) {
    println!("take {s}");
    // s is dropped
}

fn copy(v: i32) {
    println!("copy {v}");
    // v is dropped here
}

fn main() {
    // 1. Each value has an owner
    // Owner of s is s
    let s = String::from("rust");
    // Owner of i is i
    let i = 1;

    // 2. There can only be one owner at a time
    let s = String::from("dog");
    // Owner of s is s1
    let s1 = s;
    // Owner of s is s2
    let s2 = s1;
    // This will not compile
    // println!("{s}");
    println!("{s2}");

    // Ownnership doesn't move for types that implement the Copy trait.
    // Values are copied, separate owners for i, i1 an i2
    // Owner of i is i
    let i = 1;
    // Owner of i1 is i1
    let i1 = i;
    // Owner of i2 is i2
    let i2 = i1;
    println!("i = {i}, i1 = {i1}, i2 = {i2}");

    // 3. When the owner goes out of scope, the value will be dropped
    let s = String::from("cat");
    {
        s;
    }
    // This will not compile
    // println!("{s}");

    let s = String::from("cat");
    {
        // Owner of s is s1
        let s1 = s;
        println!("{s1}");
        // s1 is dropped
    }
    // This will not compile
    // println!("{s}");

    let s = String::from("cat");
    take(s);
    // This will not compile
    // println!("{s}");

    let i = 1;
    // i is copied as function input
    copy(i);
    // i is not dropped so this compiles
    println!("i = {i}");
}
