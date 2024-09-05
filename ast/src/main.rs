#![allow(unused)]

use std::collections::HashMap;
use std::fs;

pub mod graph;
pub mod types;

use graph::{Contract, Function, Import, Variable};
use types::Ast;

fn main() {
    let file_path = "tmp/tmp.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();

    let mut graph_nodes: HashMap<i64, graph::Node> = HashMap::new();

    // println!("{:#?}", ast);
    // return;

    // TODO: Read from state variable
    // TODO: contract inheritance
    // TODO: function call and assign returned value
    // TODO: map state variable to interface

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

    for node in ast.ast.nodes.iter() {
        match node {
            types::SourceUnitNode::ImportDirective(import_directive) => {
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
            types::SourceUnitNode::ContractDefinition(contract_def) => {
                if !graph_nodes.contains_key(&contract_def.id) {
                    graph_nodes.insert(
                        contract_def.id,
                        graph::Node::Contract(Contract::new(contract_def.id, &contract_def.name)),
                    );
                }

                // println!("{:#?}", contract_def.base_contracts);

                let graph::Node::Contract(ref mut contract) =
                    graph_nodes.get_mut(&contract_def.id).unwrap()
                else {
                    todo!("TODO panic");
                };

                for node in contract_def.nodes.iter() {
                    match node {
                        types::ContractNode::VariableDeclaration(var_dec) => {
                            if var_dec.state_variable {
                                contract.state_variable_ids.push(var_dec.id);
                                contract
                                    .state_variables
                                    .insert(var_dec.id, Variable::new(var_dec.id, &var_dec.name));
                            }
                        }
                        types::ContractNode::FunctionDefinition(func_def) => {
                            contract.function_ids.push(func_def.id);
                            let mut func = Function::new(func_def.id, &func_def.name);

                            if let Some(body) = &func_def.body {
                                if let Some(statements) = &body.statements {
                                    for s in statements.iter() {
                                        if let types::Statement::ExpressionStatement(
                                            exp_statement,
                                        ) = s
                                        {
                                            match *exp_statement.expression.clone() {
                                                /*
                                                                                                types::Expression::Assignment(assignment) => {
                                                                                                    if let types::Expression::Identifier(id) =
                                                                                                        *assignment.left_hand_side
                                                                                                    {
                                                                                                        let ref_dec =
                                                                                                            id.referenced_declaration.unwrap();
                                                                                                        if let Some(state_var) =
                                                                                                            contract.state_variables.get(&ref_dec)
                                                                                                        {
                                                                                                            assert!(&id.name == state_var);
                                                                                                            func.body
                                                                                                                .push(format!("{}", state_var));
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                */
                                                types::Expression::FunctionCall(func_call) => {
                                                    match *func_call.expression {
                                                        types::Expression::MemberAccess(
                                                            mem_acc,
                                                        ) => {
                                                            let mut func_name = mem_acc.member_name;
                                                            if let types::Expression::Identifier(
                                                                id,
                                                            ) = *mem_acc.expression
                                                            {
                                                                func_name = format!(
                                                                    "{}.{}()",
                                                                    id.name, func_name
                                                                );
                                                            };
                                                            func.body.push(func_name)
                                                        }
                                                        types::Expression::Identifier(id) => {
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
