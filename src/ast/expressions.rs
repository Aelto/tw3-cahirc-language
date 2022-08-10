use std::{rc::Rc, borrow::Borrow};

use ariadne::{Report, Label};

use super::{*, inference::ToType, codegen::type_inference::InferedType};

#[derive(Debug)]
pub struct Expression {
  pub infered_type: InferedType,
  pub body: ExpressionBody
}

impl Expression {
  pub fn new(body: ExpressionBody) -> Self {
    Self {
      infered_type: InferedType::Unknown,
      body
    }
  }
}

impl visitor::Visited for Expression {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.body.accept(visitor);
  }
}

impl Codegen for Expression {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.body.emit(context, f)
  }
}


#[derive(Debug)]
pub enum ExpressionBody {
  Integer(Spanned<String>),
  Float(Spanned<String>),

  String(Spanned<String>),
  Name(Spanned<String>),

  Identifier(Box<IdentifierTerm>),

  FunctionCall(FunctionCall),
  ClassInstantiation(ClassInstantiation),
  Lambda(Lambda),

  /// An operation between two expressions
  Operation(Rc<Expression>, OperationCode, Rc<Expression>),

  Not(Rc<Expression>),
  Nesting(Vec<Expression>),
  Cast(String, Rc<Expression>),

  /// Expressions surrounded by parenthesis
  Group(Rc<Expression>),

  Error,
}

impl visitor::Visited for ExpressionBody {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      ExpressionBody::Integer(_) | ExpressionBody::Float(_) | ExpressionBody::String(_) | ExpressionBody::Name(_) => {}
      ExpressionBody::Cast(_, x) => x.accept(visitor),
      ExpressionBody::Identifier(x) => x.accept(visitor),
      ExpressionBody::FunctionCall(x) => x.accept(visitor),
      ExpressionBody::Operation(x, _, y) => {
        x.accept(visitor);
        y.accept(visitor);
      }
      ExpressionBody::Nesting(x) => x.accept(visitor),
      ExpressionBody::Error => todo!(),
      ExpressionBody::Group(x) => x.accept(visitor),
      ExpressionBody::ClassInstantiation(x) => x.accept(visitor),
      ExpressionBody::Not(x) => x.accept(visitor),
      ExpressionBody::Lambda(x) => x.accept(visitor),
    }
  }
}

impl Codegen for ExpressionBody {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      ExpressionBody::Integer(x) => x.emit(context, f),
      ExpressionBody::Float(x) => x.emit(context, f),
      ExpressionBody::String(x) => x.emit(context, f),
      ExpressionBody::Name(x) => x.emit(context, f),
      ExpressionBody::Not(x) => {
        write!(f, "!")?;
        x.emit(context, f)
      }
      ExpressionBody::Identifier(x) => x.emit(context, f),
      ExpressionBody::FunctionCall(x) => x.emit(context, f),
      ExpressionBody::Operation(left, op, right) => {
        left.emit(context, f)?;
        op.emit(context, f)?;
        right.emit(context, f)
      }
      ExpressionBody::Error => todo!(),
      ExpressionBody::Nesting(x) => x.emit(context, f),
      ExpressionBody::Cast(t, x) => {
        write!(f, "({t})(")?;
        x.emit(context, f)?;
        write!(f, ")")
      }
      ExpressionBody::ClassInstantiation(x) => x.emit(context, f),
      ExpressionBody::Group(x) => {
        write!(f, "(")?;
        x.emit(context, f)?;
        write!(f, ")")
      }
      ExpressionBody::Lambda(x) => x.emit(context, f),
    }
  }
}

