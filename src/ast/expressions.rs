use std::borrow::Borrow;
use std::rc::Rc;

use ariadne::{Label, Report};

use super::codegen::type_inference::{FunctionInferedType, InferedType, TypeInferenceMap};
use super::inference::Type;
use super::*;

#[derive(Debug)]
pub struct Expression {
  ///
  pub infered_type: RefCell<Rc<InferedType>>,
  pub infered_type_name: RefCell<Type>,

  pub body: ExpressionBody
}

impl Expression {
  pub fn new(body: ExpressionBody) -> Self {
    Self {
      infered_type: RefCell::new(Rc::new(InferedType::Unknown)),
      infered_type_name: RefCell::new(Type::Unknown),
      body
    }
  }

  pub fn set_infered_type(&self, name: Type, t: Rc<InferedType>) {
    // println!("set infered_type type={:?}, infered_type={:?}", name, t);

    self.infered_type_name.replace(name);
    self.infered_type.replace(t);
  }

  pub fn deduce_type(
    &self, current_context: &Rc<RefCell<Context>>, inference_map: &TypeInferenceMap,
    global_inference_map: &TypeInferenceMap, span_manager: &SpanManager
  ) -> Result<(), Vec<(Report, Span)>> {
    {
      let self_infered_type_name: &Type = &self.infered_type_name.borrow();
      if let Type::Unknown = self_infered_type_name {
      } else {
        // Since sometimes the deduce_type function may be called early
        // due to expressions nesting or operations, there is no need
        // to deduce the type again if it's already been deduced.
        return Ok(());
      };
    }

    match &self.body {
      ExpressionBody::Integer(_) => {
        if let Some(infered_type) = inference_map.get("int") {
          self.set_infered_type(Type::Int, infered_type.clone());
        };
      }
      ExpressionBody::Float(_) => {
        if let Some(infered_type) = inference_map.get("float") {
          self.set_infered_type(Type::Float, infered_type.clone());
        }
      }
      ExpressionBody::String(_) => {
        if let Some(infered_type) = inference_map.get("string") {
          self.set_infered_type(Type::String, infered_type.clone());
        }
      }
      ExpressionBody::Name(_) => {
        if let Some(infered_type) = inference_map.get("name") {
          self.set_infered_type(Type::Name, infered_type.clone());
        }
      }
      ExpressionBody::Identifier(identifier) => {
        let a: &IdentifierTerm = &identifier.borrow();

        if a.text == "this" {
          let result = ExpressionBody::get_type_for_this(current_context, inference_map);
          match result {
            Ok(t) => {
              if let Some(infered_type) = inference_map.get(&t.to_string()) {
                self.set_infered_type(t, infered_type.clone());
              }
            }
            Err(message) => {
              return Err(vec![(
                Report::build(
                  ariadne::ReportKind::Error,
                  (),
                  span_manager.get_left(identifier.span)
                )
                .with_message(&"Could not infer type for `this`")
                .with_label(
                  Label::new(span_manager.get_range(identifier.span)).with_message(&message)
                )
                .finish(),
                identifier.span
              )]);
            }
          }
        } else if a.text == "parent" {
          let result = ExpressionBody::get_type_for_parent(current_context, inference_map);
          match result {
            Ok(t) => {
              if let Some(infered_type) = inference_map.get(&t.to_string()) {
                self.set_infered_type(t, infered_type.clone());
              }
            }
            Err(message) => {
              return Err(vec![(
                Report::build(
                  ariadne::ReportKind::Error,
                  (),
                  span_manager.get_left(identifier.span)
                )
                .with_message(&"Could not infer type for `parent`")
                .with_label(
                  Label::new(span_manager.get_range(identifier.span)).with_message(&message)
                )
                .finish(),
                identifier.span
              )]);
            }
          }
        } else {
          let a: &RefCell<Context> = current_context.borrow();

          match a.borrow().get_variable_type_string(&identifier.text) {
            Some(t) => {
              if let Some(infered_type) = inference_map.get(t) {
                self.set_infered_type(Type::Identifier(t.clone()), infered_type.clone());
              }
            }
            None => {
              return Err(vec![(
                Report::build(
                  ariadne::ReportKind::Error,
                  (),
                  span_manager.get_left(identifier.span)
                )
                .with_message(&"Unknown local variable")
                .with_label(
                  Label::new(span_manager.get_range(identifier.span))
                    .with_message(&"No variable or property exists with such name")
                )
                .finish(),
                identifier.span
              )]);
            }
          }
        }
      }
      ExpressionBody::FunctionCall(function) => {
        match inference_map.get(&function.accessor.text) {
          Some(infered_type) => match infered_type.as_ref() {
            crate::ast::codegen::type_inference::InferedType::Function(rc_function) => {
              function
                .infered_function_type
                .replace(Some(rc_function.clone()));

              match &(*rc_function).return_type {
                Some(s) => {
                  if let Some(infered_type) = global_inference_map.get(s) {
                    self.set_infered_type(Type::Identifier(s.clone()), infered_type.clone());
                  } else {
                    // todo: handle unknown return type
                  }
                }
                None => {
                  self.set_infered_type(Type::Void, Rc::new(InferedType::Unknown));
                }
              }
            }
            _ => {
              return Err(vec![(
                Report::build(
                  ariadne::ReportKind::Error,
                  (),
                  span_manager.get_left(function.accessor.span)
                )
                .with_message(&"Invalid function call")
                .with_label(
                  Label::new(span_manager.get_range(function.accessor.span))
                    .with_message(&format!("{} is not a function.", &function.accessor.text))
                )
                .finish(),
                function.accessor.span
              )]);
            }
          },
          None => {
            return Err(vec![(
              Report::build(
                ariadne::ReportKind::Warning,
                (),
                span_manager.get_left(function.accessor.span)
              )
              .with_message(&"Call to unknown function")
              .with_label(
                Label::new(span_manager.get_range(function.accessor.span)).with_message(&format!(
                  "{} is not a known function.",
                  &function.accessor.text
                ))
              )
              .finish(),
              function.accessor.span
            )]);
          }
        };
      }
      ExpressionBody::ClassInstantiation(instantiation) => {
        if let Some(infered_type) = inference_map.get(&instantiation.class_name) {
          self.set_infered_type(
            Type::Identifier(instantiation.class_name.clone()),
            infered_type.clone()
          );
        }
      }
      ExpressionBody::Lambda(lambda) => {
        let return_type =
          FunctionBodyStatement::get_return_type_from_last_statement(&lambda.body_statements);
        let the_type: Type = Type::Identifier(LambdaDeclaration::stringified_type_representation(
          &lambda.parameters,
          &return_type
        ));
        let infered_type: Rc<InferedType> =
          Rc::new(InferedType::Lambda(Rc::new(FunctionInferedType {
            parameters: FunctionDeclarationParameter::to_function_infered_parameter_types(
              &lambda.parameters
            ),
            return_type: return_type.and_then(|t| Some(t.clone())),
            span: lambda.span
          })));

        self.set_infered_type(the_type, infered_type);
      }
      ExpressionBody::Operation(left, operation, right) => {
        match &left.body.borrow() {
          // when it starts with a string, it can only be a string
          // concatenation
          ExpressionBody::String(_) => {
            if let Some(infered_type) = inference_map.get("string") {
              self.set_infered_type(Type::String, infered_type.clone());
            }
          }
          ExpressionBody::Float(_) => {
            if let Some(infered_type) = inference_map.get("float") {
              self.set_infered_type(Type::Float, infered_type.clone());
            }
          }
          ExpressionBody::Integer(_) => {
            if let Some(infered_type) = inference_map.get("int") {
              self.set_infered_type(Type::Int, infered_type.clone());
            }
          }
          _ => {
            match &operation {
              OperationCode::Nesting => {
                left.deduce_type(current_context, inference_map, inference_map, span_manager)?;

                // special check for lambda calls:
                {
                  let left_infered_type = left.as_ref().infered_type.borrow();
                  let left_type = left_infered_type.as_ref();
                  let right_type = &right.as_ref().body;

                  match (left_type, &right_type) {
                    (InferedType::Lambda(lambda), ExpressionBody::FunctionCall(_)) => {
                      match lambda.return_type.as_ref() {
                        Some(t) => {
                          let lambda_return_type = global_inference_map.get(t);

                          if let Some(return_type) = lambda_return_type {
                            self.set_infered_type(Type::Identifier(t.clone()), return_type.clone());

                            // set a blank type on the right so it is not scanned
                            right.set_infered_type(
                              self.infered_type_name.borrow().clone(),
                              self.infered_type.borrow().clone()
                            );

                            return Ok(());
                          } else {
                            let span = lambda.span;

                            return Err(vec![(
                              Report::build(
                                ariadne::ReportKind::Warning,
                                (),
                                span_manager.get_left(span)
                              )
                              .with_message(&"Unknown return type in lambda")
                              .with_label(Label::new(span_manager.get_range(span)).with_message(
                                &format!("The returned type \"{t}\" is not a known type")
                              ))
                              .finish(),
                              span
                            )]);
                          }
                        }
                        None => {}
                      };

                      // leave no matter what, we don't want warnings/errors from nested calls
                      // from lambdas
                      return Ok(());
                    }
                    _ => {}
                  };
                }

                let left_infered_type = left.infered_type.borrow();
                let (left_compound_type_inference_map, left_extended_type) =
                  match &**left_infered_type {
                    InferedType::Compound {
                      type_inference_map,
                      extends
                    } => (type_inference_map, extends),
                    _ => {
                      let span = left.body.get_span();

                      return Err(vec![(
                        Report::build(
                          ariadne::ReportKind::Warning,
                          (),
                          span_manager.get_left(span)
                        )
                        .with_message(&"Invalid nesting")
                        .with_label(Label::new(span_manager.get_range(span)).with_message(
                          &"Nesting but left side expression does not result in a compound type."
                        ))
                        .finish(),
                        span
                      )]);
                    }
                  };

                let left_type_name: &Type = &left.infered_type_name.borrow();
                let left_type_identifier = match left_type_name {
                  Type::Identifier(s) => s,
                  _ => {
                    let span = left.body.get_span();

                    return Err(vec![(
                      Report::build(
                        ariadne::ReportKind::Warning,
                        (),
                        span_manager.get_left(span)
                      )
                      .with_message(&"Invalid nesting")
                      .with_label(
                        Label::new(span_manager.get_range(span))
                          .with_message(&"Nesting but left side expression is not an identifier.")
                      )
                      .finish(),
                      span
                    )]);
                  }
                };

                // now that we know the left side of the nesting is a compound type,
                let top_most_context = &Context::get_top_most_context(&current_context);
                let top_most_context: &RefCell<Context> = &top_most_context.borrow();
                let top_most_context: &Context = &top_most_context.borrow();

                for file_context in &top_most_context.children_contexts {
                  let context = Context::get_ref(&file_context);
                  for global_type_context in &context.children_contexts {
                    let context = Context::get_ref(&global_type_context);
                    match context.get_compound_name() {
                      Some(compound_name) => {
                        if &compound_name == left_type_identifier {
                          // we need to handle class inheritance for nesting
                          // on compound types.
                          let mut used_type_inference_map = left_compound_type_inference_map;
                          let mut used_extend = left_extended_type;

                          loop {
                            let result = right.deduce_type(
                              &global_type_context,
                              &used_type_inference_map.borrow(),
                              inference_map,
                              span_manager
                            );

                            if result.is_ok() {
                              break;
                            }

                            match used_extend {
                              // there is still a type in the inheritance tree
                              Some(extend_type_identifier) => {
                                dbg!(&extend_type_identifier);
                                match global_inference_map.get(extend_type_identifier) {
                                  Some(base_type) => {
                                    match base_type.borrow() {
                                      InferedType::Compound {
                                        type_inference_map,
                                        extends
                                      } => {
                                        used_type_inference_map = type_inference_map;
                                        used_extend = extends;
                                      }
                                      _ => {
                                        return result;
                                      }
                                    };
                                  }
                                  None => return result
                                };
                              }
                              None => return result
                            }
                          }

                          self.set_infered_type(
                            right.infered_type_name.borrow().clone(),
                            right.infered_type.borrow().clone()
                          );
                        }
                      }
                      None => {}
                    };
                  }
                }
              }
              _ => {
                // todo: other operation nestings such as +, /, -, *
              }
            };
          }
        }
      }
      ExpressionBody::Not(_) => {
        if let Some(infered_type) = inference_map.get("bool") {
          self.set_infered_type(Type::Bool, infered_type.clone());
        }
      }
      ExpressionBody::List(x) => {
        if let Some(first) = x.first() {
          first.deduce_type(
            current_context,
            inference_map,
            global_inference_map,
            span_manager
          )?;
        }
      }
      ExpressionBody::Nesting(_) => unreachable!(),
      ExpressionBody::Cast(type_name, expr) => {
        match inference_map.get(type_name) {
          Some(infered_type) => {
            self.set_infered_type(Type::Identifier(type_name.clone()), infered_type.clone());
          }
          None => {
            let span = expr.body.get_span();

            return Err(vec![(
              Report::build(
                ariadne::ReportKind::Warning,
                (),
                span_manager.get_left(span)
              )
              .with_message(&"Cast to unknown type")
              .with_label(
                Label::new(span_manager.get_range(span))
                  .with_message(&format!("{} is not a known type.", &type_name))
              )
              .finish(),
              span
            )]);
          }
        };
      }
      ExpressionBody::Group(expr) => {
        // deduce early the type of the expression:
        expr.deduce_type(current_context, inference_map, inference_map, span_manager)?;

        self.set_infered_type(
          expr.infered_type_name.borrow().clone(),
          expr.infered_type.borrow().clone()
        );
      }
      ExpressionBody::Error => {}
    };

    Ok(())
  }
}

