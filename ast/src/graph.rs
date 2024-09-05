use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Variable(Variable),
    Function(Function),
    Contract(Contract),
    Import(Import),
}

#[derive(Debug)]
pub enum Action {
    Read,
    Write,
}

#[derive(Debug)]
pub struct Arrow(Node, Node, Action);

#[derive(Debug)]
pub struct Variable {
    pub id: i64,
    pub name: String,
}

impl Variable {
    pub fn new<S: Into<String>>(id: i64, name: S) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    // TODO: handle constructor, fallback, recieve
    pub id: i64,
    pub name: String,
    // TODO: Vec<Node>
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
pub struct Contract {
    // TODO: handle inheritance
    pub id: i64,
    pub name: String,
    pub state_variable_ids: Vec<i64>,
    pub state_variables: HashMap<i64, Variable>,
    pub function_ids: Vec<i64>,
    pub functions: HashMap<i64, Function>,
}

impl Contract {
    pub fn new<S: Into<String>>(id: i64, name: S) -> Self {
        Self {
            id,
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

#[derive(Debug)]
pub struct Import {
    pub abs_path: String,
    pub name: String,
    pub id: i64,
}
