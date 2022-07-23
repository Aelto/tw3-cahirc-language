use std::rc::Rc;

use super::*;

#[derive(Debug)]
pub enum Expression {
  Integer(i32),
  Float(f32),

  String(String),
  Name(String),

  Identifier(Box<IdentifierTerm>),

  FunctionCall(FunctionCall),
  ClassInstantiation(ClassInstantiation),

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),

  Not(Rc<Expression>),

  Error,
}

impl visitor::Visited for Expression {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      Expression::Integer(_)
      | Expression::Float(_)
      | Expression::String(_)
      | Expression::Name(_)
      | Expression::Not(_)
      | Expression::ClassInstantiation(_) => {}
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

impl Codegen for Expression {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      Expression::Integer(x) => write!(f, "{x}"),
      Expression::Float(x) => write!(f, "{x}f"),
      Expression::String(x) => write!(f, "{}", x),
      Expression::Name(x) => write!(f, "{}", x),
      Expression::Not(x) => {
        write!(f, "!")?;
        x.emit(context, f)
      }
      Expression::Identifier(x) => x.emit(context, f),
      Expression::FunctionCall(x) => x.emit(context, f),
      Expression::Operation(left, op, right) => {
        left.emit(context, f)?;
        write!(f, " ")?;
        op.emit(context, f)?;
        write!(f, " ")?;
        right.emit(context, f)
      }
      Expression::Error => todo!(),
      Expression::ClassInstantiation(x) => x.emit(context, f),
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

impl Codegen for OperationCode {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      OperationCode::Mul => write!(f, "*"),
      OperationCode::Div => write!(f, "/"),
      OperationCode::Add => write!(f, "+"),
      OperationCode::Sub => write!(f, "-"),
      OperationCode::Comparison(x) => x.emit(context, f),
    }
  }
}

#[derive(Debug)]
pub enum AssignmentType {
  Equal,
  PlusEqual,
  MinusEqual,
  AsteriskEqual,
  SlashEqual,
}

impl Codegen for AssignmentType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      AssignmentType::Equal => write!(f, "="),
      AssignmentType::PlusEqual => write!(f, "+="),
      AssignmentType::MinusEqual => write!(f, "-="),
      AssignmentType::AsteriskEqual => write!(f, "*="),
      AssignmentType::SlashEqual => write!(f, "/="),
    }
  }
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

impl Codegen for ComparisonType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      ComparisonType::Greater => write!(f, ">"),
      ComparisonType::GreaterEqual => write!(f, ">="),
      ComparisonType::Lower => write!(f, "<"),
      ComparisonType::LowerEqual => write!(f, "<="),
      ComparisonType::Equal => write!(f, "=="),
      ComparisonType::Different => write!(f, "!="),
    }
  }
}
