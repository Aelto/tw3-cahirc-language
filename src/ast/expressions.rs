use std::rc::Rc;

use super::{*, inference::ToType};

#[derive(Debug)]
pub enum Expression {
  Integer(String),
  Float(String),

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
      Expression::Integer(_) | Expression::Float(_) | Expression::String(_) | Expression::Name(_) => {}
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
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      Expression::Integer(x) => write!(f, "{x}"),
      Expression::Float(x) => write!(f, "{x}"),
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
        op.emit(context, f)?;
        right.emit(context, f)
      }
      Expression::Error => todo!(),
      Expression::Nesting(x) => x.emit(context, f),
      Expression::Cast(t, x) => {
        write!(f, "({t})(")?;
        x.emit(context, f)?;
        write!(f, ")")
      }
      Expression::ClassInstantiation(x) => x.emit(context, f),
      Expression::Group(x) => {
        write!(f, "(")?;
        x.emit(context, f)?;
        write!(f, ")")
      }
      Expression::Lambda(x) => x.emit(context, f),
    }
  }
}

impl ToType for Expression {
  fn resulting_type(
    &self,
      current_context: &Rc<RefCell<Context>>,
      inference_store: &codegen::type_inference::TypeInferenceStore
    ) -> inference::Type {
      match self {
        Expression::Integer(_) => inference::Type::Int,
        Expression::Float(_) => inference::Type::Float,
        Expression::String(_) => inference::Type::String,
        Expression::Name(_) => inference::Type::Name,
        Expression::Identifier(identifier) => {
          match current_context.borrow_mut().local_variables_inference.get(&identifier.text) {
            Some(t) => inference::Type::Identifier(t.clone()),
            None => inference::Type::Unknown
          }
        },
        Expression::FunctionCall(function) => {
          let function_return_type = match inference_store.types.get(&function.accessor.text) {
              Some(infered_type) => match infered_type {
                crate::ast::codegen::type_inference::InferedType::Function { parameters: _, return_type } => match return_type {
                  Some(s) => inference::Type::Identifier(s.clone()),
                  None => inference::Type::Void
                },
                _ => {
                  println!("function call {}(), but {} is not a function", &function.accessor.text, &function.accessor.text);

                  inference::Type::Unknown
                },
              },
              None => todo!(),
          };

          function_return_type
        },
        Expression::ClassInstantiation(instantiation) => inference::Type::Identifier(instantiation.class_name.clone()),
        Expression::Lambda(_) => inference::Type::Unknown,
        Expression::Operation(_, _, _) => todo!(),
        Expression::Not(_) => inference::Type::Bool,
        Expression::Nesting(_) => todo!(),
        Expression::Cast(type_name, _) => inference::Type::Identifier(type_name.clone()),
        Expression::Group(_) => todo!(),
        Expression::Error => inference::Type::Unknown,
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
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      OperationCode::Mul => write!(f, "*"),
      OperationCode::Div => write!(f, "/"),
      OperationCode::Modulo => write!(f, "%"),
      OperationCode::Add => write!(f, "+"),
      OperationCode::Sub => write!(f, "-"),
      OperationCode::Nesting => write!(f, "."),
      OperationCode::Comparison(x) => x.emit(context, f),
      OperationCode::BooleanJoin(x) => x.emit(context, f),
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

#[derive(Copy, Clone, Debug)]
pub enum BooleanJoinType {
  And,
  Or,
}

impl Codegen for BooleanJoinType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      BooleanJoinType::And => write!(f, " && "),
      BooleanJoinType::Or => write!(f, " || "),
    }
  }
}
