use std::rc::Rc;

use super::*;

#[derive(Debug)]
pub enum Expression {
  Integer(String),

  String(String),
  Name(String),

  Identifier(Box<IdentifierTerm>),

  FunctionCall(FunctionCall),
  ClassInstantiation(ClassInstantiation),
  Lambda(Lambda),

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),

  Not(Rc<Expression>),
  Nesting(Vec<Expression>),
  Cast(String, Rc<Expression>),

  Group(Rc<Expression>),

  Error,
}

impl visitor::Visited for Expression {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      Expression::Integer(_) | Expression::String(_) | Expression::Name(_) => {}
      Expression::Cast(_, x) => x.accept(visitor),
      Expression::Identifier(x) => x.accept(visitor),
      Expression::FunctionCall(x) => x.accept(visitor),
      Expression::Operation(x, _, y) => {
        x.accept(visitor);
        y.accept(visitor);
      }
      Expression::Nesting(x) => x.accept(visitor),
      Expression::Error => todo!(),
      Expression::Group(x) => x.accept(visitor),
      Expression::ClassInstantiation(x) => x.accept(visitor),
      Expression::Not(x) => x.accept(visitor),
      Expression::Lambda(x) => x.accept(visitor),
    }
  }
}

impl Codegen for Expression {
  fn emit(
    &self, context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      Expression::Integer(x) => write!(f, "{x}"),
      Expression::String(x) => write!(f, "{}", x),
      Expression::Name(x) => write!(f, "{}", x),
      Expression::Not(x) => {
        write!(f, "!")?;
        x.emit(context, f, data)
      }
      Expression::Identifier(x) => x.emit(context, f, data),
      Expression::FunctionCall(x) => x.emit(context, f, data),
      Expression::Operation(left, op, right) => {
        left.emit(context, f, data)?;
        op.emit(context, f, data)?;
        right.emit(context, f, data)
      }
      Expression::Error => todo!(),
      Expression::Nesting(x) => x.emit(context, f, data),
      Expression::Cast(t, x) => {
        write!(f, "({t})(")?;
        x.emit(context, f, data)?;
        write!(f, ")")
      }
      Expression::ClassInstantiation(x) => x.emit(context, f, data),
      Expression::Group(x) => {
        write!(f, "(")?;
        x.emit(context, f, data)?;
        write!(f, ")")
      }
      Expression::Lambda(x) => x.emit(context, f, data),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum OperationCode {
  Mul,
  Div,
  Add,
  Sub,
  Modulo,
  Nesting,
  BitwiseOr,
  BitwiseAnd,
  BooleanJoin(BooleanJoinType),
  Comparison(ComparisonType),
}

impl Codegen for OperationCode {
  fn emit(
    &self, context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      OperationCode::Mul => write!(f, "*"),
      OperationCode::Div => write!(f, "/"),
      OperationCode::Modulo => write!(f, "%"),
      OperationCode::Add => write!(f, "+"),
      OperationCode::Sub => write!(f, "-"),
      OperationCode::Nesting => write!(f, "."),
      OperationCode::Comparison(x) => x.emit(context, f, data),
      OperationCode::BooleanJoin(x) => x.emit(context, f, data),
      OperationCode::BitwiseOr => write!(f, "|"),
      OperationCode::BitwiseAnd => write!(f, "&"),
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
  fn emit(
    &self, _: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
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
  fn emit(
    &self, _: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
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

#[derive(Copy, Clone, Debug)]
pub enum BooleanJoinType {
  And,
  Or,
}

impl Codegen for BooleanJoinType {
  fn emit(
    &self, _: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      BooleanJoinType::And => write!(f, " && "),
      BooleanJoinType::Or => write!(f, " || "),
    }
  }
}
