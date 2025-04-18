#![allow(unused)]

use std::thread;

// Scoped thread
// - borrow
// - threads are automatically joined

fn main() {
    // Normal threads cannot borrow
    let msg = "hello".to_string();

    // Must move ownership of msg into thread
    thread::spawn(move || {
        println!("thread: {:?}", msg);
    });

    // This doesn't compile - ownership of msg transferred to thread above
    // println!("main thread: {:?}", msg);

    // Scoped threads can borrow
    let msg = "hello".to_string();

    thread::scope(|scope| {
        println!("scored thread: {:?}", msg);
    });

    println!("main thread: {:?}", msg);

    // Auto join
    let t1 = thread::spawn(|| {
        println!("thread 1");
    });

    let t2 = thread::spawn(|| {
        println!("thread 2");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("scoped thread 1");
        });

        scope.spawn(|| {
            println!("scoped thread 2");
        });
    });

    // Return values from scoped thread
    let (v1, v2) = thread::scope(|scope| {
        let t1 = scope.spawn(|| 1);

        let t2 = scope.spawn(|| 2);

        (t1.join().unwrap(), t2.join().unwrap())
    });

    println!("{} {}", v1, v2);
}
