#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    // Implicit return, no semicolon
    x + y
}

// No output
fn print_name(name: String) {
    println!("My name is {}", name);
}

// Diverge - never return
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("error");
}

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = add(x, y);
    println!("{} + {} = {}", x, y, z);

    print_name("Rust".to_string());
}
