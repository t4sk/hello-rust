#![allow(unused)]

fn main() {
    // Tuples - fixed size, known at compile time
    let t: (bool, char, u32) = (true, 'c', 3);
    println!("({}, {}, {})", t.0, t.1, t.2);

    // Destructure
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Ignore first and last
    let (_, b, _) = t;

    // Empty tuple
    let empty_tup = ();

    // Nested tuple
    let nested_tuple = (('a', 1.23), ('b', true, 1), ());
}
