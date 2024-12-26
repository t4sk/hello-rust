#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");
    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");

    set.insert(2);
    set.insert(3);

    println!("1: {}", set.contains(&1));
    println!("2: {}", set.contains(&2));
    println!("3: {}", set.contains(&3));
    println!("4: {}", set.contains(&4));
}
