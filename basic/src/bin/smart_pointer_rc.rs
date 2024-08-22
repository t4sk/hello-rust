#![allow(unused)]
// RC<T> - reference count
// enable multiple ownership explicitly by using the Rust type Rc<T>,
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // RC<T>
    // Rc::clone doesn't make deep copy
    // Cloning increases reference count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }

    println!("{}", Rc::strong_count(&a));
}
