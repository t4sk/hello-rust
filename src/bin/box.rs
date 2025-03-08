#![allow(unused)]

// Smart pointer
// Pointer with metadata and additional capabilities

// Box
// - Allows data to be stored on the heap
// - Useful for data where the size is not known at compile time
//   - Trait objects
//   - Recursive data structure

// Box<dyn Error> = trait object that implements the Error trait
fn f() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// Recursive data structures
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct Tree {
    val: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

use crate::List::{Cons, Nil};

fn main() {
    // Box - allocate data to heap
    let b: Box<i32> = Box::new(0);
    // Dereference to get the inner value
    let v = *b;
    println!("box: {}", v);

    // List
    // 3 -> 2 -> 1 -> Nil
    let mut list = Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))));
    // Example - print all values in list
    while let Cons(i, tail) = list {
        print!("{} -> ", i);
        // Dereference a box
        list = *tail;
    }
    println!("Nil");

    // Tree
    let tree = Tree {
        val: 1,
        left: Some(Box::new(Tree {
            val: 2,
            left: None,
            right: Some(Box::new(Tree {
                val: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Tree {
            val: 3,
            left: None,
            right: None,
        })),
    };
    // No need to dereference Box<Tree>
    // Rust automatically dereferences struct fields
    println!(
        "tree.left.right.val: {:?}",
        tree.left.unwrap().right.unwrap().val
    );
}
