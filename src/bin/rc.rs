#![allow(unused)]

use std::rc::Rc;

// Rc - reference counting
// - Use to share ownership for read only purpose
// - Keeps track of the number of references to the value wrapped in Rc
// - Reference count increases by 1 when Rc is cloned,
//   decreases by 1 when cloned Rc is dropped
// - Cloning a Rc never performs a deep copy
// - Single threaded use

// Box takes ownership, cannot share ownership of List
// Lifetime is tricky to manage
/*
#[derive(Debug)]
enum List<'a> {
    Cons(i32, Box<&'a List<'a>>),
    Nil,
}
*/

// Use Rc to share ownership of List
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    /*
    // Cannot create lists that fork at b
    // Cannot share ownership of b with c and d
    // a = 1 <- nil
    // b = 2 <- a
    // c = 3 <- b
    // d = 4 <- b
    let nil = Nil;
    let a = Cons(1, Box::new(&nil));
    let b = Cons(2, Box::new(&a));
    let c = Cons(3, Box::new(&b));
    let d = Cons(4, Box::new(&b));
    */

    // 2 -> 1 -> Nil
    let a = Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))));
    println!("a: {}", Rc::strong_count(&a));

    // 3 -> a
    let b = Cons(3, Rc::clone(&a));
    // strong_count increases by 1
    println!("b: {}", Rc::strong_count(&a));

    {
        // 4 -> a
        let c = Cons(4, Rc::clone(&a));
        // strong_count increases by 1
        println!("c: {}", Rc::strong_count(&a));
    }

    // strong_count decreases by 1
    println!("drop c: {}", Rc::strong_count(&a));

    // Example - print all values in List
    let mut curr: &List = &b;
    // v: &i32
    // tail: &Rc<List>
    while let Cons(v, tail) = curr {
        print!("{v} -> ");

        // Deref coercion
        // &Rc<List> is automatically coerced into &List
        curr = tail;
        // This also works
        // curr = &**tail;
    }
    println!("Nil");
}
