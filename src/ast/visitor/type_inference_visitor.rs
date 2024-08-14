use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use ariadne::{Label, Report, ReportKind};

use crate::ast::codegen::context::{Context, ContextType};
use crate::ast::codegen::type_inference::{InferedType, TypeInferenceStore};
use crate::ast::inference::Type;
use crate::ast::{
  Expression, FunctionDeclarationParameter, ReportManager, SpanManager, TypeDeclaration,
  TypedIdentifier
};

use super::lambda_declaration_visitor::ClosureVisitor;

/// 1.
/// Registers all the compound types from the program
pub struct CompoundTypesVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> CompoundTypesVisitor<'a> {
  pub fn new(
    current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
    report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager
  ) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for CompoundTypesVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    let result = self
      .inference_store
      .register_compound(node.name.clone(), node.extended_class_name.clone());

    if let Err(reason) = result {
      let span = node.span_name;

      self.report_manager.push(
        Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
          .with_message(&"Invalid class definition")
          .with_label(Label::new(self.span_manager.get_range(span)).with_message(reason))
          .finish(),
        span
      );
    }

    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();

    let parent_context = &Context::get_ref(&node.context).parent_context;
    let parent_context_type = if let Some(parent_context) = parent_context {
      Context::get_ref(&parent_context).context_type.clone()
    } else {
      ContextType::Global
    };

    let parameters =
      FunctionDeclarationParameter::to_function_infered_parameter_types(&node.parameters);

    // we try to see if the function is inside a struct or class or if
    // it is a global function.
    match parent_context_type {
      ContextType::ClassOrStruct
      | ContextType::State {
        parent_class_name: _
      } => {
        if let Some(parent_context) = parent_context {
          let parent_context = Context::get_ref(&parent_context);
          let compound_parent_name = parent_context.get_class_name()
            .expect("could not get the name of the parent compound type while analysing a method definition");

          let result = self.inference_store.register_method(
            compound_parent_name,
            node.name.clone(),
            parameters,
            match &node.type_declaration {
              Some(decl) => Some(decl.to_string()),
              None => None
            },
            node.span_name
          );

          if let Err(reason) = result {
            let span = node.span_name;

            self.report_manager.push(
              Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
                .with_message(&"Invalid method definition")
                .with_label(Label::new(self.span_manager.get_range(span)).with_message(reason))
                .finish(),
              span
            );
          }
        }
      }
      _ => {
        let result = self.inference_store.register_function(
          node.name.clone(),
          parameters,
          match &node.type_declaration {
            Some(decl) => Some(decl.to_string()),
            None => None
          },
          node.span_name
        );

        if let Err(reason) = result {
          let span = node.span_name;

          self.report_manager.push(
            Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
              .with_message(&"Invalid function definition")
              .with_label(Label::new(self.span_manager.get_range(span)).with_message(reason))
              .finish(),
            span
          );
        }
      }
    };
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    let result = self
      .inference_store
      .register_compound(node.name.clone(), None);

    if let Err(reason) = result {
      let span = node.span_name;

      self.report_manager.push(
        Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
          .with_message(&"Invalid struct definition")
          .with_label(Label::new(self.span_manager.get_range(span)).with_message(reason))
          .finish(),
        span
      );
    }

    self.current_context = node.context.clone();
  }
}

/// 2.
/// Visits every expression in the program and deduce their types
/// for other visitors.
pub struct ExpressionTypeInferenceVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> ExpressionTypeInferenceVisitor<'a> {
  pub fn new(
    current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
    report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager
  ) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for ExpressionTypeInferenceVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    self.current_context = node.context.clone();
  }

  fn visit_expression(&mut self, node: &Expression) {
    let result = node.deduce_type(
      &self.current_context,
      &self.inference_store.types,
      &self.inference_store.types,
      self.span_manager
    );

    if let InferedType::Lambda(_) = node.infered_type.borrow().as_ref() {
      self.inference_store.types.insert(
        node.infered_type_name.borrow().to_string(),
        node.infered_type.borrow().clone()
      );
    }

    if let Err(errors) = result {
      self.report_manager.push_many(errors);
    }
  }

  fn visit_variable_declaration(&mut self, node: &crate::ast::VariableDeclaration) {
    match &node {
      crate::ast::VariableDeclaration::Explicit {
        declaration,
        following_expression: _
      } => {
        for variable_name in &declaration.names {
          let type_declaration_string = declaration.type_declaration.to_string();

          self
            .current_context
            .borrow_mut()
            .local_variables_inference
            .insert(variable_name.clone(), type_declaration_string);
        }
      }
      crate::ast::VariableDeclaration::Implicit {
        names,
        following_expression
      } => {
        let expression: &Expression = &following_expression.borrow();

        let result = expression.deduce_type(
          &self.current_context,
          &self.inference_store.types,
          &self.inference_store.types,
          self.span_manager
        );

        if let Err(errors) = result {
          self.report_manager.push_many(errors);
        }

        let the_type: &Type = &expression.infered_type_name.borrow();

        match the_type {
          crate::ast::inference::Type::Void => {
            let span = following_expression.body.get_span();

            self.report_manager.push(
              Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
                .with_message(&"Cannot infer variable type")
                .with_label(
                  Label::new(self.span_manager.get_range(span))
                    .with_message(&"Implicit variable declaration but resulting type is void")
                )
                .finish(),
              span
            );

            return;
          }
          crate::ast::inference::Type::Unknown => {
            let span = following_expression.body.get_span();

            self.report_manager.push(
              Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
                .with_message(&"Cannot infer variable type")
                .with_label(Label::new(self.span_manager.get_range(span)).with_message(
                  &"Implicit variable declaration but resulting type is unknown at the time"
                ))
                .with_help(&"Prefer an explicit type annotation here")
                .finish(),
              span
            );

            return;
          }
          _ => {}
        };

        let the_type = the_type.to_string();

        for name in names {
          self
            .current_context
            .borrow_mut()
            .local_variables_inference
            .insert(name.clone(), the_type.clone());
        }

        self.register_variable_declaration(Rc::new(TypedIdentifier {
          names: names.clone(),
          type_declaration: TypeDeclaration::Regular {
            type_name: the_type,
            generic_type_assignment: None,
            mangled_accessor: RefCell::new(None)
          }
        }));
      }
    };
  }

  fn register_variable_declaration(&mut self, declaration: Rc<TypedIdentifier>) {
    self
      .current_context
      .borrow_mut()
      .variable_declarations
      .push(declaration);
  }

  fn visit_lambda(&mut self, node: &crate::ast::Lambda) {
    use super::Visited;

    let mut visitor = ClosureVisitor::new(
      self.current_context.clone(),
      self.inference_store,
      self.report_manager,
      self.span_manager
    );

    node.body_statements.accept(&mut visitor);

    // exclude the parameters from the captured variables
    let current_context = self.current_context.borrow_mut();
    let filtered_captured_variables: Vec<(String, Type)> = visitor
      .captured_variables
      .into_iter()
      .filter(|(variable_name, _)| {
        variable_name == "this"
          || (!node
            .parameters
            .iter()
            .any(|param| param.typed_identifier.names.contains(variable_name))
            && current_context
              .local_parameters_inference
              .contains_key(variable_name)
            || current_context
              .local_variables_inference
              .contains_key(variable_name))
      })
      .collect();

    node.captured_variables.replace(filtered_captured_variables);
  }
}

