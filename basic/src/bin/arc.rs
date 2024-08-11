#![allow(unused)]
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Arc - atomic reference count
fn main() {
    let message = Arc::new("hello");

    for i in 0..10 {
        let message = Arc::clone(&message);
        thread::spawn(move || println!("{} {:?}", i, message));
    }
    thread::sleep(Duration::from_secs(1));
}
