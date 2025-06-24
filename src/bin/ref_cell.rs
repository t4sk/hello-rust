#![allow(unused)]

use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

// Rc<T>
// - Strong reference
// - Increments strong_count
// - Always a reference to live data
// - Data is dropped when strong_count = 0
// - Keeps data alive, can cause memory leak

// Weak<T>
// - Weak reference
// - Increments weak_count
// - Reference to data that may be live or deallocated
// - Data can be dropped even if weak_count > 0
// - Doesn't keep data alive

#[derive(Debug)]
struct Node {
    val: u32,
    neighbors: RefCell<Vec<Weak<Node>>>,
}

fn main() {
    // Rc<T>
    // Rc::new and Rc::clone +1 strong_count
    // Rc drop -1 strong_count
    // x is dropped when strong_count = 0
    let x = "hello".to_string();

    let r0: Rc<String> = Rc::new(x);
    println!("strong count: {}", Rc::strong_count(&r0));

    {
        let r1 = Rc::clone(&r0);
        println!("strong count: {}", Rc::strong_count(&r0));
    }

    println!("strong count: {}", Rc::strong_count(&r0));

    // Weak<T>
    // Rc::downgrade
    // - Creates weak reference
    // - weak_count increases by 1
    // - Cannot access data behind reference
    // Rc::upgrade
    // - Upgrades weak to strong reference
    // - strong_count increases
    // - Can access data behind reference
    // - Returns Option<Rc<T>>
    let w1: Weak<String> = Rc::downgrade(&r0);
    println!("--- w1 ---");
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));
    // Cannot see data behind reference
    println!("w1: {:#?}", w1);

    let w2: Weak<String> = Rc::downgrade(&r0);
    println!("--- w2 ---");
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));

    // Upgrade weak to strong reference
    let u = w1.upgrade();
    println!("--- upgrade ---");
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));
    // Can see data behind reference
    println!("w1 upgrade: {:#?}", u);

    // Drop all strong references
    println!("--- drop ---");
    std::mem::drop(u);
    std::mem::drop(r0);

    let u = w1.upgrade();
    // upgrade returns None
    println!("w1 upgrade: {:#?}", u);

    // See mem_leak for example of memory leak
    // mem_leak();

    // Example - Node
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });

    // Reference cycle
    // node 0 -> node 1
    // node 1 -> node 0
    {
        // node 0 -> node 1
        let mut r0: RefMut<'_, Vec<Weak<Node>>> = node0.neighbors.borrow_mut();
        r0.push(Rc::downgrade(&node1));

        // node 1 -> node 0
        let mut r1: RefMut<'_, Vec<Weak<Node>>> = node1.neighbors.borrow_mut();
        r1.push(Rc::downgrade(&node0));
    }

    // No infinite loop
    println!("{:#?}", node0);

    // No memory leak
    println!(
        "node 0 strong: {:?}, weak: {:?}",
        Rc::strong_count(&node0),
        Rc::weak_count(&node0)
    );
    println!(
        "node 1 strong: {:?}, weak: {:?}",
        Rc::strong_count(&node1),
        Rc::weak_count(&node1)
    );

    // Drop node1
    std::mem::drop(node1);

    // Dropping node1 decreases Rc::weak_count(&node0) by 1
    println!("--- node 1 dropped ---");
    println!(
        "node 0 strong: {:?}, weak: {:?}",
        Rc::strong_count(&node0),
        Rc::weak_count(&node0)
    );
}

fn mem_leak() {
    #[derive(Debug)]
    struct Node {
        val: u32,
        neighbors: RefCell<Vec<Rc<Node>>>,
    }

    // Reference cycle
    // node 0 -> node 1
    // node 1 -> node 0
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });

    {
        let mut r0 = node0.neighbors.borrow_mut();
        r0.push(Rc::clone(&node1));

        let mut r1 = node1.neighbors.borrow_mut();
        r1.push(Rc::clone(&node0));
    }

    // Infinite loop
    // println!("{:#?}", node1);

    // Memory leak
    println!("node 0: {:?}", Rc::strong_count(&node0));
    println!("node 1: {:?}", Rc::strong_count(&node1));

    std::mem::drop(node1);

    // node1 is dropped but node0.neighbors stores reference to node1
    // node1.neighbors stores reference to node0
    // Hence Rc::strong_count(&node0) remains the same (2)
    println!("node 0: {:?}", Rc::strong_count(&node0));
}
