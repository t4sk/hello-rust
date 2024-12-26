#![allow(unused)]

fn take_string(s: String) {}
fn borrow_string(s: &String) {}

// This will not compile - str size not known at compile time
// fn take_str(s: str) {}
fn borrow_str(s: &str) {}

fn main() {
    // String
    // &String
    // &mut String
    // str
    // &str
    // &mut str

    // String
    // pub struct String {
    //     vec: Vec<u8>,
    // }
    // - Owned, mutable, growable
    // - Allocated on the heap
    // - &String can be coerced into &str

    // String
    let s = String::from("hello");
    // Moves ownership into take_string
    take_string(s);
    // This won't compile
    // println!("String {s}");

    // Mutable
    let mut s = String::from("hello");
    s += "!";

    // &String
    let s = String::from("hello");
    let s1: &String = &s;
    // Borrow
    borrow_string(s1);
    // Can be coerced into &str
    borrow_str(&s);
    println!("&String {s}");

    // &mut String
    let mut s = String::from("hello");
    let s1: &mut String = &mut s;

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size not known at compile time
    // - Rust needs to know how much memory to allocate for each type at compile time

    // Cannot create a variable of type str or use for function input
    // let s: str = "hello";

    // &str
    // - size known at compile time (pointer)
    // - immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("&str {s}");

    // &mut str
    // Possible to create &mut str but uncommon. Use mut String.
    let mut s: String = "hello".to_string();
    let s1: &mut str = &mut s;
}
