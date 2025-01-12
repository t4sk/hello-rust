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
struct Book<'a> {
    title: &'a str,
    id: u32,
}

impl<'a> Book<'a> {
    fn edit(&'a mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    // Static lifetime
    let s: &'static str = "Hello";

    // Placeholder lifetime - let Rust infer the lifetime
    let s: &'_ str = "Rust";
}
