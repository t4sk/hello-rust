use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    // Can also chain ?
    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used on

    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    println!("username {}", username);

    Ok(())
}
