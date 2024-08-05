#![allow(unused)]

fn main() {
    // Tuples - fixed size, known at compile time
    let tup: (bool, char, u32) = (true, 'c', 3);
    println!("tup 0 = {}", tup.0);
    println!("tup 1 = {}", tup.1);
    println!("tup 2 = {}", tup.2);

    let empty_tup = ();
}
