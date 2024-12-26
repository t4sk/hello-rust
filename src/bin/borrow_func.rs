#![allow(unused)]

fn take(s: String) {
    println!("take {s}");
}

fn borrow(s: &str) {
    println!("borrow {s}");
}

fn borrow_mut(s: &mut String) {
    s.push_str("!");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_ownership(s: String) -> String {
    println!("length = {}", s.len());
    s
}
// Note - cannot return &str
// This will not compile
// fn print_len_return_ref(s: String) -> &str {
//     println!("length = {}", s.len());
//     &s
// }

fn print_len_borrow(s: &str) {
    println!("length = {}", s.len());
}

fn main() {
    // Take ownership
    let s = String::from("rust");
    take(s);
    // s is dropped after take(s)
    // This will not compile
    // println!("{s}");

    // Borrow -> doesn't move ownership
    let s = String::from("rust");
    // Rust automatically casts &String into &str
    borrow(&s);
    // s is not dropped after borrow(&s) so s can be printed
    println!("{s}");

    // Mutable borrow -> doesn't move ownership
    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");

    // Modify a function in 3 steps
    // 1. Take ownership
    // 2. Returns ownership
    // 3. Borrows

    // Example 1
    // Take ownership
    let s = String::from("rust");
    print_len(s);
    // THis will not compile
    // println!("{s}");

    // Example 2
    // Return ownership
    let s = String::from("rust");
    // Ownership of s is returned
    let s = print_len_return_ownership(s);
    println!("after return {s}");

    // Example 3
    // Borrow
    let s = String::from("rust");
    print_len_borrow(&s);
    println!("after borrow {s}");
}
