use std::fs;

pub mod types;

use types::{Ast, NodeType};

fn main() {
    let file_path = "tmp/ERC20.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();

    // println!("{:#?}", ast);
    for node in ast.ast.nodes.iter() {
        match node {
            types::Node::ContractDefinition(contract_def) => {
                println!("{:?}", contract_def.name);
                for node in contract_def.nodes.iter() {
                    match node {
                        types::Node::VariableDeclaration(var_dec) => {
                            if var_dec.state_variable {
                                println!("{:?}", var_dec.name);
                            }
                        }
                        types::Node::FunctionDefinition(func_def) => {
                            println!("- {:?}", func_def.name);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}
