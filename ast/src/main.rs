use std::fs;

pub mod types;

use types::Ast;

fn main() {
    let file_path = "tmp/ERC20.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();
    // println!("{:#?}", ast);
    for node in ast.ast.nodes.iter() {
        println!("{:?}", node.r#type());
    }
}
