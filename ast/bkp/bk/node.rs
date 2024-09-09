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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Mutable,
    Immutable,
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Calldata,
    Default,
    Memory,
    Storage,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct FunctionCallOptions {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: u32,
    pub src: String,
    pub name: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub overloaded_declarations: Vec<u32>,
    pub referenced_declaration: Option<u32>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub base_expression: Box<Expression>,
    pub index_expression: Option<Box<Expression>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub base_expression: Box<Expression>,
    pub end_expression: Option<Box<Expression>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub start_expression: Option<Box<Expression>>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    String,
    Number,
    Bool,
    HexString,
    UnicodeString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub hex_value: String,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub kind: LiteralKind,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub subdenomination: String,
    pub type_descriptions: TypeDescriptions,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub member_location: String,
    pub member_name: String,
    pub reference_declaration: Option<u32>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
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
pub struct TupleExpression {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub components: Option<Vec<Expression>>,
    pub is_constant: bool,
    pub is_inline_array: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub id: u32,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub function: Option<u32>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub operator: String,
    pub prefix: bool,
    pub sub_expression: Box<Expression>,
    pub type_descriptions: TypeDescriptions,
}

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
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: u32,
    pub src: String,
    pub statements: Option<Vec<Statement>>,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub body: Box<Statement>,
    pub condition: Box<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub event_call: FunctionCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub expression: Box<Expression>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ExpressionStatementOrVariableDeclarationStatement {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub body: Box<Statement>,
    pub condition: Option<Expression>,
    pub initialization_expression: Option<ExpressionStatementOrVariableDeclarationStatement>,
    pub is_simple_counter_loop: bool,
    pub loope_expression: Option<ExpressionStatement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub condition: Box<Statement>,
    pub false_body: Option<Box<Statement>>,
    pub true_body: Box<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: u32,
    pub src: String,
    pub block: Block,
    pub error_name: String,
    pub parameters: Option<ParameterList>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    #[serde(rename = "AST")]
    pub ast: YulBlock,
    pub evm_version: String,
    pub external_references: Vec<ExternalReference>,
    pub flags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub expression: Option<Expression>,
    pub function_return_parameters: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub error_call: FunctionCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub clauses: Vec<TryCatchClause>,
    pub external_call: FunctionCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub assignments: Option<Vec<u32>>,
    pub declarations: Option<Vec<VariableDeclaration>>,
    pub initial_value: Option<Expression>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: u32,
    pub src: String,
    pub documentation: Option<String>,
    pub condition: Expression,
    pub body: Box<Statement>,
}

// Statement
#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    EmitStatement(EmitStatement),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    IfStatement(IfStatement),
    InlineAssembly(InlineAssembly),
    PlaceholderStatement(PlaceholderStatement),
    Return(Return),
    RevertStatement(RevertStatement),
    TryStatement(TryStatement),
    UncheckedBlock(UncheckedBlock),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub src: String,
    pub native_src: Option<String>,
    pub arguments: Vec<YulExpression>,
    pub function_name: YulIdentifier,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub src: String,
    pub native_src: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "nodeType")]