impl visitor::Visited for Expression {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_expression(self);

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
  List(Vec<Rc<Expression>>),

  /// Expressions surrounded by parenthesis
  Group(Rc<Expression>),

  Error
}

impl visitor::Visited for ExpressionBody {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      ExpressionBody::Integer(_)
      | ExpressionBody::Float(_)
      | ExpressionBody::String(_)
      | ExpressionBody::Name(_) => {}
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
      ExpressionBody::List(x) => x.accept(visitor),
      ExpressionBody::Lambda(x) => x.accept(visitor)
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
      ExpressionBody::List(x) => {
        writeln!(f, "{{")?;

        let mut items = x.iter().peekable();

        while let Some(item) = items.next() {
          item.emit(context, f)?;

          if items.peek().is_some() {
            writeln!(f, ",")?;
          } else {
            writeln!(f, "")?;
          }
        }

        write!(f, "}}")
      }
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
      ExpressionBody::Lambda(x) => x.emit(context, f)
    }
  }
}

impl ExpressionBody {
  pub fn get_type_for_this(
    current_context: &Rc<RefCell<Context>>,
    inference_map: &codegen::type_inference::TypeInferenceMap
  ) -> Result<inference::Type, String> {
    let a: &RefCell<Context> = &current_context.borrow();
    let parent_context = &a.borrow().parent_context;

    match parent_context {
      Some(context) => {
        let a: &RefCell<Context> = &context.borrow();
        let context = &a.borrow();

        match &context.context_type {
          ContextType::ClassOrStruct
          | ContextType::State {
            parent_class_name: _
          } => {
            let class_name = match context.get_class_name() {
              Some(n) => n,
              None => {
                return Err(String::from(
                  "Cannot use `this` outside of a class or a state"
                ));
              }
            };

            if inference_map.contains_key(&class_name) {
              return Ok(inference::Type::Identifier(class_name));
            } else {
              return Err(format!(
                "Cannot use `this` as {class_name} is not a known compound type"
              ));
            }
          }
          _ => {
            return Err(String::from(
              "Cannot use `this` outside of a class or a state"
            ));
          }
        }
      }
      None => {
        return Err(String::from(
          "Cannot use `this` outside of a class or a state"
        ));
      }
    };
  }

