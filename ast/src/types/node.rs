use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum NodeType {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue,
    ContractDefinition,
    DoWhileStatement,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    ErrorDefinition,
    EventDefinition,
    ExpressionStatement,
    ForStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    Mapping,
    MemberAccess,
    ModifierDefinition,
    ModifierInvocation,
    NewExpression,
    OverrideSpecifier,
    ParameterList,
    PlaceholderStatement,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    Throw,
    TryCatchClause,
    TryStatement,
    TupleExpression,
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    YulAssignment,
    YulBlock,
    YulCase,
    YulExpressionStatement,
    YulFunctionCall,
    YulIdentifier,
    YulLiteral,
    YulSwitch,
    YulTypedName,
    YulVariableDeclaration,
    Unknown,
}

// Common
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Default,
    Memory,
    Calldata,
    Storage,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Immutable,
    Mutable,
    Constant,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Payable,
    NonPayable,
    Pure,
    View,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Internal,
    Public,
    External,
}

#[derive(Debug, Deserialize)]
pub struct StructuredDocumentation {
    pub id: u32,
    pub src: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: u32,
    pub src: String,
    pub name: String,
    pub referenced_declaration: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum UserDefinedTypeNameOrIdentifierPath {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Deserialize)]
pub struct OverrideSpecifier {
    pub id: u32,
    pub src: String,
    pub overrides: Vec<UserDefinedTypeNameOrIdentifierPath>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub id: u32,
    pub src: String,
    pub base_functions: Option<Vec<u32>>,
    pub constant: bool,
    pub documentation: Option<StructuredDocumentation>,
    pub function_selector: Option<String>,
    pub indexed: Option<bool>,
    pub mutability: Mutability,
    pub name: String,
    pub name_location: Option<String>,
    pub overrides: Option<OverrideSpecifier>,
    pub scope: u32,
    pub state_variable: bool,
    pub storage_location: StorageLocation,
    pub type_description: TypeDescriptions,
    pub type_name: Option<TypeName>,
    pub value: Option<Expression>,
    pub visibility: Visibility,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: u32,
    pub src: String,
    pub parameters: Vec<VariableDeclaration>,
}

// TypeName
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub id: u32,
    pub src: String,
    pub base_type: Box<TypeName>,
    pub type_description: TypeDescriptions,
    pub length: Option<Expression>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: u32,
    pub src: String,
    pub type_description: TypeDescriptions,
    pub name: String,
    pub state_mutability: StateMutability,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub id: u32,
    pub src: String,
    pub type_description: TypeDescriptions,
    pub state_mutability: StateMutability,
    pub visibility: Visibility,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub id: u32,
    pub src: String,
    pub key_name: Option<String>,
    pub key_name_location: Option<String>,
    pub key_type: Box<TypeName>,
    pub type_description: TypeDescriptions,
    pub value_name: Option<String>,
    pub value_name_location: Option<String>,
    pub value_type: Box<TypeName>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub id: u32,
    pub src: String,
    pub name: Option<String>,
    pub path_node: Option<IdentifierPath>,
    pub reference_declaration: u32,
    pub type_description: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
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

