#![allow(unused)]

fn main() {
    // Closures can capture variables by
    // - Borrow immutable reference &T
    // - Borrow mutable reference &mut T
    // - Take ownership of value T

    // Borrow immutable reference
    let s = "hello".to_string();
    let f = || println!("borrow: {}", s);

    f();
    println!("main: {}", s);

    // Borrow mutable
    let mut s = "hello".to_string();
    let mut f = || s += " world";

    f();
    println!("main: mut {}", s);

    // Taking ownership
    // Using `move` forces closure to take ownership of captured variables
    let s = "hello".to_string();
    let f = move || {
        println!("move: {}", s);
        // Force drop on s
        std::mem::drop(s);
    };

    // Cannot call twice - ownership moved into f
    f();
    // Cannot call print - ownership moved into f
    // println!("main {}", s);
}