pub enum YulLiteral {
    YulLiteralValue(YulLiteralValue),
    YulLiteralHexValue(YulLiteralHexValue),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteralValue {
    pub src: String,
    pub native_src: Option<String>,
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteralHexValue {
    pub src: String,
    pub native_src: Option<String>,
    pub kind: String,
    pub hex_value: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub src: String,
    pub native_src: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum YulStatement {
    YulAssignment,
    YulBlock,
    YulBreak,
    YulContinue,
    YulExpressionStatement,
    YulIf,
    YulSwitch,
    YulVariableDeclaration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub src: String,
    pub native_src: Option<String>,
    pub value: YulExpression,
    pub variable_names: Vec<YulIdentifier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub src: String,
    pub native_src: Option<String>,
    pub statements: Vec<YulStatement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulBreak {
    pub src: String,
    pub native_src: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulContinue {
    pub src: String,
    pub native_src: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub src: String,
    pub native_src: Option<String>,
    pub expression: YulExpression,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
    pub src: String,
    pub native_src: Option<String>,
    pub body: YulBlock,
    pub condition: YulExpression,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub src: String,
    pub native_src: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub src: String,
    pub native_src: Option<String>,
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ExternalReference {
    // TODO:
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

// TODO
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub r#abstract: bool,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub canonical_name: Option<String>,
    pub contract_dependencies: Vec<u32>,
    // contract | interface | library
    pub contract_kind: String,
    pub documentation: Option<StructuredDocumentation>,
    pub src: String,
    pub native_src: Option<String>,
    pub value: Option<YulExpression>,
    pub variables: Vec<YulTypedName>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Node {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Break(Break),
    Conditional(Conditional),
    Continue(Continue),
    ContractDefinition(ContractDefinition),
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
            Node::ArrayTypeName(_) => NodeType::ArrayTypeName,
            Node::Assignment(_) => NodeType::Assignment,
            Node::BinaryOperation(_) => NodeType::BinaryOperation,
            Node::Block(_) => NodeType::Block,
            Node::Break(_) => NodeType::Break,
            Node::Conditional(_) => NodeType::Conditional,
            Node::Continue(_) => NodeType::Continue,
            Node::ContractDefinition(_) => NodeType::ContractDefinition,
            Node::DoWhileStatement(_) => NodeType::DoWhileStatement,
            Node::ElementaryTypeName(_) => NodeType::ElementaryTypeName,
            Node::ElementaryTypeNameExpression(_) => NodeType::ElementaryTypeNameExpression,
            Node::EmitStatement(_) => NodeType::EmitStatement,
            Node::EnumDefinition(_) => NodeType::EnumDefinition,
            Node::EnumValue(_) => NodeType::EnumValue,
            Node::ErrorDefinition(_) => NodeType::ErrorDefinition,
            Node::EventDefinition(_) => NodeType::EventDefinition,
            Node::ExpressionStatement(_) => NodeType::ExpressionStatement,
            Node::ForStatement(_) => NodeType::ForStatement,
            Node::FunctionCall(_) => NodeType::FunctionCall,
            Node::FunctionCallOptions(_) => NodeType::FunctionCallOptions,
            Node::FunctionDefinition(_) => NodeType::FunctionDefinition,
            Node::FunctionTypeName(_) => NodeType::FunctionTypeName,
            Node::Identifier(_) => NodeType::Identifier,
            Node::IdentifierPath(_) => NodeType::IdentifierPath,
            Node::IfStatement(_) => NodeType::IfStatement,
            Node::ImportDirective(_) => NodeType::ImportDirective,
            Node::IndexAccess(_) => NodeType::IndexAccess,
            Node::IndexRangeAccess(_) => NodeType::IndexRangeAccess,
            Node::InheritanceSpecifier(_) => NodeType::InheritanceSpecifier,
            Node::InlineAssembly(_) => NodeType::InlineAssembly,
            Node::Literal(_) => NodeType::Literal,
            Node::Mapping(_) => NodeType::Mapping,
            Node::MemberAccess(_) => NodeType::MemberAccess,
            Node::ModifierDefinition(_) => NodeType::ModifierDefinition,
            Node::ModifierInvocation(_) => NodeType::ModifierInvocation,
            Node::NewExpression(_) => NodeType::NewExpression,
            Node::OverrideSpecifier(_) => NodeType::OverrideSpecifier,
            Node::ParameterList(_) => NodeType::ParameterList,
            Node::PlaceholderStatement(_) => NodeType::PlaceholderStatement,
            Node::PragmaDirective(_) => NodeType::PragmaDirective,
            Node::Return(_) => NodeType::Return,
            Node::RevertStatement(_) => NodeType::RevertStatement,
            Node::SourceUnit(_) => NodeType::SourceUnit,
            Node::StructDefinition(_) => NodeType::StructDefinition,
            Node::StructuredDocumentation(_) => NodeType::StructuredDocumentation,
            Node::Throw(_) => NodeType::Throw,
            Node::TryCatchClause(_) => NodeType::TryCatchClause,
            Node::TryStatement(_) => NodeType::TryStatement,
            Node::TupleExpression(_) => NodeType::TupleExpression,
            Node::UnaryOperation(_) => NodeType::UnaryOperation,
            Node::UncheckedBlock(_) => NodeType::UncheckedBlock,
            Node::UserDefinedTypeName(_) => NodeType::UserDefinedTypeName,
            Node::UserDefinedValueTypeDefinition(_) => NodeType::UserDefinedValueTypeDefinition,
            Node::UsingForDirective(_) => NodeType::UsingForDirective,
            Node::VariableDeclaration(_) => NodeType::VariableDeclaration,
            Node::VariableDeclarationStatement(_) => NodeType::VariableDeclarationStatement,
            Node::WhileStatement(_) => NodeType::WhileStatement,
            Node::YulAssignment(_) => NodeType::YulAssignment,
            Node::YulBlock(_) => NodeType::YulBlock,
            Node::YulCase(_) => NodeType::YulCase,
            Node::YulExpressionStatement(_) => NodeType::YulExpressionStatement,
            Node::YulFunctionCall(_) => NodeType::YulFunctionCall,
            Node::YulIdentifier(_) => NodeType::YulIdentifier,
            Node::YulLiteral(_) => NodeType::YulLiteral,
            Node::YulSwitch(_) => NodeType::YulSwitch,
            Node::YulTypedName(_) => NodeType::YulTypedName,
            Node::YulVariableDeclaration(_) => NodeType::YulVariableDeclaration,
            Node::Unknown(_) => NodeType::Unknown,
        }
    }

    pub fn get_nodes(&self) -> Option<&Vec<Node>> {
        match self {
            Node::SourceUnit { nodes, .. } => Some(&nodes),
            // Node::ContractDefinition(node) => Some(node),
            Node::FunctionDefinition { nodes, .. } => Some(&nodes),
            Node::Unknown { nodes, .. } => Some(&nodes),
            _ => None,
        }
    }
}
