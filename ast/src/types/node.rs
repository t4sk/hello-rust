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
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ContractDefinition {
//     pub id: u32,
//     pub nodes: Vec<Node>
// }

// #[derive(Debug, Deserialize)]
// #[serde(tag = "nodeType")]
// #[serde(rename_all = "camelCase")]
// pub enum Node {
//     ContractDefinition(ContractDefinition),
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
    pub nodes: Vec<Node>,
}
