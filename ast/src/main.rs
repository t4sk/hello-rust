#![allow(unused)]

use std::collections::HashMap;
use std::fs;

pub mod types;

use types::Ast;

#[derive(Debug)]
struct Function {
    // TODO: handle constructor, fallback, recieve
    pub name: String,
}

#[derive(Debug)]
struct Contract {
    // TODO: handle inheritance
    pub name: String,
    pub state_variables: Vec<String>,
    pub functions: Vec<Function>,
}

impl Contract {
    pub fn new(name: String) -> Self {
        Self {
            name,
            // TODO: store and sort by slots?
            // TODO: store types
            state_variables: vec![],
            functions: vec![],
        }
    }
}

fn main() {
    let file_path = "tmp/Pot.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();

    let mut contracts: HashMap<String, Contract> = HashMap::new();

    // println!("{:#?}", ast);
    // return;

    // Internal function
    // FunctionDefinition
    // -> name
    // -> statements
    //    -> FunctionCall
    //       -> expression
    //          -> Identifier
    //             -> name
    //

    // Write to state variable
    // FunctionDefinition
    // -> statements
    //    -> ExpressionStatement
    //       -> expression
    //          Assignment
    //          -> left_hand_side
    //             -> Identifier
    //                -> name
    //                -> referenced_declaration

    // TODO: Read from state variable

    for node in ast.ast.nodes.iter() {
        if let types::Node::ContractDefinition(contract_def) = node {
            if !contracts.contains_key(&contract_def.name) {
                contracts.insert(
                    contract_def.name.to_string(),
                    Contract::new(contract_def.name.to_string()),
                );
            }

            let mut contract = contracts.get_mut(&contract_def.name).unwrap();

            for node in contract_def.nodes.iter() {
                match node {
                    types::Node::VariableDeclaration(var_dec) => {
                        if var_dec.state_variable {
                            contract.state_variables.push(var_dec.name.to_string());
                        }
                    }
                    types::Node::FunctionDefinition(func_def) => {
                        contract.functions.push(Function {
                            name: func_def.name.to_string(),
                        });

                        if let Some(body) = &func_def.body {
                            if let Some(statements) = &body.statements {
                                for s in statements.iter() {
                                    match s {
                                        types::Statement::ExpressionStatement(exp_statement) => {
                                            match *exp_statement.expression.clone() {
                                                types::Expression::Assignment(assignment) => {
                                                    if let types::Expression::Identifier(id) =
                                                        *assignment.left_hand_side
                                                    {
                                                        println!(
                                                            "write {} {:?}",
                                                            id.name, id.referenced_declaration
                                                        );
                                                    }
                                                }
                                                types::Expression::FunctionCall(func_call) => {
                                                    match *func_call.expression {
                                                        types::Expression::MemberAccess(
                                                            mem_acc,
                                                        ) => {
                                                            let func = mem_acc.member_name;
                                                            match *mem_acc.expression {
                                                                types::Expression::Identifier(
                                                                    id,
                                                                ) => {
                                                                    println!(
                                                                        "-- {}.{}",
                                                                        id.name, func
                                                                    );
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                        types::Expression::Identifier(id) => {
                                                            println!("-- {}", id.name);
                                                        }
                                                        _ => (),
                                                    }
                                                }
                                                _ => (),
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    println!("{:#?}", contracts);
}
