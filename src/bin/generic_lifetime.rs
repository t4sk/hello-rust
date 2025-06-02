#![allow(unused)]

// Every reference has a lifetime

// Both x and y live at least 'a
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Multiple lifetime
fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("{} {}", x, y);
}

// Must return correct lifetime
fn f_out<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // Cannot return y since lifetime of return type is 'a
    x
}

// Elision - Rust figures out the lifetime
fn no_need_to_declare_lifetime(x: &str) {
    println!("{}", x);
}

// Struct example
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    let x = "Hello".to_string();
    // This will not compile (z lives longer than y)
    /*
    let z = {
        let y = "Rust".to_string();
        longest_str(&x, &y)
    };
    println!("longest {:?}", z);
    */

    // This compiles (z lives atleast as long as both x and y)
    let y = "Rust".to_string();
    let z = longest_str(&x, &y);
    println!("longest {:?}", z);

    // Static lifetime
    let s: &'static str = "Hello";

    // Placeholder lifetime - let Rust infer the lifetime
    let s: &'_ str = "Rust";

    // Book
    let mut book = Book { title: "Rust" };
    book.edit("Solidity");
    println!("book: {:?}", book);
}
