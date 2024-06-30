use serde::Deserialize;

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
    Unknown,
}

// Common
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    NonPayable,
    Payable,
    View,
    Pure,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

// FunctionDeclaration
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Function,
    Receive,
    Fallback,
    FreeFunction,
}

// Block
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: u32,
    pub src: String,
    pub statements: Vec<Statement>,
}

// ExpressionStatement
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: u32,
    pub src: String,
    pub expression: Expression,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: u32,
    pub src: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: u32,
    pub src: String,
    pub name: String,
}

// Expression
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall(FunctionCall),
    FunctionCallOptions,
    Identifier(Identifier),
    IndexAccess,
    IndexRangeAccess,
    Literal,
    MemberAccess,
    NewExpression,
    TupleExpression,
    UnaryOperation,
}

// Statement
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break,
    Continue,
    DoWhileStatement,
    EmitStatement,
    ExpressionStatement(ExpressionStatement),
    ForStatement,
    IfStatement,
    InlineAssembly,
    PlaceholderStatement,
    Return,
    RevertStatement,
    TryStatement,
    UncheckedBlock,
    VariableDeclarationStatement,
    WhileStatement,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Node {
    #[serde(rename_all = "camelCase")]
    SourceUnit { id: u32, nodes: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    PragmaDirective {},
    #[serde(rename_all = "camelCase")]
    ImportDirective {},
    #[serde(rename_all = "camelCase")]
    UsingForDirective {},
    #[serde(rename_all = "camelCase")]
    ContractDefinition {
        id: u32,
        nodes: Vec<Node>,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    InheritanceSpecifier {},
    #[serde(rename_all = "camelCase")]
    OverrideSpecifier {},
    #[serde(rename_all = "camelCase")]
    IdentifierPath {},
    #[serde(rename_all = "camelCase")]
    StructuredDocumentation {},
    #[serde(rename_all = "camelCase")]
    VariableDeclaration {
        id: u32,
        nodes: Vec<Node>,
        name: String,
        visibility: Visibility,
    },
    #[serde(rename_all = "camelCase")]
    Mapping {},
    #[serde(rename_all = "camelCase")]
    ElementaryTypeName {},
    #[serde(rename_all = "camelCase")]
    ElementaryTypeNameExpression {},
    #[serde(rename_all = "camelCase")]
    ArrayTypeName {},
    #[serde(rename_all = "camelCase")]
    TupleExpression {},
    #[serde(rename_all = "camelCase")]
    FunctionDefinition {
        id: u32,
        nodes: Vec<Node>,
        kind: FunctionKind,
        name: String,
        visibility: Visibility,
        state_mutability: StateMutability,
        body: Block,
    },
    #[serde(rename_all = "camelCase")]
    ParameterList {},
    #[serde(rename_all = "camelCase")]
    Block(Block),
    #[serde(rename_all = "camelCase")]
    UncheckedBlock {},
    #[serde(rename_all = "camelCase")]
    Continue {},
    #[serde(rename_all = "camelCase")]
    Break {},
    #[serde(rename_all = "camelCase")]
    Return {},
    #[serde(rename_all = "camelCase")]
    Throw {},
    #[serde(rename_all = "camelCase")]
    Literal {},
    #[serde(rename_all = "camelCase")]
    Conditional {},
    #[serde(rename_all = "camelCase")]
    Identifier {},
    #[serde(rename_all = "camelCase")]
    IndexAccess {},
    #[serde(rename_all = "camelCase")]
    IndexRangeAccess {},
    #[serde(rename_all = "camelCase")]
    MemberAccess {},
    #[serde(rename_all = "camelCase")]
    Assignment {},
    #[serde(rename_all = "camelCase")]
    FunctionCall {},
    #[serde(rename_all = "camelCase")]
    FunctionCallOptions {},
    #[serde(rename_all = "camelCase")]
    FunctionTypeName {},
    #[serde(rename_all = "camelCase")]
    NewExpression {},
    #[serde(rename_all = "camelCase")]
    ExpressionStatement {},
    #[serde(rename_all = "camelCase")]
    VariableDeclarationStatement {},
    #[serde(rename_all = "camelCase")]
    IfStatement {},
    #[serde(rename_all = "camelCase")]
    TryCatchClause {},
    #[serde(rename_all = "camelCase")]
    UnaryOperation {},
    #[serde(rename_all = "camelCase")]
    BinaryOperation {},
    #[serde(rename_all = "camelCase")]
    EventDefinition {},
    #[serde(rename_all = "camelCase")]
    ErrorDefinition {},
    #[serde(rename_all = "camelCase")]
    EmitStatement {},
    #[serde(rename_all = "camelCase")]
    PlaceholderStatement {},
    #[serde(rename_all = "camelCase")]
    TryStatement {},
    #[serde(rename_all = "camelCase")]
    RevertStatement {},
    #[serde(rename_all = "camelCase")]
    ForStatement {},
    #[serde(rename_all = "camelCase")]
    WhileStatement {},
    #[serde(rename_all = "camelCase")]
    DoWhileStatement {},
    #[serde(rename_all = "camelCase")]
    ModifierDefinition {},
    #[serde(rename_all = "camelCase")]
    ModifierInvocation {},
    #[serde(rename_all = "camelCase")]
    EnumDefinition {},
    #[serde(rename_all = "camelCase")]
    EnumValue {},
    #[serde(rename_all = "camelCase")]
    StructDefinition {},
    #[serde(rename_all = "camelCase")]
    UserDefinedTypeName {},
    #[serde(rename_all = "camelCase")]
    InlineAssembly {},
    #[serde(rename_all = "camelCase")]
    YulLiteral {},
    #[serde(rename_all = "camelCase")]
    YulTypedName {},
    #[serde(rename_all = "camelCase")]
    YulSwitch {},
    #[serde(rename_all = "camelCase")]
    YulCase {},
    #[serde(rename_all = "camelCase")]
    YulFunctionCall {},
    #[serde(rename_all = "camelCase")]
    YulExpressionStatement {},
    #[serde(rename_all = "camelCase")]
    YulAssignment {},
    #[serde(rename_all = "camelCase")]
    YulIdentifier {},
    #[serde(rename_all = "camelCase")]
    YulVariableDeclaration {},
    #[serde(rename_all = "camelCase")]
    YulBlock {},
    #[serde(rename_all = "camelCase")]
    UserDefinedValueTypeDefinition {},
    #[serde(rename_all = "camelCase")]
    #[serde(untagged)]
    Unknown { id: u32, nodes: Vec<Node> },
}

impl Node {
    pub fn get_type(&self) -> NodeType {
        match self {
            Node::SourceUnit { .. } => NodeType::SourceUnit,
            Node::PragmaDirective { .. } => NodeType::PragmaDirective,
            Node::ImportDirective { .. } => NodeType::ImportDirective,
            Node::UsingForDirective { .. } => NodeType::UsingForDirective,
            Node::ContractDefinition { .. } => NodeType::ContractDefinition,
            Node::InheritanceSpecifier { .. } => NodeType::InheritanceSpecifier,
            Node::OverrideSpecifier { .. } => NodeType::OverrideSpecifier,
            Node::IdentifierPath { .. } => NodeType::IdentifierPath,
            Node::StructuredDocumentation { .. } => NodeType::StructuredDocumentation,
            Node::VariableDeclaration { .. } => NodeType::VariableDeclaration,
            Node::Mapping { .. } => NodeType::Mapping,
            Node::ElementaryTypeName { .. } => NodeType::ElementaryTypeName,
            Node::ElementaryTypeNameExpression { .. } => NodeType::ElementaryTypeNameExpression,
            Node::ArrayTypeName { .. } => NodeType::ArrayTypeName,
            Node::TupleExpression { .. } => NodeType::TupleExpression,
            Node::FunctionDefinition { .. } => NodeType::FunctionDefinition,
            Node::ParameterList { .. } => NodeType::ParameterList,
            Node::Block { .. } => NodeType::Block,
            Node::UncheckedBlock { .. } => NodeType::UncheckedBlock,
            Node::Continue { .. } => NodeType::Continue,
            Node::Break { .. } => NodeType::Break,
            Node::Return { .. } => NodeType::Return,
            Node::Throw { .. } => NodeType::Throw,
            Node::Literal { .. } => NodeType::Literal,
            Node::Conditional { .. } => NodeType::Conditional,
            Node::Identifier { .. } => NodeType::Identifier,
            Node::IndexAccess { .. } => NodeType::IndexAccess,
            Node::IndexRangeAccess { .. } => NodeType::IndexRangeAccess,
            Node::MemberAccess { .. } => NodeType::MemberAccess,
            Node::Assignment { .. } => NodeType::Assignment,
            Node::FunctionCall { .. } => NodeType::FunctionCall,
            Node::FunctionCallOptions { .. } => NodeType::FunctionCallOptions,
            Node::FunctionTypeName { .. } => NodeType::FunctionTypeName,
            Node::NewExpression { .. } => NodeType::NewExpression,
            Node::ExpressionStatement { .. } => NodeType::ExpressionStatement,
            Node::VariableDeclarationStatement { .. } => NodeType::VariableDeclarationStatement,
            Node::IfStatement { .. } => NodeType::IfStatement,
            Node::TryCatchClause { .. } => NodeType::TryCatchClause,
            Node::UnaryOperation { .. } => NodeType::UnaryOperation,
            Node::BinaryOperation { .. } => NodeType::BinaryOperation,
            Node::EventDefinition { .. } => NodeType::EventDefinition,
            Node::ErrorDefinition { .. } => NodeType::ErrorDefinition,
            Node::EmitStatement { .. } => NodeType::EmitStatement,
            Node::PlaceholderStatement { .. } => NodeType::PlaceholderStatement,
            Node::TryStatement { .. } => NodeType::TryStatement,
            Node::RevertStatement { .. } => NodeType::RevertStatement,
            Node::ForStatement { .. } => NodeType::ForStatement,
            Node::WhileStatement { .. } => NodeType::WhileStatement,
            Node::DoWhileStatement { .. } => NodeType::DoWhileStatement,
            Node::ModifierDefinition { .. } => NodeType::ModifierDefinition,
            Node::ModifierInvocation { .. } => NodeType::ModifierInvocation,
            Node::EnumDefinition { .. } => NodeType::EnumDefinition,
            Node::EnumValue { .. } => NodeType::EnumValue,
            Node::StructDefinition { .. } => NodeType::StructDefinition,
            Node::UserDefinedTypeName { .. } => NodeType::UserDefinedTypeName,
            Node::InlineAssembly { .. } => NodeType::InlineAssembly,
            Node::YulLiteral { .. } => NodeType::YulLiteral,
            Node::YulTypedName { .. } => NodeType::YulTypedName,
            Node::YulSwitch { .. } => NodeType::YulSwitch,
            Node::YulCase { .. } => NodeType::YulCase,
            Node::YulFunctionCall { .. } => NodeType::YulFunctionCall,
            Node::YulExpressionStatement { .. } => NodeType::YulExpressionStatement,
            Node::YulAssignment { .. } => NodeType::YulAssignment,
            Node::YulIdentifier { .. } => NodeType::YulIdentifier,
            Node::YulVariableDeclaration { .. } => NodeType::YulVariableDeclaration,
            Node::YulBlock { .. } => NodeType::YulBlock,
            Node::UserDefinedValueTypeDefinition { .. } => NodeType::UserDefinedValueTypeDefinition,
            Node::Unknown { .. } => NodeType::Unknown,
        }
    }

    pub fn get_nodes(&self) -> Option<&Vec<Node>> {
        match self {
            Node::SourceUnit { nodes, .. } => Some(&nodes),
            Node::ContractDefinition { nodes, .. } => Some(&nodes),
            Node::FunctionDefinition { nodes, .. } => Some(&nodes),
            Node::Unknown { nodes, .. } => Some(&nodes),
            _ => None,
        }
    }
}
