use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;

// -----------------------------------------------------------------------------

pub struct ProgramInformation {
  pub generic_functions: RefCell<Vec<Rc<FunctionDeclaration>>>
}

impl ProgramInformation {
  pub fn new() -> Self {
    Self { generic_functions: RefCell::new(Vec::new()) }
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Statement {
  Expression(Box<Expression>),
  FunctionDeclaration(Rc<FunctionDeclaration>),
  ClassDeclaration(ClassDeclaration),
  StructDeclaration(StructDeclaration)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,
  pub body_statements: Vec<ClassBodyStatement>
}

#[derive(Debug)]
pub enum ClassType {
  Class,
  StatemachineClass
}

#[derive(Debug)]
pub enum ClassBodyStatement {
  Method {
    encapsulation: Option<EncapsulationType>,
    function_declaration: Rc<FunctionDeclaration>
  },
  Property {
    encapsulation: Option<EncapsulationType>,
    property_declaration: VariableDeclaration
  },
  DefaultValue(VariableAssignment)
}

#[derive(Debug)]
pub enum EncapsulationType {
  Private,
  Public,
  Protected
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct StructDeclaration {
  pub name: String,
  pub body_statements: Vec<StructBodyStatement>
}

#[derive(Debug)]
pub enum StructBodyStatement {
  Property(VariableDeclaration),
  DefaultValue(VariableAssignment)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
/// property.
pub struct FunctionDeclaration {
  pub function_type: FunctionType,
  pub name: String,
  pub generic_types: Option<Vec<String>>,
  pub parameters: Vec<TypedIdentifier>,
  pub type_declaration: Option<TypeDeclaration>,
  pub body_statements: Vec<FunctionBodyStatement>,
  pub is_latent: bool
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Box<Expression>),
  Return(Box<Expression>),
  Assignement(VariableAssignment),
  IfStatement(IfStatement),
  ForStatement(ForStatement),
  WhileStatement(WhileStatement),
  DoWhileStatement(DoWhileStatement)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum IfStatement {
  If {
    condition: Box<Expression>,
    body_statements: Vec<FunctionBodyStatement>,
    else_statements: Vec<Box<IfStatement>>
  },
  Else {
    condition: Option<Box<Expression>>,
    body_statements: Vec<FunctionBodyStatement>
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: IdentifierTerm,
  pub assignment_type: AssignmentType,
  pub following_expression: Box<Expression>
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ForStatement {
  pub initialization: Option<VariableDeclarationOrAssignment>,
  pub condition: Box<Expression>,
  pub iteration: VariableAssignment,
  pub body_statements: Vec<FunctionBodyStatement>
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignment {
  Declaration(VariableDeclaration),
  Assignement(VariableAssignment)
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: Box<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

#[derive(Debug)]
pub struct DoWhileStatement {
  pub condition: Box<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

// -----------------------------------------------------------------------------


#[derive(Debug)]
pub struct VariableDeclaration {
  pub declaration: TypedIdentifier,
  pub following_expression: Option<Box<Expression>>
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Box<Expression>>);

#[derive(Debug)]
pub struct FunctionCall {
  pub accessor: IdentifierTerm,
  pub parameters: FunctionCallParameters
}

#[derive(Debug)]
pub enum IdentifierTerm {
  Identifier(String),
  NestedIdentifiers(Vec<String>),
}

#[derive(Debug)]
pub struct TypedIdentifier {
  pub name: String,
  pub type_declaration: TypeDeclaration
}

/// Represents a type declaration that could be after anything, for example
/// ```
/// a: int
/// ```
/// 
/// `: int` is the typeDeclaration
#[derive(Debug)]
pub struct TypeDeclaration {
  pub type_name: String,
  pub generic_type_assignment: Option<Vec<TypeDeclaration>>
}

#[derive(Debug)]
pub enum Expression {
  Number(i32),

  String(String),

  Identifier(IdentifierTerm),

  FunctionCall {
    accessor: IdentifierTerm,
    parameters: FunctionCallParameters
  },

  /// An operation between two expressions
  Operation(Box<Expression>, OperationCode, Box<Expression>),
  Error,
}

#[derive(Copy, Clone, Debug)]
pub enum OperationCode {
  Mul,
  Div,
  Add,
  Sub,
  Comparison(ComparisonType)
}

#[derive(Debug)]
pub enum AssignmentType {
  Equal,
  PlusEqual,
  MinusEqual,
  AsteriskEqual,
  SlashEqual
}

#[derive(Copy, Clone, Debug)]
pub enum ComparisonType {
  Greater,
  GreaterEqual,
  Lower,
  LowerEqual,
  Equal,
  Different
}