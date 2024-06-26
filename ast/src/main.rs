use std::fs;
use serde_json::Value;

fn main() {
    let file_path = "tmp/ERC20.json";
    let content = fs::read_to_string(file_path).unwrap();
    let v = serde_json::from_str::<Value>(&content).unwrap();
    println!("{:?}", v);
}
