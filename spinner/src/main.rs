// use std::io::{self, Write};
use std::{
    io::{stderr, stdout, Result, Write},
    time::Instant,
};

use std::{thread, time};

fn main() {
    let frames = vec!["-", "\\", "|", "/"];

    let mut stream = std::io::stdout();

    loop {
        for f in frames.iter() {
            // print!("\r{}", f);
            // io::stdout().flush().unwrap();
            write!(stream, "\r{}", f).unwrap();
            stream.flush().unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}
