#![allow(unused)]

fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    while let Some(i) = iter.next() {
        println!("i = {}", i);
    }
}
