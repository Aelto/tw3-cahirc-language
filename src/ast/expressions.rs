use std::rc::Rc;

use super::*;


#[derive(Debug)]
pub enum Expression {
  Number(i32),

  String(String),

  Identifier(Box<IdentifierTerm>),

  FunctionCall {
    accessor: Box<IdentifierTerm>,
    generic_types: Option<Vec<String>>,
    parameters: FunctionCallParameters
  },

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),
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
