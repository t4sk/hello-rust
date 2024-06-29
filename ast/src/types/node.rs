use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Into;

#[derive(Debug, Deserialize, PartialEq)]
pub enum NodeType {
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    UsingForDirective,
    ContractDefinition,
    InheritanceSpecifier,
    OverrideSpecifier,
    IdentifierPath,
    StructuredDocumentation,
    VariableDeclaration,
    Mapping,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    ArrayTypeName,
    TupleExpression,
    FunctionDefinition,
    ParameterList,
    Block,
    UncheckedBlock,
    Continue,
    Break,
    Return,
    Throw,
    Literal,
    Conditional,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    MemberAccess,
    Assignment,
    FunctionCall,
    FunctionCallOptions,
    FunctionTypeName,
    NewExpression,
    ExpressionStatement,
    VariableDeclarationStatement,
    IfStatement,
    TryCatchClause,
    UnaryOperation,
    BinaryOperation,
    EventDefinition,
    ErrorDefinition,
    EmitStatement,
    PlaceholderStatement,
    TryStatement,
    RevertStatement,
    ForStatement,
    WhileStatement,
    DoWhileStatement,
    ModifierDefinition,
    ModifierInvocation,
    EnumDefinition,
    EnumValue,
    StructDefinition,
    UserDefinedTypeName,
    InlineAssembly,
    YulLiteral,
    YulTypedName,
    YulSwitch,
    YulCase,
    YulFunctionCall,
    YulExpressionStatement,
    YulAssignment,
    YulIdentifier,
    YulVariableDeclaration,
    YulBlock,
    UserDefinedValueTypeDefinition,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
    pub nodes: Vec<Node>,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct ContractDefintion {
    pub name: String,
}

impl Into<ContractDefintion> for &HashMap<String, Value> {
    fn into(self) -> ContractDefintion {
        ContractDefintion {
            name: self.get("name").unwrap().as_str().unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum FunctionKind {
    Constructor,
    Function,
    Receive,
    Fallback,
    FreeFunction,
}


#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    // pub kind: FunctionKind
}

impl Into<FunctionDefinition> for &HashMap<String, Value> {
    fn into(self) -> FunctionDefinition {
        FunctionDefinition {
            name: self.get("name").unwrap().as_str().unwrap().to_string(),
        }
    }
}
