#![allow(unused)]

// String = vector of u8 (Vec<u8>) valid UTF-8
// &str = slice of u8 (&[u8]) valid UTF-8

// When to use String vs &str
// String -> mutate or data needs to be owned
// &str -> read only
fn main() {
    // String
    let msg: String = String::from("Hello Rust 🦀");
    let len: usize = msg.len();
    println!("String length = {len}");

    // str
    // - String slice
    // &str
    // - usually use str with reference (borrowed)
    // - immutable
    let msg: String = String::from("Hello Rust 🦀");
    // String slice
    let s: &str = &msg[..5];
    println!("slice = {s}");
    let len: usize = s.len();
    println!("slice length = {len}");

    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of the binary
    // - immutable because hard-coded inside binary
    let hello: &str = "Hello Rust";

    // Multi line string literal
    let s: &str = r#"
        { "a": 1,
          "b": { "c": 2 },
          "d": 3
        }
    "#;
    println!("{s}");

    // Deref coercion
    // Rust automatically dereferences &String into a &str
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg;
    println!("slice = {s}");

    // Add str to string
    let mut msg = "Hello".to_string();
    msg += " Rust";
    println!("{msg}");

    // String interpolation - format!
    let lang = "Rust";
    let emoji = "🦀";
    let msg: String = format!("Hello {} {}", lang, emoji);
    println!("{msg}");
}
