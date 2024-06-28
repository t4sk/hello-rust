use serde::Deserialize;

use super::node::Node;

#[derive(Debug, Deserialize)]
pub struct Ast {
    pub ast: Node,
}
