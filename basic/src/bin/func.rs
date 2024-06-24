fn add(x: u32, y: u32) -> u32 {
    // Implicit return, no semicolon
    x + y
}

// No output
fn print_name(name: String) {
    println!("My name is {}", name);
}

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = add(x, y);
    println!("{} + {} = {}", x, y, z);

    print_name("Rusty".to_string());
}
