#![allow(unused)]

fn main() {
    let vals = vec![1, 2, 3, 4];
    let mut iter = vals.iter();

    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}
