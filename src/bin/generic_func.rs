#![allow(unused)]

use std::cmp::PartialOrd;

fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];

    for item in s {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

// Multiple generic types
fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

fn main() {
    let nums = vec![33, 1, 22, 54, 25, 99, 10];
    let largest = max(&nums);
    println!("largest num: {:?}", largest);

    let chars = vec!['a', 'c', 'y', 'i', 'm'];
    let largest = max(&chars);
    println!("largest char: {:?}", largest);

    let t: (i32, bool) = (1, true);
    let s = swap(t);
    println!("swapped: {:?}", s);
}
