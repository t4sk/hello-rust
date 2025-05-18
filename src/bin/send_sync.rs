#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

fn main() {
    // Send - Type is safe to move between threads
    // Sync - Type is safe to be referenced between threads
    // If T is Send then &T is Sync

    // Examples of Send
    // Send - String
    // Not Send - Rc (not thread safe)

    // Examples of Sync
    // Sync - Arc, Mutex
    // Not Sync - Rc, RefCell, raw pointers

    // Send - String
    let s = "🦀".to_string();
    thread::spawn(move || {
        println!("Send: {}", s);
    })
    .join()
    .unwrap();

    // Not Send - Rc
    // Doesn't compile - Rc<i32> cannot be sent between threads safely
    // Why is Rc not thread safe?
    // Rc::clone increments its reference count without thread safe operations (locks or atomics)
    /*
    let rc = Rc::new(1);
    thread::spawn(move || {
        println!("Not Send: {:?}", rc);
    });
    */

    // Sync - Arc<T>, T must be also Sync
    // Why is Arc thread safe?
    // It uses atomic operations
    let arc = Arc::new(1);
    thread::spawn(move || {
        println!("Sync : {:?}", arc);
    })
    .join()
    .unwrap();

    // Trick to see if a type is Send and / or Sync
    is_send::<Mutex<i32>>();
    is_sync::<Mutex<i32>>();

    // Not Sync - RefCell
    // RefCell<i32> cannot be shared between threads safely
    /*
    let ref_cell = RefCell::new(1);
    let r = &ref_cell;
    thread::spawn(move || {
        println!("Not Sync : {:?}", r);
    });
    */
}