  pub fn get_type_for_parent(
    current_context: &Rc<RefCell<Context>>,
    inference_map: &codegen::type_inference::TypeInferenceMap
  ) -> Result<inference::Type, String> {
    let context = Context::get_ref(&current_context);
    let parent_context = match &context.parent_context {
      Some(p) => Context::get_ref(p),
      None => {
        return Err(String::from(
          "Cannot get `parent`'s type as it was used outside a state."
        ))
      }
    };

    match &parent_context.context_type {
      ContextType::State { parent_class_name } => {
        if inference_map.contains_key(parent_class_name) {
          return Ok(inference::Type::Identifier(parent_class_name.clone()));
        } else {
          return Err(format!(
            "Cannot use `this` as {parent_class_name} is not a known compound type"
          ));
        }
      }
      _ => {
        return Err(String::from(
          "Cannot get `parent`'s type as it was used outside a state."
        ));
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
      ExpressionBody::Operation(_, _, x) => x.body.get_span(),
      ExpressionBody::Not(x) => x.body.get_span(),
      ExpressionBody::Nesting(x) => x.last().unwrap().body.get_span(),
      ExpressionBody::List(x) => x.last().unwrap().body.get_span(),
      ExpressionBody::Cast(_, x) => x.body.get_span(),
      ExpressionBody::Group(x) => x.body.get_span(),
      ExpressionBody::Error => todo!()
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
  Comparison(ComparisonType)
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
      OperationCode::BitwiseAnd => write!(f, "&")
    }
  }
}

#[derive(Debug)]
pub enum AssignmentType {
  Equal,
  PlusEqual,
  MinusEqual,
  AsteriskEqual,
  SlashEqual
}

impl Codegen for AssignmentType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      AssignmentType::Equal => write!(f, "="),
      AssignmentType::PlusEqual => write!(f, "+="),
      AssignmentType::MinusEqual => write!(f, "-="),
      AssignmentType::AsteriskEqual => write!(f, "*="),
      AssignmentType::SlashEqual => write!(f, "/=")
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
  Different
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
      ComparisonType::Different => write!(f, "!=")
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum BooleanJoinType {
  And,
  Or
}

impl Codegen for BooleanJoinType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      BooleanJoinType::And => write!(f, " && "),
      BooleanJoinType::Or => write!(f, " || ")
    }
  }
}