// TODO:
#[derive(Debug, Deserialize)]
pub struct Assignment {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub left_hand_side: Box<Expression>,
    pub operator: String,
    pub right_hand_side: Box<Expression>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub common_type: TypeDescriptions,
    pub function: Option<u32>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub left_expression: Box<Expression>,
    pub operator: String,
    pub right_expression: Box<Expression>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub condition: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub true_expression: Box<Expression>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub type_name: Box<TypeName>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub arguments: Vec<Expression>,
    pub expression: Vec<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub kind: FunctionCallKind,
    pub l_value_requested: bool,
    pub name_locations: Option<Vec<String>>,
    pub names: Vec<String>,
    pub try_call: bool,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: u32,
    pub src: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Literal {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExpress {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpress {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {}

// Expression
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpress),
    TupleExpression(TupleExpress),
    UnaryOperation(UnaryOperation),
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

// FunctionDeclaration
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Receive,
    Fallback,
    FreeFunction,
    Function,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Node {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation {},
    Block(Block),
    Break {},
    Conditional {},
    Continue {},
    ContractDefinition {
        id: u32,
        nodes: Vec<Node>,
        name: String,
    },
    DoWhileStatement {},
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression {},
    EmitStatement {},
    EnumDefinition {},
    EnumValue {},
    ErrorDefinition {},
    EventDefinition {},
    ExpressionStatement {},
    ForStatement {},
    FunctionCall {},
    FunctionCallOptions {},
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
    FunctionTypeName(FunctionTypeName),
    Identifier {},
    IdentifierPath {},
    IfStatement {},
    ImportDirective {},
    IndexAccess {},
    IndexRangeAccess {},
    InheritanceSpecifier {},
    InlineAssembly {},
    Literal {},
    Mapping(Mapping),
    MemberAccess {},
    ModifierDefinition {},
    ModifierInvocation {},
    NewExpression {},
    OverrideSpecifier {},
    ParameterList {},
    PlaceholderStatement {},
    PragmaDirective {},
    Return {},
    RevertStatement {},
    SourceUnit {
        id: u32,
        nodes: Vec<Node>,
    },
    StructDefinition {},
    StructuredDocumentation {},
    Throw {},
    TryCatchClause {},
    TryStatement {},
    TupleExpression {},
    UnaryOperation {},
    UncheckedBlock {},
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition {},
    UsingForDirective {},
    VariableDeclaration {
        id: u32,
        nodes: Vec<Node>,
        name: String,
        visibility: Visibility,
    },
    VariableDeclarationStatement {},
    WhileStatement {},
    YulAssignment {},
    YulBlock {},
    YulCase {},
    YulExpressionStatement {},
    YulFunctionCall {},
    YulIdentifier {},
    YulLiteral {},
    YulSwitch {},
    YulTypedName {},
    YulVariableDeclaration {},
    #[serde(untagged)]
    Unknown {
        id: u32,
        nodes: Vec<Node>,
    },
}

impl Node {
    pub fn get_type(&self) -> NodeType {
        match self {
            Node::ArrayTypeName { .. } => NodeType::ArrayTypeName,
            Node::Assignment { .. } => NodeType::Assignment,
            Node::BinaryOperation { .. } => NodeType::BinaryOperation,
            Node::Block { .. } => NodeType::Block,
            Node::Break { .. } => NodeType::Break,
            Node::Conditional { .. } => NodeType::Conditional,
            Node::Continue { .. } => NodeType::Continue,
            Node::ContractDefinition { .. } => NodeType::ContractDefinition,
            Node::DoWhileStatement { .. } => NodeType::DoWhileStatement,
            Node::ElementaryTypeName { .. } => NodeType::ElementaryTypeName,
            Node::ElementaryTypeNameExpression { .. } => NodeType::ElementaryTypeNameExpression,
            Node::EmitStatement { .. } => NodeType::EmitStatement,
            Node::EnumDefinition { .. } => NodeType::EnumDefinition,
            Node::EnumValue { .. } => NodeType::EnumValue,
            Node::ErrorDefinition { .. } => NodeType::ErrorDefinition,
            Node::EventDefinition { .. } => NodeType::EventDefinition,
            Node::ExpressionStatement { .. } => NodeType::ExpressionStatement,
            Node::ForStatement { .. } => NodeType::ForStatement,
            Node::FunctionCall { .. } => NodeType::FunctionCall,
            Node::FunctionCallOptions { .. } => NodeType::FunctionCallOptions,
            Node::FunctionDefinition { .. } => NodeType::FunctionDefinition,
            Node::FunctionTypeName { .. } => NodeType::FunctionTypeName,
            Node::Identifier { .. } => NodeType::Identifier,
            Node::IdentifierPath { .. } => NodeType::IdentifierPath,
            Node::IfStatement { .. } => NodeType::IfStatement,
            Node::ImportDirective { .. } => NodeType::ImportDirective,
            Node::IndexAccess { .. } => NodeType::IndexAccess,
            Node::IndexRangeAccess { .. } => NodeType::IndexRangeAccess,
            Node::InheritanceSpecifier { .. } => NodeType::InheritanceSpecifier,
            Node::InlineAssembly { .. } => NodeType::InlineAssembly,
            Node::Literal { .. } => NodeType::Literal,
            Node::Mapping { .. } => NodeType::Mapping,
            Node::MemberAccess { .. } => NodeType::MemberAccess,
            Node::ModifierDefinition { .. } => NodeType::ModifierDefinition,
            Node::ModifierInvocation { .. } => NodeType::ModifierInvocation,
            Node::NewExpression { .. } => NodeType::NewExpression,
            Node::OverrideSpecifier { .. } => NodeType::OverrideSpecifier,
            Node::ParameterList { .. } => NodeType::ParameterList,
            Node::PlaceholderStatement { .. } => NodeType::PlaceholderStatement,
            Node::PragmaDirective { .. } => NodeType::PragmaDirective,
            Node::Return { .. } => NodeType::Return,
            Node::RevertStatement { .. } => NodeType::RevertStatement,
            Node::SourceUnit { .. } => NodeType::SourceUnit,
            Node::StructDefinition { .. } => NodeType::StructDefinition,
            Node::StructuredDocumentation { .. } => NodeType::StructuredDocumentation,
            Node::Throw { .. } => NodeType::Throw,
            Node::TryCatchClause { .. } => NodeType::TryCatchClause,
            Node::TryStatement { .. } => NodeType::TryStatement,
            Node::TupleExpression { .. } => NodeType::TupleExpression,
            Node::UnaryOperation { .. } => NodeType::UnaryOperation,
            Node::UncheckedBlock { .. } => NodeType::UncheckedBlock,
            Node::UserDefinedTypeName { .. } => NodeType::UserDefinedTypeName,
            Node::UserDefinedValueTypeDefinition { .. } => NodeType::UserDefinedValueTypeDefinition,
            Node::UsingForDirective { .. } => NodeType::UsingForDirective,
            Node::VariableDeclaration { .. } => NodeType::VariableDeclaration,
            Node::VariableDeclarationStatement { .. } => NodeType::VariableDeclarationStatement,
            Node::WhileStatement { .. } => NodeType::WhileStatement,
            Node::YulAssignment { .. } => NodeType::YulAssignment,
            Node::YulBlock { .. } => NodeType::YulBlock,
            Node::YulCase { .. } => NodeType::YulCase,
            Node::YulExpressionStatement { .. } => NodeType::YulExpressionStatement,
            Node::YulFunctionCall { .. } => NodeType::YulFunctionCall,
            Node::YulIdentifier { .. } => NodeType::YulIdentifier,
            Node::YulLiteral { .. } => NodeType::YulLiteral,
            Node::YulSwitch { .. } => NodeType::YulSwitch,
            Node::YulTypedName { .. } => NodeType::YulTypedName,
            Node::YulVariableDeclaration { .. } => NodeType::YulVariableDeclaration,
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
