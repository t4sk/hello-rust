use std::fs;

fn main() {
    let file_path = "tmp/ERC20.json";
    let json = fs::read_to_string(file_path).unwrap();
    println!("{:#?}", json);
}
