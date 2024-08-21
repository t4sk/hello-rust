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
}
