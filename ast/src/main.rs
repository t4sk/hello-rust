#![allow(unused)]

use std::collections::HashMap;
use std::fs;

pub mod types;

use types::Ast;

#[derive(Debug)]
struct Function {
    // TODO: handle constructor, fallback, recieve
    pub id: i64,
    pub name: String,
    pub body: Vec<String>,
}

impl Function {
    pub fn new<S: Into<String>>(id: i64, name: S) -> Self {
        Self {
            id,
            name: name.into(),
            body: vec![],
        }
    }
}

#[derive(Debug)]
struct Contract {
    // TODO: handle inheritance
    pub name: String,
    pub state_variable_ids: Vec<i64>,
    pub state_variables: HashMap<i64, String>,
    pub function_ids: Vec<i64>,
    pub functions: HashMap<i64, Function>,
}

impl Contract {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            // TODO: store and sort by slots?
            // TODO: store types
            state_variable_ids: vec![],
            state_variables: HashMap::new(),
            function_ids: vec![],
            functions: HashMap::new(),
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

    // TODO: Read from state variable
    // TODO: contract inheritance

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
        if let types::SourceUnitNode::ContractDefinition(contract_def) = node {
            if !contracts.contains_key(&contract_def.name) {
                contracts.insert(
                    contract_def.name.to_string(),
                    Contract::new(&contract_def.name),
                );
            }

            // println!("{:#?}", contract_def.base_contracts);

            let mut contract = contracts.get_mut(&contract_def.name).unwrap();

            for node in contract_def.nodes.iter() {
                match node {
                    types::ContractNode::VariableDeclaration(var_dec) => {
                        if var_dec.state_variable {
                            contract.state_variable_ids.push(var_dec.id);
                            contract
                                .state_variables
                                .insert(var_dec.id, var_dec.name.to_string());
                        }
                    }
                    types::ContractNode::FunctionDefinition(func_def) => {
                        contract.function_ids.push(func_def.id);
                        let mut func = Function::new(func_def.id, &func_def.name);

                        if let Some(body) = &func_def.body {
                            if let Some(statements) = &body.statements {
                                for s in statements.iter() {
                                    if let types::Statement::ExpressionStatement(exp_statement) = s
                                    {
                                        match *exp_statement.expression.clone() {
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
                                                        func.body.push(format!("{}", state_var));
                                                    }
                                                }
                                            }
                                            types::Expression::FunctionCall(func_call) => {
                                                match *func_call.expression {
                                                    types::Expression::MemberAccess(mem_acc) => {
                                                        let mut func_name = mem_acc.member_name;
                                                        if let types::Expression::Identifier(id) =
                                                            *mem_acc.expression
                                                        {
                                                            func_name = format!(
                                                                "{}.{}()",
                                                                id.name, func_name
                                                            );
                                                        };
                                                        func.body.push(func_name)
                                                    }
                                                    types::Expression::Identifier(id) => {
                                                        func.body.push(format!("{}()", id.name));
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
    }

    println!("{:#?}", contracts);
}
