use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    // Error type is io::Error
    println!("username {}", username);

    // Error type is Error(String)
    if username.len() == 0 {
        return Err("no username".into());
    }

    Ok(())
}
