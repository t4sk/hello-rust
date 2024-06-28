use serde_json::Value;
use std::fs;

pub mod types;

use types::{ast::Ast, node::Node, node::NodeType};

fn bfs(node: &Node, node_type: NodeType) {
    let mut q = vec![node];

    while let Some(node) = q.pop() {
        for node in node.nodes.iter() {
            if node.node_type == node_type {
                println!("HERE {:#?}", node);
            }
            q.push(node);
        }
    }
}

fn main() {
    let file_path = "tmp/ERC20.json";
    let content = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&content).unwrap();
    println!("{:#?}", ast);

    bfs(&ast.ast, NodeType::ContractDefinition);
}
