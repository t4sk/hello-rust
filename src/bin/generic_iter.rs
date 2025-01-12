#![allow(unused)]

use std::collections::{HashMap, HashSet};

/*
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
*/

struct Counter {
    pub count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // iter - borrows and returns a iterator that returns &T
    // into_iter - takes ownership and returns a iterator that may return T, &T or &mut T
    // iter_mut - returns &mut T

    let mut counter = Counter { count: 0 };
    while let Some(c) = counter.next() {
        println!("count {}", c);
    }

    // Default implementation of into_iter is implicitly called
    let counter = Counter { count: 0 };
    for c in counter {
        println!("count {}", c);
    }

    // into_iter
    // - For loop implicitly calls into_iter
    // - Takes ownership
    // - Cannot call twice
    let vals: Vec<i32> = vec![1, 2, 3];
    for v in vals {
        // v has type i32
        println!("into_iter: {}", v);
    }

    /*
    for v in vals {
        println!("{}", v);
    }
    */

    // into_iter on HashMap
    let mut map: HashMap<u32, char> = HashMap::new();
    map.insert(1, 'a');
    map.insert(2, 'b');
    for (k, v) in map {
        println!("{}, {}", k, v);
    }

    // Convert vector into HashSet
    let vals: Vec<u32> = vec![1, 2, 3];
    let set: HashSet<u32> = vals.into_iter().collect();

    // iter
    // Use iter to loop multiple times
    let vals: Vec<i32> = vec![1, 2, 3];
    for v in vals.iter() {
        // v has type &i32
        println!("iter: {}", v);
    }
    for v in vals.iter() {
        println!("iter: {}", v);
    }

    // iter_mut
    let mut vals = vec![1, 2, 3];
    for v in vals.iter_mut() {
        *v += 10;
    }
    println!("iter_mut: {:?}", vals);

    // For loop with reference
    let vals: Vec<i32> = vec![1, 2, 3];
    for v in &vals {
        // v has type &i32
        println!("ref: {}", v);
    }
    for v in &vals {
        println!("ref: {}", v);
    }

    // For loop with array
    // Can loop twice because arr is implicitly copied
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for a in arr {
        // a has type u32
        println!("array: {}", a);
    }
    for a in arr {
        println!("array: {}", a);
    }
}
