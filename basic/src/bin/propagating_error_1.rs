use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let file_res = File::open("hello.txt");
    let mut file = match file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => {
            println!("username {}", username);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
