#![allow(unused)]

use std::collections::HashMap;
use std::fs;

pub mod ast;
pub mod graph;

use ast::Ast;
use graph::{Contract, Function, Import, Variable};

fn main() {
    let file_path = "tmp/ERC20.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();

    let mut graph_nodes: HashMap<i64, graph::Node> = HashMap::new();

    // println!("{:#?}", ast);
    // return;

    // TODO: Read from state variable
    // TODO: contract inheritance
    // TODO: function call and assign returned value
    // TODO: map state variable to interface

    for node in ast.ast.nodes.iter() {
        match node {
            ast::SourceUnitNode::ImportDirective(import_directive) => {
                for sym in import_directive.symbol_aliases.iter() {
                    let id = sym.foreign.referenced_declaration.unwrap();
                    graph_nodes.insert(
                        id,
                        graph::Node::Import(Import {
                            id,
                            name: sym.foreign.name.to_string(),
                            abs_path: import_directive.absolute_path.to_string(),
                        }),
                    );
                }
            }
            ast::SourceUnitNode::ContractDefinition(contract_def) => {
                if !graph_nodes.contains_key(&contract_def.id) {
                    graph_nodes.insert(
                        contract_def.id,
                        graph::Node::Contract(Contract::new(contract_def.id, &contract_def.name)),
                    );
                }

                let graph::Node::Contract(ref mut contract) =
                    graph_nodes.get_mut(&contract_def.id).unwrap()
                else {
                    todo!("TODO panic");
                };

                for node in contract_def.nodes.iter() {
                    match node {
                        ast::ContractNode::VariableDeclaration(var_dec) => {
                            if var_dec.state_variable {
                                contract.state_variable_ids.push(var_dec.id);
                                contract
                                    .state_variables
                                    .insert(var_dec.id, Variable::new(var_dec.id, &var_dec.name));
                            }
                        }
                        ast::ContractNode::FunctionDefinition(func_def) => {
                            let mut func = Function::new(func_def.id, &func_def.name);

                            if let Some(body) = &func_def.body {
                                if let Some(statements) = &body.statements {
                                    for s in statements.iter() {
                                        if let ast::Statement::ExpressionStatement(exp_statement) =
                                            s
                                        {
                                            match *exp_statement.expression.clone() {
                                                ast::Expression::Assignment(assignment) => {
                                                    match *assignment.left_hand_side {
                                                        ast::Expression::Identifier(id) => {
                                                            let ref_dec =
                                                                id.referenced_declaration.unwrap();
                                                            if let Some(state_var) = contract
                                                                .state_variables
                                                                .get(&ref_dec)
                                                            {
                                                                assert!(id.name == state_var.name);
                                                                func.body.push(
                                                                    state_var.name.to_string(),
                                                                );
                                                            }
                                                        }
                                                        ast::Expression::IndexAccess(idx_acc) => {
                                                            match *idx_acc.base_expression {
                                                                // access simple mapping
                                                                ast::Expression::Identifier(id) => {
                                                                    func.body
                                                                        .push(id.name.to_string());
                                                                }
                                                                // TODO: algo to traverse and search for state variables
                                                                // access nested mapping
                                                                ast::Expression::IndexAccess(
                                                                    idx_acc,
                                                                ) => {
                                                                    match *idx_acc.base_expression {
                                                                        ast::Expression::Identifier(
                                                                            id,
                                                                        ) => {
                                                                            func.body
                                                                                .push(id.name.to_string());
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                        _ => (),
                                                    }
                                                }
                                                ast::Expression::FunctionCall(func_call) => {
                                                    match *func_call.expression {
                                                        // external call
                                                        ast::Expression::MemberAccess(mem_acc) => {
                                                            let mut func_name = mem_acc.member_name;
                                                            if let ast::Expression::Identifier(id) =
                                                                *mem_acc.expression
                                                            {
                                                                func_name = format!(
                                                                    "{}.{}()",
                                                                    id.name, func_name
                                                                );
                                                            };
                                                            func.body.push(func_name)
                                                        }
                                                        // internal call
                                                        ast::Expression::Identifier(id) => {
                                                            func.body
                                                                .push(format!("{}()", id.name));
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

                            contract.function_ids.push(func_def.id);
                            contract.functions.insert(func.id, func);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    println!("{:#?}", graph_nodes);
}
