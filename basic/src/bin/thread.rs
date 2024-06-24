use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("spawn {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // when the main thread of a Rust program completes, all spawned threads are shut down
    for i in 1..5 {
        println!("main {i}");
        thread::sleep(Duration::from_millis(1));
    }
}
