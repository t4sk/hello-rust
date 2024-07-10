use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("red"), 100);
    scores.insert(String::from("blue"), 200);

    let score = scores.get(&String::from("red"));
    println!("{:?}", score);

    let score = scores.get(&String::from("green"));
    println!("{:?}", score);

    let score = scores.entry(String::from("blue")).or_insert(0);
    *score += 1;

    let score = scores.get(&String::from("red"));
    println!("{:?}", score);
}