impl ToType for ExpressionBody {
  /// Warning: this function mutates some of the nodes if the inference succeeded.
  /// Ideally move the mutating to another function to make it cleaner. Though it
  /// wouldn't change much in terms of functionnality, but it would also make
  /// performances worse.
  fn resulting_type(
    &self,
      current_context: &Rc<RefCell<Context>>,
      inference_map: &codegen::type_inference::TypeInferenceMap,
      span_manager: &SpanManager
    ) -> Result<inference::Type, Vec<Report>> {
      match self {
        ExpressionBody::Integer(_) => Ok(inference::Type::Int),
        ExpressionBody::Float(_) => Ok(inference::Type::Float),
        ExpressionBody::String(_) => Ok(inference::Type::String),
        ExpressionBody::Name(_) => Ok(inference::Type::Name),
        ExpressionBody::Identifier(identifier) => {
          let a: &IdentifierTerm = &identifier.borrow();

          if a.text == "this" {
            match Self::get_type_for_this(current_context, inference_map) {
              Ok(t) => Ok(t),
              Err(message) => Err(vec![
                Report::build(ariadne::ReportKind::Error, (), span_manager.get_left(identifier.span))
                  .with_message(&"Could not infer type for `this`")
                  .with_label(
                    Label::new(span_manager.get_range(identifier.span))
                    .with_message(&message)
                  )
                  .finish()
              ])
            }
          }
          else if a.text == "parent" {
            match Self::get_type_for_parent(current_context, inference_map) {
              Ok(t) => Ok(t),
              Err(message) => Err(vec![
                Report::build(ariadne::ReportKind::Error, (), span_manager.get_left(identifier.span))
                  .with_message(&"Could not infer type for `parent`")
                  .with_label(
                    Label::new(span_manager.get_range(identifier.span))
                    .with_message(&message)
                  )
                  .finish()
              ])
            }
          }
          else {
            let a: &RefCell<Context> = current_context.borrow();

            match a.borrow().local_variables_inference.get(&identifier.text) {
              Some(t) => Ok(inference::Type::Identifier(t.clone())),
              None => Err(vec![
                Report::build(ariadne::ReportKind::Error, (), span_manager.get_left(identifier.span))
                  .with_message(&"Unknown local variable")
                  .with_label(
                    Label::new(span_manager.get_range(identifier.span))
                    .with_message(&"No variable or property exists with such name")
                  )
                  .finish()
              ])
            }
          }
        },
        ExpressionBody::FunctionCall(function) => {
          let function_return_type = match inference_map.get(&function.accessor.text) {
              Some(infered_type) => match infered_type {
                crate::ast::codegen::type_inference::InferedType::Function(rc_function) => {
                  function.infered_function_type.replace(Some(rc_function.clone()));

                  match &(*rc_function).return_type {
                    Some(s) => Ok(inference::Type::Identifier(s.clone())),
                    None => Ok(inference::Type::Void)
                  }
                },
                _ => {
                  Err(vec![
                    Report::build(ariadne::ReportKind::Error, (), span_manager.get_left(function.accessor.span))
                      .with_message(&"Invalid function call")
                      .with_label(
                        Label::new(span_manager.get_range(function.accessor.span))
                        .with_message(&format!("{} is not a function.", &function.accessor.text))
                      )
                      .finish()
                  ])
                },
              },
              None => {
                Err(vec![
                  Report::build(ariadne::ReportKind::Warning, (), span_manager.get_left(function.accessor.span))
                    .with_message(&"Call to unknown function")
                    .with_label(
                      Label::new(span_manager.get_range(function.accessor.span))
                      .with_message(&format!("{} is not a known function.", &function.accessor.text))
                    )
                    .finish()
                ])
              },
          };

          function_return_type
        },
        ExpressionBody::ClassInstantiation(instantiation) => Ok(inference::Type::Identifier(instantiation.class_name.clone())),
        ExpressionBody::Lambda(_) => Ok(inference::Type::Unknown),
        ExpressionBody::Operation(left, operation, right) => {
          match &left.body.borrow() {
            // when it starts with a string, it can only be a string
            // concatenation
            ExpressionBody::String(_) => Ok(inference::Type::String),
            ExpressionBody::Float(_) => Ok(inference::Type::Float),
            ExpressionBody::Integer(_) => Ok(inference::Type::Int),
            _ => {
              match &operation {
                OperationCode::Nesting => {
                  let left_type = left.body.resulting_type(current_context, inference_map, span_manager)?;
                  let left_type_identifier = match left_type {
                    inference::Type::Identifier(s) => s,
                    _ => {
                      // report in case of invalid nesting, for example with numbers?
                      return Err(vec![]);
                    },
                  };

                  let infered_type = match inference_map.get(&left_type_identifier) {
                    Some(x) => x,
                    None => {
                      return Ok(inference::Type::Unknown);
                    }
                  };

                  match infered_type {
                    codegen::type_inference::InferedType::Compound(sub_inference_map) => {
                      let top_most_context = &Context::get_top_most_context(&current_context);
                      let top_most_context: &RefCell<Context> = &top_most_context.borrow();
                      let top_most_context: &Context = &top_most_context.borrow();

                      for file_context in &top_most_context.children_contexts {
                        let context = Context::get_ref(&file_context);
                        for global_type_context in &context.children_contexts {

                          let context = Context::get_ref(&global_type_context);
                          match context.get_class_name() {
                            Some(class_name) => {
                              if class_name == left_type_identifier {
                                return right.body.resulting_type(&global_type_context, sub_inference_map, span_manager);
                              }
                            },
                            None => {}
                          };
                        }
                      };

                      Ok(inference::Type::Unknown)
                    },
                    _ => Ok(inference::Type::Unknown)
                  }
                },
                _ => Ok(inference::Type::Unknown)
            }
            }
          }
        },
        ExpressionBody::Not(_) => Ok(inference::Type::Bool),
        ExpressionBody::Nesting(_) => unreachable!(),
        ExpressionBody::Cast(type_name, _) => Ok(inference::Type::Identifier(type_name.clone())),
        ExpressionBody::Group(expr) => expr.body.resulting_type(current_context, inference_map, span_manager),
        ExpressionBody::Error => Ok(inference::Type::Unknown),
    }
  }
}