/// Does type inference for the local variables in the functions
pub struct FunctionsInferenceVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> FunctionsInferenceVisitor<'a> {
  pub fn new(
    current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
    report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager
  ) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for FunctionsInferenceVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    self.current_context = node.context.clone();
  }
}

/// Typechecks the function calls
pub struct FunctionsCallsCheckerVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> FunctionsCallsCheckerVisitor<'a> {
  pub fn new(
    current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
    report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager
  ) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for FunctionsCallsCheckerVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    self.current_context = node.context.clone();
  }

  fn visit_function_call(&mut self, node: &crate::ast::FunctionCall) {
    let some_infered_function_type = &*node.infered_function_type.borrow();

    if let Some(infered_function_type) = some_infered_function_type {
      let infered_function_type = &*infered_function_type;
      let parameter_pairs = infered_function_type
        .parameters
        .iter()
        .zip(node.parameters.0.iter());
      let mut count = 0;

      for (expected, some_supplied) in parameter_pairs {
        count += 1;

        // start by checking the optional parameters
        match expected.parameter_type {
          crate::ast::ParameterType::Optional => {
            // parameter is optional and none was passed, go to next parameter
            if some_supplied.is_none() {
              continue;
            }
          }
          _ => {
            // the parameter is not optional but None was passed
            if some_supplied.is_none() {
              self.report_manager.push(
                Report::build(
                  ariadne::ReportKind::Error,
                  (),
                  self.span_manager.get_left(node.accessor.span)
                )
                .with_message(&"Missing required parameter")
                .with_label(
                  Label::new(self.span_manager.get_range(node.accessor.span)).with_message(
                    &format!("Parameter n° {count} is required but is missing from function call")
                  )
                )
                .finish(),
                node.accessor.span
              );

              self.report_manager.push(
                Report::build(
                  ariadne::ReportKind::Advice,
                  (),
                  self.span_manager.get_left(expected.span)
                )
                .with_label(
                  Label::new(self.span_manager.get_range(expected.span))
                    .with_message("Try passing a parameter of the following type")
                )
                .finish(),
                expected.span
              );

              continue;
            }

            // now compare the types from the expected and the supplied
            // some types are also automatically casted, such as
            //  int -> float
            //  name -> string
            if let Some(supplied) = &some_supplied {
              let supplied_type = supplied.infered_type_name.borrow();
              if !supplied_type.equals_string(&expected.infered_type)
                && !supplied_type.can_auto_cast(&expected.infered_type)
              {
                let span = supplied.body.get_span();

                self.report_manager.push(
                  Report::build(
                    ariadne::ReportKind::Error,
                    (),
                    self.span_manager.get_left(span)
                  )
                  .with_message(&"Parameter type mismatch")
                  .with_label(
                    Label::new(self.span_manager.get_range(span)).with_message(&format!(
                      "Parameter n°{count} is expected to be a {} but a {} was passed",
                      &expected.infered_type,
                      supplied_type.to_string()
                    ))
                  )
                  .finish(),
                  span
                );

                self.report_manager.push(
                  Report::build(
                    ariadne::ReportKind::Advice,
                    (),
                    self.span_manager.get_left(expected.span)
                  )
                  .with_label(
                    Label::new(self.span_manager.get_range(expected.span))
                      .with_message("Try passing a parameter of the following type")
                  )
                  .finish(),
                  expected.span
                );

                continue;
              }
            }
          }
        };
      }
    }
  }
}
