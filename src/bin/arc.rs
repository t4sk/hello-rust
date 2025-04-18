#![allow(unused)]

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

// Arc
// - Atomic reference counter
// - Thread safe reference count
// - Takes ownership of data inside Arc

fn main() {
    // Arc<Mutex> is conceptually a thread safe version of Rc<RefCell>
    let counter = Arc::new(Mutex::new(0));

    let c1 = Arc::clone(&counter);
    let c2 = Arc::clone(&counter);

    let t1 = thread::spawn(move || {
        let mut count = c1.lock().unwrap();
        *count += 1;
    });

    let t2 = thread::spawn(move || {
        let mut count = c2.lock().unwrap();
        *count += 1;
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("{:?}", counter);
}
