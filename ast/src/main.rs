use std::fs;

pub mod types;

use types::Ast;

fn main() {
    let file_path = "tmp/Ast.json";
    let json = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&json).unwrap();

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
            println!("{}", contract_def.name);

            for node in contract_def.nodes.iter() {
                match node {
                    types::Node::VariableDeclaration(var_dec) => {
                        if var_dec.state_variable {
                            println!("{}", var_dec.name);
                        }
                    }
                    types::Node::FunctionDefinition(func_def) => {
                        println!("- {}", func_def.name);
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
}
