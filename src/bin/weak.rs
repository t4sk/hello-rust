#![allow(unused)]

use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

// Strong reference shares ownership
// Weak reference breaks reference cycles and avoid memory leaks

#[derive(Debug)]

struct Node {
    val: u32,
    neighbors: RefCell<Vec<Weak<Node>>>,
}

fn main() {
    // Strong reference
    // Rc::clone -> strong_count increases
    // r0 is only dropped when strong_count is 0
    let x = "hello".to_string();

    let r0: Rc<String> = Rc::new(x);
    println!("r0 - strong count: {}", Rc::strong_count(&r0));

    let r1 = Rc::clone(&r0);
    println!("r1 - strong count: {}", Rc::strong_count(&r0));

    // Weak refenence
    // Rc::downgrade -> weak_count increases
    // Rc::upgrade -> returns Option<Rc<T>>`
    let w1: Weak<String> = Rc::downgrade(&r0);
    println!("w1 - strong count: {}", Rc::strong_count(&r0));
    println!("w1 - weak count: {}", Rc::weak_count(&r0));

    let w2: Weak<String> = Rc::downgrade(&r0);
    println!("w2 - strong count: {}", Rc::strong_count(&r0));
    println!("w2 - weak count: {}", Rc::weak_count(&r0));

    // Upgrade weak to strong reference
    let u0 = w1.upgrade();
    println!("u0 - upgrade w1: {:?}", u0);
    println!("u0 - strong count: {}", Rc::strong_count(&r0));
    println!("u0 - weak count: {}", Rc::weak_count(&r0));

    // Drop some strong references
    println!("drop u0 and r1");
    std::mem::drop(u0);
    std::mem::drop(r1);

    let u1 = w1.upgrade();
    println!("u1 - upgrade w1: {:?}", u1);
    println!("u1 - strong count: {}", Rc::strong_count(&r0));
    println!("u1 - weak count: {}", Rc::weak_count(&r0));

    // Drop all strong references
    println!("drop u1 and r0");
    std::mem::drop(u1);
    std::mem::drop(r0);

    let u2 = w1.upgrade();
    println!("u2 - upgrade w1: {:?}", u2);

    // Example - Node
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });

    // Create a cycle
    // node 0 -> node 1
    // node 1 -> node 0
    {
        // Rc::clone increments strong_count by 1.
        // Rc<T> cannot be dropped unless strong_count is 0.
        // Since node 0 and node 1 references each other, their strong_count can never reach 0.
        // Hence neither node 0 nor node 1 can ever be dropped.

        // Weak<T> does not increment strong_count.
        // Hence both node 0 and 1 can be dropped.

        // node 0 -> node 1
        let mut r0: RefMut<'_, Vec<Weak<Node>>> = node0.neighbors.borrow_mut();
        r0.push(Rc::downgrade(&node1));

        // node 1 -> node 0
        let mut r1: RefMut<'_, Vec<Weak<Node>>> = node1.neighbors.borrow_mut();
        r1.push(Rc::downgrade(&node0));
    }

    // No infinite loop
    println!("{:#?}", node0);

    // Print node1 - prints Some(Node)
    println!(
        "{:#?}",
        node0
            .neighbors
            .borrow()
            .get(0)
            .map(|weak_ref| weak_ref.upgrade())
    );

    // Drop node1
    std::mem::drop(node1);

    // Print node1 - prints None
    println!(
        "{:#?}",
        node0
            .neighbors
            .borrow()
            .get(0)
            .map(|weak_ref| weak_ref.upgrade())
    );
}
