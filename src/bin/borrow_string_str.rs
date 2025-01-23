#![allow(unused)]

// String and str

// String
fn take_string(s: String) {}
fn borrow_string(s: &String) {}
fn make_string() -> String {
    "".to_string()
}

// str
fn borrow_str(s: &str) {}

// Why can't you use str for function inputs and outputs - str size not known at compile time
// fn take_str(s: str) {}
// fn make_str() -> str {
//     ""
// }

// How about a function that returns &str?
// Reference outlives value s
// fn make_str() -> &str {
//     let s = "";
//     s
// }

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
    // println!("String: {s}");

    // Mutable
    let mut s = String::from("hello");
    s += "!";
    println!("mut String: {s}");

    // &String
    let s = String::from("hello");
    let s1: &String = &s;
    // Borrow
    borrow_string(s1);
    // Can be coerced into &str
    borrow_str(&s);
    println!("&String: {s}");

    // &mut String
    let mut s = String::from("hello");
    let s1: &mut String = &mut s;
    s1.push_str("!");
    println!("&mut String: {s}");

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size of the type not known at compile time
    // - Compiler needs to know the size of each type
    // Example - size of type cannot be determined, strings have different lengths
    // let a: str = "hello";
    // let b: str = "hello rust";

    // Cannot create a variable of type str or use for function input
    // let s: str = "hello";

    // &str
    // - size known at compile time (pointer)
    // - immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("&str: {s}");

    // &mut str
    // Possible to create &mut str but uncommon. Use mut String.
    let mut s: String = "hello".to_string();
    let s1: &mut str = &mut s;
}
