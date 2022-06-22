use std::rc::Rc;

use super::*;

#[derive(Debug)]
pub enum Expression {
  Number(i32),

  String(String),

  Identifier(Box<IdentifierTerm>),

  FunctionCall(FunctionCall),

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),
  Error,
}

impl visitor::Visited for Expression {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      Expression::Number(_) => {}
      Expression::String(_) => {}
      Expression::Identifier(x) => x.accept(visitor),
      Expression::FunctionCall(x) => x.accept(visitor),
      Expression::Operation(x, _, y) => {
        x.accept(visitor);
        y.accept(visitor);
      }
      Expression::Error => todo!(),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum OperationCode {
  Mul,
  Div,
  Add,
  Sub,
  Comparison(ComparisonType),
}

#[derive(Debug)]
pub enum AssignmentType {
  Equal,
  PlusEqual,
  MinusEqual,
  AsteriskEqual,
  SlashEqual,
}

#[derive(Copy, Clone, Debug)]
pub enum ComparisonType {
  Greater,
  GreaterEqual,
  Lower,
  LowerEqual,
  Equal,
  Different,
}
