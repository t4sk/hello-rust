use serde::Deserialize;
use std::collections::HashMap;

pub type NodeId = i64;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Mutable,
    Immutable,
    Constant,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Payable,
    NonPayable,
    Pure,
    View,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Internal,
    Public,
    External,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Calldata,
    Default,
    Memory,
    Storage,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Constructor,
    Receive,
    Fallback,
    FreeFunction,
    Function,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "nodeType")]
pub enum BaseName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub id: NodeId,
    pub src: String,
    pub base_name: BaseName,
    pub arguments: Option<Vec<Expression>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "nodeType")]
pub enum ContractNode {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    ModifierDefinition(ModifierDefinition),
    #[serde(untagged)]
    Unknown {
        id: NodeId,
        nodes: Vec<Node>,
    },
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub r#abstract: bool,
    pub id: NodeId,
    pub src: String,
    // pub native_src: Option<String>,
    pub name: String,
    pub contract_kind: ContractKind,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub contract_dependencies: Vec<NodeId>,
    pub linearized_base_contracts: Vec<NodeId>,
    pub nodes: Vec<ContractNode>,
    // pub nodes: Vec<Node>,
    /*
    canonicalName?: string;
    contractDependencies: number[];
    contractKind: "contract" | "interface" | "library";
    documentation?: null | StructuredDocumentation;
    fullyImplemented: boolean;
    id: number;
    internalFunctionIDs?: {
        [k: string]: number | undefined;
    };
    linearizedBaseContracts: number[];
    name: string;
    nameLocation?: string;
    scope: number;
    src: string;
    usedErrors?: number[];
    usedEvents?: number[];
    */
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: NodeId,
    pub src: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: NodeId,
    pub src: String,
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: NodeId,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub arguments: Vec<Expression>,
    pub expression: Box<Expression>,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub id: NodeId,
    pub src: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: Option<bool>,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub overloaded_declarations: Vec<NodeId>,
    pub referenced_declaration: Option<NodeId>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub id: NodeId,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    pub member_location: Option<String>,
    pub member_name: String,
    pub referenced_declaration: Option<NodeId>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub id: NodeId,
    pub src: String,
    pub base_type: Box<TypeName>,
    pub length: Option<Expression>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub state_mutability: Option<StateMutability>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub id: NodeId,
    pub src: String,
    pub state_mutability: StateMutability,
    pub visibility: Visibility,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub id: NodeId,
    pub src: String,
    pub key_name: Option<String>,
    pub key_name_location: Option<String>,
    pub key_type: Box<TypeName>,
    pub value_name: Option<String>,
    pub value_name_location: Option<String>,
    pub value_type: Box<TypeName>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub referenced_declaration: Option<NodeId>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub id: NodeId,
    pub src: String,
    pub name: Option<String>,
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: NodeId,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub state_variable: bool,
    pub constant: bool,
    pub function_selector: Option<String>,
    pub mutability: Mutability,
    pub storage_location: StorageLocation,
    pub visibility: Visibility,
    pub type_name: Option<TypeName>,
    pub value: Option<Expression>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: NodeId,
    pub src: String,
    pub parameters: Vec<VariableDeclaration>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    pub id: NodeId,
    pub src: String,
    pub nodes: Vec<Node>,
    pub kind: FunctionKind,
    pub name: String,
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    pub function_selector: Option<String>,
    pub body: Option<Block>,
    pub parameters: ParameterList,
    pub return_parameters: ParameterList,
}
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Identifier,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub absolute_path: String,
    pub file: String,
    pub id: NodeId,
    pub name_location: Option<String>,
    pub scope: NodeId,
    pub source_unit: NodeId,
    pub src: String,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub unit_alias: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    ContractDefinition(ContractDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    /*
    #[serde(untagged)]
    Unknown {
        id: NodeId,
    },
    */
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub id: NodeId,
    pub src: String,
    pub absolute_path: String,
    pub nodes: Vec<SourceUnitNode>,
    pub exported_symbols: Option<HashMap<String, Vec<NodeId>>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Ast {
    // pub ast: Value
    pub ast: SourceUnit,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Throw {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub id: NodeId,
    pub src: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    DoWhileStatement(DoWhileStatement),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    InlineAssembly(InlineAssembly),
    Literal(Literal),
    Mapping(Mapping),
    MemberAccess(MemberAccess),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    NewExpression(NewExpression),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PlaceholderStatement(PlaceholderStatement),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    Throw(Throw),
    TryCatchClause(TryCatchClause),
    TryStatement(TryStatement),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UncheckedBlock(UncheckedBlock),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
    YulBlock(YulBlock),
    YulCase(YulCase),
    YulExpressionStatement(YulExpressionStatement),
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
    YulSwitch(YulSwitch),
    YulTypedName(YulTypedName),
    YulVariableDeclaration(YulVariableDeclaration),
    #[serde(untagged)]
    Unknown {
        id: NodeId,
        nodes: Vec<Node>,
    },
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

impl Node {
    pub fn r#type(&self) -> NodeType {
        match self {
            // Node::ArrayTypeName { .. } => NodeType::ArrayTypeName,
            // Node::Assignment { .. } => NodeType::Assignment,
            // Node::BinaryOperation { .. } => NodeType::BinaryOperation,
            // Node::Block { .. } => NodeType::Block,
            // Node::Break { .. } => NodeType::Break,
            // Node::Conditional { .. } => NodeType::Conditional,
            // Node::Continue { .. } => NodeType::Continue,
            Node::ContractDefinition(_) => NodeType::ContractDefinition,
            // Node::DoWhileStatement { .. } => NodeType::DoWhileStatement,
            // Node::ElementaryTypeName { .. } => NodeType::ElementaryTypeName,
            // Node::ElementaryTypeNameExpression { .. } => NodeType::ElementaryTypeNameExpression,
            // Node::EmitStatement { .. } => NodeType::EmitStatement,
            // Node::EnumDefinition { .. } => NodeType::EnumDefinition,
            // Node::EnumValue { .. } => NodeType::EnumValue,
            // Node::ErrorDefinition { .. } => NodeType::ErrorDefinition,
            // Node::EventDefinition { .. } => NodeType::EventDefinition,
            // Node::ExpressionStatement { .. } => NodeType::ExpressionStatement,
            // Node::ForStatement { .. } => NodeType::ForStatement,
            // Node::FunctionCall { .. } => NodeType::FunctionCall,
            // Node::FunctionCallOptions { .. } => NodeType::FunctionCallOptions,
            // Node::FunctionDefinition { .. } => NodeType::FunctionDefinition,
            // Node::FunctionTypeName { .. } => NodeType::FunctionTypeName,
            // Node::Identifier { .. } => NodeType::Identifier,
            // Node::IdentifierPath { .. } => NodeType::IdentifierPath,
            // Node::IfStatement { .. } => NodeType::IfStatement,
            // Node::ImportDirective { .. } => NodeType::ImportDirective,
            // Node::IndexAccess { .. } => NodeType::IndexAccess,
            // Node::IndexRangeAccess { .. } => NodeType::IndexRangeAccess,
            // Node::InheritanceSpecifier { .. } => NodeType::InheritanceSpecifier,
            // Node::InlineAssembly { .. } => NodeType::InlineAssembly,
            // Node::Literal { .. } => NodeType::Literal,
            // Node::Mapping { .. } => NodeType::Mapping,
            // Node::MemberAccess { .. } => NodeType::MemberAccess,
            // Node::ModifierDefinition { .. } => NodeType::ModifierDefinition,
            // Node::ModifierInvocation { .. } => NodeType::ModifierInvocation,
            // Node::NewExpression { .. } => NodeType::NewExpression,
            // Node::OverrideSpecifier { .. } => NodeType::OverrideSpecifier,
            // Node::ParameterList { .. } => NodeType::ParameterList,
            // Node::PlaceholderStatement { .. } => NodeType::PlaceholderStatement,
            // Node::PragmaDirective { .. } => NodeType::PragmaDirective,
            // Node::Return { .. } => NodeType::Return,
            // Node::RevertStatement { .. } => NodeType::RevertStatement,
            // Node::SourceUnit { .. } => NodeType::SourceUnit,
            // Node::StructDefinition { .. } => NodeType::StructDefinition,
            // Node::StructuredDocumentation { .. } => NodeType::StructuredDocumentation,
            // Node::Throw { .. } => NodeType::Throw,
            // Node::TryCatchClause { .. } => NodeType::TryCatchClause,
            // Node::TryStatement { .. } => NodeType::TryStatement,
            // Node::TupleExpression { .. } => NodeType::TupleExpression,
            // Node::UnaryOperation { .. } => NodeType::UnaryOperation,
            // Node::UncheckedBlock { .. } => NodeType::UncheckedBlock,
            // Node::UserDefinedTypeName { .. } => NodeType::UserDefinedTypeName,
            // Node::UserDefinedValueTypeDefinition { .. } => NodeType::UserDefinedValueTypeDefinition,
            // Node::UsingForDirective { .. } => NodeType::UsingForDirective,
            // Node::VariableDeclaration { .. } => NodeType::VariableDeclaration,
            // Node::VariableDeclarationStatement { .. } => NodeType::VariableDeclarationStatement,
            // Node::WhileStatement { .. } => NodeType::WhileStatement,
            // Node::YulAssignment { .. } => NodeType::YulAssignment,
            // Node::YulBlock { .. } => NodeType::YulBlock,
            // Node::YulCase { .. } => NodeType::YulCase,
            // Node::YulExpressionStatement { .. } => NodeType::YulExpressionStatement,
            // Node::YulFunctionCall { .. } => NodeType::YulFunctionCall,
            // Node::YulIdentifier { .. } => NodeType::YulIdentifier,
            // Node::YulLiteral { .. } => NodeType::YulLiteral,
            // Node::YulSwitch { .. } => NodeType::YulSwitch,
            // Node::YulTypedName { .. } => NodeType::YulTypedName,
            // Node::YulVariableDeclaration { .. } => NodeType::YulVariableDeclaration,
            _ => NodeType::Unknown,
        }
    }
}
