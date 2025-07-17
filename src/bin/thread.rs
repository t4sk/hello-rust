#![allow(unused)]

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let t1: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("t1: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    let t2: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("t2: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Waits until handle terminates
    t1.join().unwrap();
    t2.join().unwrap();

    // Return value from thread
    let t: JoinHandle<u32> = thread::spawn(|| {
        return 1;
    });
    let v = t.join().unwrap();
    println!("value: {v}");

    // move
    let v = vec![1, 2, 3];
    // Closure may outlive the main function so transfer ownership of v
    let t = thread::spawn(move || {
        println!("{v:?}");
    });

    // Cannot compile - ownership transferred into closure above
    // println!("{:?}", v);

    t.join().unwrap();

    // Panic
    let t = thread::spawn(|| {
        panic!("💀");
    });

    // This will crash the main thread
    // t.join().unwrap();

    match t.join() {
        Ok(v) => {
            println!("Thread ok: {:?}", v);
        }
        Err(err) => {
            println!("Thread error: {:?}", err);
        }
    }
}
