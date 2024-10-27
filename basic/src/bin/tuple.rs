#![allow(unused)]

fn main() {
    // Tuples - fixed size, known at compile time
    let tup: (bool, char, u32) = (true, 'c', 3);
    println!("tup 0 = {}", tup.0);
    println!("tup 1 = {}", tup.1);
    println!("tup 2 = {}", tup.2);

    // Destructure
    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    let empty_tup = ();
    let nested_tuple = (('a', 1.23), ('b', true, 1), ());
}
