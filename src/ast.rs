use std::fmt::{Debug};

#[derive(Debug)]
pub enum Statement {
  Expression(Box<Expression>),
  FunctionDeclaration(FunctionDeclaration)
}

#[derive(Debug)]
pub enum FunctionDeclaration {
  Function {
    name: String,
    parameters: Vec<TypedIdentifier>,
    type_declaration: Option<TypeDeclaration>,
    body_statements: Vec<FunctionBodyStatement>
  },
  Event {

  },
  Timer {

  }
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Box<Expression>),
  Return(Box<Expression>),
  Assignement {
    variable_name: IdentifierTerm,
    assignment_type: AssignmentType,
    following_expression: Box<Expression>
  },
  IfStatement(IfStatement)
}

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
  pub type_name: String
}

/// Represents a type declaration that could be after anything, for example
/// ```
/// a: int
/// ```
/// 
/// `: int` is the typeDeclaration
#[derive(Debug)]
pub struct TypeDeclaration {
  pub type_name: String
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