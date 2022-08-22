use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let msgs = vec!["|", "/", "-", "\\"];

    loop {
        for msg in msgs.iter() {
            print!("{}\r", msg);
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    }
}
