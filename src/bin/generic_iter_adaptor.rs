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

    // map and collect
    let vals: Vec<(u32, u32)> = vec![(1, 2), (3, 4)];
    let v: Vec<(u32, u32)> = vals.iter().map(|x| (x.0 * 2, x.1 * 2)).collect();
    let v: HashMap<u32, u32> = vals.iter().map(|x| (x.0 * 2, x.1 * 2)).collect();

    println!("collect: {:?}", v);

    // filter
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    // iter borrows
    let v: Vec<&u32> = vals.iter().filter(|&x| *x <= 3).collect();
    // into_iter takes ownership
    let v: Vec<u32> = vals.into_iter().filter(|&x| x <= 3).collect();
    // Ownership moved to v so this won't compile
    // println!("{:?}", vals);
    println!("filter: {:?}", v);

    // Chaining map and filter
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals
        .iter()
        .map(|x| x * 2)
        .into_iter()
        .filter(|&x| x <= 4)
        .collect();

    println!("map and then filter: {:?}", v);

    // enumerate
    let vals: Vec<u32> = vec![10, 20, 30, 40, 50];
    for (i, val) in vals.iter().enumerate() {
        println!("{i}: {val}");
    }

    // rev
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals.into_iter().rev().collect();
    println!("reverse: {:?}", v);

    // zip
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let letters = vec!["a", "b", "c"];

    let zipped: Vec<(u32, &str)> = vals.into_iter().zip(letters.into_iter()).collect();
    println!("zip: {:?}", zipped);

    // fold - can implement map, filter, zip, etc.. from fold
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let s = vals.iter().fold(0, |z, x| z + x);
    println!("fold: {s}");

    // fold on range
    let s = (1..=10).fold(0, |z, x| z + x);
    println!("range: {s}");
}
