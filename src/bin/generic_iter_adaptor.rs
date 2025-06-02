#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // map
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals
        .iter()
        .map(|x| {
            // use { } for multi line
            x * 2
        })
        .collect();

    println!("map: {:?}", v);

    // filter
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals.into_iter().filter(|x| *x <= 3).collect();
    println!("filter: {:?}", v);

    // Chaining map and filter
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals
        .into_iter()
        .filter(|x| *x <= 3)
        .map(|x| x * 2)
        .collect();

    println!("filter and then map: {:?}", v);

    // zip
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let letters: Vec<String> = vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect();

    let zipped: Vec<(String, u32)> = letters.into_iter().zip(vals.into_iter()).collect();
    println!("zip: {:?}", zipped);

    // zip into a hash map
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let letters: Vec<String> = vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect();

    let zipped: HashMap<String, u32> = letters.into_iter().zip(vals.into_iter()).collect();
    println!("zip hash map: {:?}", zipped);

    // fold - can implement map, filter, zip, etc.. from fold
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let s = vals.iter().fold(0, |z, x| z + x);
    println!("fold: {s}");

    // fold on range
    let s = (1..=10).fold(0, |z, x| z + x);
    println!("range: {s}");
}