impl ExpressionBody {
  pub fn get_type_for_this(
    current_context: &Rc<RefCell<Context>>,
    inference_map: &codegen::type_inference::TypeInferenceMap,
  ) -> Result<inference::Type, String> {
    let a: &RefCell<Context> = &current_context.borrow();
    let parent_context = &a.borrow().parent_context;

    match parent_context {
      Some(context) => {
        let a: &RefCell<Context> = &context.borrow();
        let context = &a.borrow();

        match &context.context_type {
          ContextType::ClassOrStruct | ContextType::State { parent_class_name: _ } => {
            let class_name = match context.get_class_name() {
              Some(n) => n,
              None => {
                return Err(String::from("Cannot use `this` outside of a class or a state"));
              }
            };

            if inference_map.contains_key(&class_name) {
              return Ok(inference::Type::Identifier(class_name));
            }
            else {
              return Err(format!("Cannot use `this` as {class_name} is not a known compound type"));
            }
          },
          _ => {
            return Err(String::from("Cannot use `this` outside of a class or a state"));
          }
        }
      },
      None => {
        return Err(String::from("Cannot use `this` outside of a class or a state"));
      }
    };
  }

  pub fn get_type_for_parent(
    current_context: &Rc<RefCell<Context>>,
    inference_map: &codegen::type_inference::TypeInferenceMap,
  ) -> Result<inference::Type, String> {

    let context = Context::get_ref(&current_context);
    let parent_context = match &context.parent_context {
      Some(p) => Context::get_ref(p),
      None => return Err(String::from("Cannot get `parent`'s type as it was used outside a state."))
    };

    match &parent_context.context_type {
        ContextType::State { parent_class_name } => {
          if inference_map.contains_key(parent_class_name) {
            return Ok(inference::Type::Identifier(parent_class_name.clone()));
          }
          else {
            return Err(format!("Cannot use `this` as {parent_class_name} is not a known compound type"));
          }
        },
        _ => {
          return Err(String::from("Cannot get `parent`'s type as it was used outside a state."));
        }
    };
  }

  pub fn get_span(&self) -> Span {
    match &self {
        ExpressionBody::Integer(x) => x.span,
        ExpressionBody::Float(x) => x.span,
        ExpressionBody::String(x) => x.span,
        ExpressionBody::Name(x) => x.span,
        ExpressionBody::Identifier(x) => x.span,
        ExpressionBody::FunctionCall(x) => x.span,
        ExpressionBody::ClassInstantiation(x) => x.span,
        ExpressionBody::Lambda(x) => x.span,
        ExpressionBody::Operation(x, _, _) => x.body.get_span(),
        ExpressionBody::Not(x) => x.body.get_span(),
        ExpressionBody::Nesting(x) => x[0].body.get_span(),
        ExpressionBody::Cast(_, x) => x.body.get_span(),
        ExpressionBody::Group(x) => x.body.get_span(),
        ExpressionBody::Error => todo!(),
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
