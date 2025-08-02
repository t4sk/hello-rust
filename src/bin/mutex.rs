#![allow(unused)]

use std::sync::{Mutex, MutexGuard};
use std::thread;

// Mutex
// - Mutual exclusion
// - Allows access to data from 1 thread at a time

fn main() {
    let m: Mutex<i32> = Mutex::new(0);

    {
        let mut val: MutexGuard<'_, i32> = m.lock().unwrap();
        println!("{:?}", m);

        // Trying to acquire the second lock will block this thread
        // let mut val = m.lock().unwrap();
        *val += 1;
        // mutex guard is dropped
    }
    {
        let mut val = m.lock().unwrap();
        *val += 1;
        // mutex guard is dropped
    }
    println!("{:?}", m);

    // Example of Mutex with scoped threads
    thread::scope(|scope| {
        scope.spawn(|| {
            let mut val: MutexGuard<'_, i32> = m.lock().unwrap();
            *val += 1;
        });
        scope.spawn(|| {
            let mut val: MutexGuard<'_, i32> = m.lock().unwrap();
            *val += 1;
        });
    });

    println!("{:?}", m);
}
