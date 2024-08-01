#![allow(unused)]
// Smart pointers, on the other hand, are data structures that act like a
// pointer but also have additional metadata and capabilities

// Box<T> store data on the heap

// Box<T> recusive type example
#[derive(Debug)]
struct Tree {
    value: u32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

// RC<T> - reference count
// enable multiple ownership explicitly by using the Rust type Rc<T>,
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Box<T>
    let b = Box::new(5);
    println!("b = {b}");

    // Box<T> recursive type
    let tree = Tree {
        value: 0,
        left: Some(Box::new(Tree {
            value: 1,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Tree {
            value: 2,
            left: None,
            right: None,
        })),
    };

    println!("{:#?}", tree);

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
