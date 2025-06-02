#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("red"), 100);
    scores.insert(String::from("blue"), 200);

    // Get
    let score: Option<&u32> = scores.get("red");
    println!("red: {:?}", score);

    let score = scores.get("green");
    println!("green: {:?}", score);

    // Insert
    scores.insert("green".to_string(), 300);

    // Upsert
    let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *score += 200;

    let score = scores.get("blue");
    println!("blue: {:?}", score);

    println!("scores: {:#?}", scores);
}
