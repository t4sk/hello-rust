use serde::Deserialize;
use std::collections::HashMap;
use std::convert::From;
use std::convert::Into;

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

impl From<InheritanceSpecifier> for Node {
    fn from(item: InheritanceSpecifier) -> Node {
        Node::InheritanceSpecifier(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<EnumDefinition> for Node {
    fn from(item: EnumDefinition) -> Node {
        Node::EnumDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<ErrorDefinition> for Node {
    fn from(item: ErrorDefinition) -> Node {
        Node::ErrorDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<StructDefinition> for Node {
    fn from(item: StructDefinition) -> Node {
        Node::StructDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<UserDefinedValueTypeDefinition> for Node {
    fn from(item: UserDefinedValueTypeDefinition) -> Node {
        Node::UserDefinedValueTypeDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub id: NodeId,
    pub src: String,
}

impl From<UsingForDirective> for Node {
    fn from(item: UsingForDirective) -> Node {
        Node::UsingForDirective(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<EventDefinition> for Node {
    fn from(item: EventDefinition) -> Node {
        Node::EventDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub id: NodeId,
    pub src: String,
}

impl From<ModifierDefinition> for Node {
    fn from(item: ModifierDefinition) -> Node {
        Node::ModifierDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub id: NodeId,
    pub src: String,
}

impl From<ModifierInvocation> for Node {
    fn from(item: ModifierInvocation) -> Node {
        Node::ModifierInvocation(item)
    }
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

impl From<ContractNode> for Node {
    fn from(item: ContractNode) -> Node {
        match item {
            ContractNode::EnumDefinition(inner) => Node::EnumDefinition(inner),
            ContractNode::ErrorDefinition(inner) => Node::ErrorDefinition(inner),
            ContractNode::FunctionDefinition(inner) => Node::FunctionDefinition(inner),
            ContractNode::StructDefinition(inner) => Node::StructDefinition(inner),
            ContractNode::UserDefinedValueTypeDefinition(inner) => {
                Node::UserDefinedValueTypeDefinition(inner)
            }
            ContractNode::UsingForDirective(inner) => Node::UsingForDirective(inner),
            ContractNode::VariableDeclaration(inner) => Node::VariableDeclaration(inner),
            ContractNode::EventDefinition(inner) => Node::EventDefinition(inner),
            ContractNode::ModifierDefinition(inner) => Node::ModifierDefinition(inner),
            ContractNode::Unknown { id, nodes } => Node::Unknown { id, nodes },
        }
    }
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

impl From<ContractDefinition> for Node {
    fn from(item: ContractDefinition) -> Node {
        Node::ContractDefinition(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: NodeId,
    pub src: String,
    pub expression: Box<Expression>,
}

impl From<ExpressionStatement> for Node {
    fn from(item: ExpressionStatement) -> Node {
        Node::ExpressionStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Break {
    pub id: NodeId,
    pub src: String,
}

impl From<Break> for Node {
    fn from(item: Break) -> Node {
        Node::Break(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Continue {
    pub id: NodeId,
    pub src: String,
}

impl From<Continue> for Node {
    fn from(item: Continue) -> Node {
        Node::Continue(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<DoWhileStatement> for Node {
    fn from(item: DoWhileStatement) -> Node {
        Node::DoWhileStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<EmitStatement> for Node {
    fn from(item: EmitStatement) -> Node {
        Node::EmitStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<ForStatement> for Node {
    fn from(item: ForStatement) -> Node {
        Node::ForStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<IfStatement> for Node {
    fn from(item: IfStatement) -> Node {
        Node::IfStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: NodeId,
    pub src: String,
}

impl From<InlineAssembly> for Node {
    fn from(item: InlineAssembly) -> Node {
        Node::InlineAssembly(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<PlaceholderStatement> for Node {
    fn from(item: PlaceholderStatement) -> Node {
        Node::PlaceholderStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: NodeId,
    pub src: String,
}

impl From<Return> for Node {
    fn from(item: Return) -> Node {
        Node::Return(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<RevertStatement> for Node {
    fn from(item: RevertStatement) -> Node {
        Node::RevertStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<TryStatement> for Node {
    fn from(item: TryStatement) -> Node {
        Node::TryStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: NodeId,
    pub src: String,
}

impl From<UncheckedBlock> for Node {
    fn from(item: UncheckedBlock) -> Node {
        Node::UncheckedBlock(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<VariableDeclarationStatement> for Node {
    fn from(item: VariableDeclarationStatement) -> Node {
        Node::VariableDeclarationStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<WhileStatement> for Node {
    fn from(item: WhileStatement) -> Node {
        Node::WhileStatement(item)
    }
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

impl From<Statement> for Node {
    fn from(item: Statement) -> Node {
        match item {
            Statement::Block(item) => Node::Block(item),
            Statement::Break(item) => Node::Break(item),
            Statement::Continue(item) => Node::Continue(item),
            Statement::DoWhileStatement(item) => Node::DoWhileStatement(item),
            Statement::EmitStatement(item) => Node::EmitStatement(item),
            Statement::ExpressionStatement(item) => Node::ExpressionStatement(item),
            Statement::ForStatement(item) => Node::ForStatement(item),
            Statement::IfStatement(item) => Node::IfStatement(item),
            Statement::InlineAssembly(item) => Node::InlineAssembly(item),
            Statement::PlaceholderStatement(item) => Node::PlaceholderStatement(item),
            Statement::Return(item) => Node::Return(item),
            Statement::RevertStatement(item) => Node::RevertStatement(item),
            Statement::TryStatement(item) => Node::TryStatement(item),
            Statement::UncheckedBlock(item) => Node::UncheckedBlock(item),
            Statement::VariableDeclarationStatement(item) => {
                Node::VariableDeclarationStatement(item)
            }
            Statement::WhileStatement(item) => Node::WhileStatement(item),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: NodeId,
    pub src: String,
    pub statements: Option<Vec<Statement>>,
}

impl From<Block> for Node {
    fn from(item: Block) -> Node {
        Node::Block(item)
    }
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

impl From<Assignment> for Node {
    fn from(item: Assignment) -> Node {
        Node::Assignment(item)
    }
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

impl From<FunctionCall> for Node {
    fn from(item: FunctionCall) -> Node {
        Node::FunctionCall(item)
    }
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

impl From<FunctionCallOptions> for Node {
    fn from(item: FunctionCallOptions) -> Node {
        Node::FunctionCallOptions(item)
    }
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

impl From<Identifier> for Node {
    fn from(item: Identifier) -> Node {
        Node::Identifier(item)
    }
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

impl From<IndexAccess> for Node {
    fn from(item: IndexAccess) -> Node {
        Node::IndexAccess(item)
    }
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

impl From<MemberAccess> for Node {
    fn from(item: MemberAccess) -> Node {
        Node::MemberAccess(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: NodeId,
    pub src: String,
}

impl From<BinaryOperation> for Node {
    fn from(item: BinaryOperation) -> Node {
        Node::BinaryOperation(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub id: NodeId,
    pub src: String,
}

impl From<Conditional> for Node {
    fn from(item: Conditional) -> Node {
        Node::Conditional(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: NodeId,
    pub src: String,
}

impl From<ElementaryTypeNameExpression> for Node {
    fn from(item: ElementaryTypeNameExpression) -> Node {
        Node::ElementaryTypeNameExpression(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: NodeId,
    pub src: String,
}

impl From<IndexRangeAccess> for Node {
    fn from(item: IndexRangeAccess) -> Node {
        Node::IndexRangeAccess(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: NodeId,
    pub src: String,
}

impl From<Literal> for Node {
    fn from(item: Literal) -> Node {
        Node::Literal(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub id: NodeId,
    pub src: String,
}

impl From<NewExpression> for Node {
    fn from(item: NewExpression) -> Node {
        Node::NewExpression(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub id: NodeId,
    pub src: String,
}

impl From<TupleExpression> for Node {
    fn from(item: TupleExpression) -> Node {
        Node::TupleExpression(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub id: NodeId,
    pub src: String,
}

impl From<UnaryOperation> for Node {
    fn from(item: UnaryOperation) -> Node {
        Node::UnaryOperation(item)
    }
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

impl From<ArrayTypeName> for Node {
    fn from(item: ArrayTypeName) -> Node {
        Node::ArrayTypeName(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub state_mutability: Option<StateMutability>,
}

impl From<ElementaryTypeName> for Node {
    fn from(item: ElementaryTypeName) -> Node {
        Node::ElementaryTypeName(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub id: NodeId,
    pub src: String,
    pub state_mutability: StateMutability,
    pub visibility: Visibility,
}

impl From<FunctionTypeName> for Node {
    fn from(item: FunctionTypeName) -> Node {
        Node::FunctionTypeName(item)
    }
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

impl From<Mapping> for Node {
    fn from(item: Mapping) -> Node {
        Node::Mapping(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: NodeId,
    pub src: String,
    pub name: String,
    pub referenced_declaration: Option<NodeId>,
}

impl From<IdentifierPath> for Node {
    fn from(item: IdentifierPath) -> Node {
        Node::IdentifierPath(item)
    }
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

impl From<UserDefinedTypeName> for Node {
    fn from(item: UserDefinedTypeName) -> Node {
        Node::UserDefinedTypeName(item)
    }
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

impl From<TypeName> for Node {
    fn from(item: TypeName) -> Node {
        match item {
            TypeName::ArrayTypeName(item) => Node::ArrayTypeName(item),
            TypeName::ElementaryTypeName(item) => Node::ElementaryTypeName(item),
            TypeName::FunctionTypeName(item) => Node::FunctionTypeName(item),
            TypeName::Mapping(item) => Node::Mapping(item),
            TypeName::UserDefinedTypeName(item) => Node::UserDefinedTypeName(item),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

impl From<TypeDescriptions> for Node {
    fn from(item: TypeDescriptions) -> Node {
        Node::TypeDescriptions(item)
    }
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

impl From<VariableDeclaration> for Node {
    fn from(item: VariableDeclaration) -> Node {
        Node::VariableDeclaration(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: NodeId,
    pub src: String,
    pub parameters: Vec<VariableDeclaration>,
}

impl From<ParameterList> for Node {
    fn from(item: ParameterList) -> Node {
        Node::ParameterList(item)
    }
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

impl From<FunctionDefinition> for Node {
    fn from(item: FunctionDefinition) -> Node {
        Node::FunctionDefinition(item)
    }
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

impl From<ImportDirective> for Node {
    fn from(item: ImportDirective) -> Node {
        Node::ImportDirective(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub id: NodeId,
    pub src: String,
}

impl From<PragmaDirective> for Node {
    fn from(item: PragmaDirective) -> Node {
        Node::PragmaDirective(item)
    }
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

impl From<SourceUnitNode> for Node {
    fn from(item: SourceUnitNode) -> Node {
        match item {
            SourceUnitNode::ContractDefinition(item) => Node::ContractDefinition(item),
            SourceUnitNode::EnumDefinition(item) => Node::EnumDefinition(item),
            SourceUnitNode::ErrorDefinition(item) => Node::ErrorDefinition(item),
            SourceUnitNode::FunctionDefinition(item) => Node::FunctionDefinition(item),
            SourceUnitNode::ImportDirective(item) => Node::ImportDirective(item),
            SourceUnitNode::PragmaDirective(item) => Node::PragmaDirective(item),
            SourceUnitNode::StructDefinition(item) => Node::StructDefinition(item),
            SourceUnitNode::UserDefinedValueTypeDefinition(item) => {
                Node::UserDefinedValueTypeDefinition(item)
            }
            SourceUnitNode::UsingForDirective(item) => Node::UsingForDirective(item),
            SourceUnitNode::VariableDeclaration(item) => Node::VariableDeclaration(item),
        }
    }
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

impl From<SourceUnit> for Node {
    fn from(item: SourceUnit) -> Node {
        Node::SourceUnit(item)
    }
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

impl From<EnumValue> for Node {
    fn from(item: EnumValue) -> Node {
        Node::EnumValue(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub id: NodeId,
    pub src: String,
}

impl From<OverrideSpecifier> for Node {
    fn from(item: OverrideSpecifier) -> Node {
        Node::OverrideSpecifier(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub id: NodeId,
    pub src: String,
}

impl From<StructuredDocumentation> for Node {
    fn from(item: StructuredDocumentation) -> Node {
        Node::StructuredDocumentation(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Throw {
    pub id: NodeId,
    pub src: String,
}

impl From<Throw> for Node {
    fn from(item: Throw) -> Node {
        Node::Throw(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: NodeId,
    pub src: String,
}

impl From<TryCatchClause> for Node {
    fn from(item: TryCatchClause) -> Node {
        Node::TryCatchClause(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub id: NodeId,
    pub src: String,
}

impl From<YulAssignment> for Node {
    fn from(item: YulAssignment) -> Node {
        Node::YulAssignment(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub id: NodeId,
    pub src: String,
}

impl From<YulBlock> for Node {
    fn from(item: YulBlock) -> Node {
        Node::YulBlock(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub id: NodeId,
    pub src: String,
}

impl From<YulCase> for Node {
    fn from(item: YulCase) -> Node {
        Node::YulCase(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub id: NodeId,
    pub src: String,
}

impl From<YulExpressionStatement> for Node {
    fn from(item: YulExpressionStatement) -> Node {
        Node::YulExpressionStatement(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub id: NodeId,
    pub src: String,
}

impl From<YulFunctionCall> for Node {
    fn from(item: YulFunctionCall) -> Node {
        Node::YulFunctionCall(item)
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub id: NodeId,
    pub src: String,
}

impl From<YulIdentifier> for Node {
    fn from(item: YulIdentifier) -> Node {
        Node::YulIdentifier(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub id: NodeId,
    pub src: String,
}

impl From<YulLiteral> for Node {
    fn from(item: YulLiteral) -> Node {
        Node::YulLiteral(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub id: NodeId,
    pub src: String,
}

impl From<YulSwitch> for Node {
    fn from(item: YulSwitch) -> Node {
        Node::YulSwitch(item)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub id: NodeId,
    pub src: String,
}

impl From<YulTypedName> for Node {
    fn from(item: YulTypedName) -> Node {
        Node::YulTypedName(item)
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub id: NodeId,
    pub src: String,
}

impl From<YulVariableDeclaration> for Node {
    fn from(item: YulVariableDeclaration) -> Node {
        Node::YulVariableDeclaration(item)
    }
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
    TypeDescriptions(TypeDescriptions),
    UnaryOperation(UnaryOperation),
    UncheckedBlock(UncheckedBlock),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
    YulAssignment(YulAssignment),
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
    TypeDescriptions,
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
    pub fn nodes(&self) -> Option<&Vec<Node>> {
        match self {
            Node::ArrayTypeName(_) => None,
            Node::Assignment(_) => None,
            Node::BinaryOperation(_) => None,
            Node::Block(_) => None,
            Node::Break(_) => None,
            Node::Conditional(_) => None,
            Node::Continue(_) => None,
            Node::ContractDefinition(node) => None,
            Node::DoWhileStatement(_) => None,
            Node::ElementaryTypeName(_) => None,
            Node::ElementaryTypeNameExpression(_) => None,
            Node::EmitStatement(_) => None,
            Node::EnumDefinition(_) => None,
            Node::EnumValue(_) => None,
            Node::ErrorDefinition(_) => None,
            Node::EventDefinition(_) => None,
            Node::ExpressionStatement(_) => None,
            Node::ForStatement(_) => None,
            Node::FunctionCall(_) => None,
            Node::FunctionCallOptions(_) => None,
            Node::FunctionDefinition(_) => None,
            Node::FunctionTypeName(_) => None,
            Node::Identifier(_) => None,
            Node::IdentifierPath(_) => None,
            Node::IfStatement(_) => None,
            Node::ImportDirective(_) => None,
            Node::IndexAccess(_) => None,
            Node::IndexRangeAccess(_) => None,
            Node::InheritanceSpecifier(_) => None,
            Node::InlineAssembly(_) => None,
            Node::Literal(_) => None,
            Node::Mapping(_) => None,
            Node::MemberAccess(_) => None,
            Node::ModifierDefinition(_) => None,
            Node::ModifierInvocation(_) => None,
            Node::NewExpression(_) => None,
            Node::OverrideSpecifier(_) => None,
            Node::ParameterList(_) => None,
            Node::PlaceholderStatement(_) => None,
            Node::PragmaDirective(_) => None,
            Node::Return(_) => None,
            Node::RevertStatement(_) => None,
            Node::SourceUnit(_) => None,
            Node::StructDefinition(_) => None,
            Node::StructuredDocumentation(_) => None,
            Node::Throw(_) => None,
            Node::TryCatchClause(_) => None,
            Node::TryStatement(_) => None,
            Node::TupleExpression(_) => None,
            Node::TypeDescriptions(_) => None,
            Node::UnaryOperation(_) => None,
            Node::UncheckedBlock(_) => None,
            Node::UserDefinedTypeName(_) => None,
            Node::UserDefinedValueTypeDefinition(_) => None,
            Node::UsingForDirective(_) => None,
            Node::VariableDeclaration(_) => None,
            Node::VariableDeclarationStatement(_) => None,
            Node::WhileStatement(_) => None,
            Node::YulAssignment(_) => None,
            Node::YulBlock(_) => None,
            Node::YulCase(_) => None,
            Node::YulExpressionStatement(_) => None,
            Node::YulFunctionCall(_) => None,
            Node::YulIdentifier(_) => None,
            Node::YulLiteral(_) => None,
            Node::YulSwitch(_) => None,
            Node::YulTypedName(_) => None,
            Node::YulVariableDeclaration(_) => None,
            Node::Unknown { nodes, .. } => Some(nodes),
            _ => None,
        }
    }

    pub fn r#type(&self) -> NodeType {
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
            Node::TypeDescriptions(_) => NodeType::TypeDescriptions,
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
            _ => NodeType::Unknown,
        }
    }
}
