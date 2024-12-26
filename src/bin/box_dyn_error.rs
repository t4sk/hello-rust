#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn read(src_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut src_file = File::open(src_path)?;
    let mut data = String::new();
    src_file.read_to_string(&mut data)?;

    let lines: Vec<String> = data.trim().split('\n').map(|s| s.to_string()).collect();

    Ok(lines)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // read returns Result<Vec<String>, std::io::Error>
    let lines = read(&args[1])?;

    let mut sum: i32 = 0;
    for line in lines {
        // parse returns Result<i32, ParseIntError>
        let num: i32 = line.parse()?;
        sum += num;
    }

    println!("{}", sum);

    Ok(())
}
