#![allow(unused)]

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

// Interior mutability
// - Allows data mutatation even when there are immutable references to that data
// - RefCell
//   - Interior mutability
//   - Run time error when borrowing rules are broken
//   - Single threaded use
//   - RefCell is used in combination with Rc to create a mutable data with shared ownership

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct Node {
    val: u32,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

use crate::List::{Cons, Nil};

fn main() {
    // This will not compile
    // Cannot borrow mut on immutable value
    // let s = String::from("rust");
    // let s1 = &mut s;
    // s1 += "🦀";

    // RefCell
    let s = String::from("rust");
    let r: RefCell<String> = RefCell::new(s);
    {
        let mut r1 = r.borrow_mut();
        *r1 += "🦀";
        // r is borrowed until r1 is dropped
        println!("{:#?}", r);

        // Run time error - already borrowed
        // let mut r2 = r.borrow_mut();

        // r1 is dropped
    }

    // s is owned by r
    println!("{:#?}", r);

    // Example - List
    // 1 -> Nil
    let mut a = Rc::new(Cons(Rc::new(RefCell::new(1)), Rc::new(Nil)));
    // 2 -> a
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // 3 -> a
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Example - traverse to update last element
    let mut curr: &List = &*a;
    while let Cons(v, tail) = curr {
        print!("{} -> ", *(v.borrow()));
        // tail = &Rc<List>
        if let Nil = **tail {
            let mut x = v.borrow_mut();
            *x += 100;
            break;
        }
        // Deref coercion
        // &Rc<List> is automatically coerced into &List
        curr = tail;
    }
    println!("Nil");

    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");

    // Example - update last element without traversing
    let v = Rc::new(RefCell::new(1));
    // 1 -> Nil
    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));
    // 2 -> a
    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    // 3 -> a
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    {
        // Need to create new scope to drop x after update
        let mut x: std::cell::RefMut<'_, i32> = (*v).borrow_mut();
        *x += 100;
    }
    // Same as
    // *(v.borrow_mut()) += 100;

    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");

    // Example - Node
    /*
    #[derive(Debug)
    struct Node<'a> {
        pub val: u32,
        pub neighbors: Vec<&'a Node<'a>>,
    }

    let mut node0 = Node {
        val: 0,
        neighbors: vec![],
    };
    let mut node1 = Node {
        val: 1,
        neighbors: vec![],
    };
    // Does not compile
    // node 0 -> node 1 (immutable borrow of node 1)
    node0.neighbors.push(&node1);
    // node 1 -> node 0 (mutable borrow of node 1)
    node1.neighbors.push(&node0);
    */

    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });

    {
        // node 0 -> node 1
        let mut r0: RefMut<'_, Vec<Rc<Node>>> = node0.neighbors.borrow_mut();
        r0.push(Rc::clone(&node1));

        // node 1 -> node 0
        let mut r1: RefMut<'_, Vec<Rc<Node>>> = node1.neighbors.borrow_mut();
        r1.push(Rc::clone(&node0));
    }

    // Infinite loop - panics
    // println!("{:?}", node0);
}
