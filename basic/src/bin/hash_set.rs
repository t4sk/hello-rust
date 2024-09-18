#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    let mut map: HashMap<u32, bool> = HashMap::new();
    //           HashMap<T, ()>
    let mut set: HashSet<u32> = HashSet::new();

    set.insert(1);
    println!("{}", set.contains(&1));
    println!("{}", set.contains(&2));
}
