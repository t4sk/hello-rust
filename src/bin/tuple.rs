#![allow(unused)]

// Function that returns multiple values using a tuple
fn return_many() -> (u32, bool) {
    (1, true)
}

// Function returning no value implicitly returns the unit type `()`
fn no_return() {}

// Equivalent to `no_return`, but explicitly returning the unit type
fn return_empty_tuple() -> () {}

fn main() {
    // Tuples - fixed size, mixed types, known at compile time
    let t: (bool, char, u32) = (true, 'c', 3);
    println!("({}, {}, {})", t.0, t.1, t.2);

    // Destructuring a tuple
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Partial destructuring (ignore first and last values)
    let (_, b, _) = t;

    // Destructuring function return value
    let (u, b) = return_many();

    // Empty tuple = unit type
    // Useful when returning a type with no value (example: Result<(), Error>)
    let empty = ();

    // Nested tuple
    let nested = (('a', 1.23), ('b', true, 1), ());
    println!("nested: {}", (nested.0).1);
}
