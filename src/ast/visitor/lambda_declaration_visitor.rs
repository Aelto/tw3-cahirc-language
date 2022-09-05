use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::codegen::context::ContextType;
use crate::ast::codegen::type_inference::TypeInferenceStore;
use crate::ast::inference::Type;
use crate::ast::ExpressionBody;
use crate::ast::ReportManager;
use crate::ast::SpanManager;

/// Looks for generic calls and register them to the GenericCallRegister
pub struct LambdaDeclarationVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub emitted_code: &'a mut Vec<u8>,
  pub emitted_types: HashSet<String>,
}

impl<'a> LambdaDeclarationVisitor<'a> {
  pub fn new(emitted_code: &'a mut Vec<u8>) -> Self {
    Self {
      current_context: Rc::new(RefCell::new(Context::new(
        "empty",
        None,
        ContextType::Global,
      ))),
      emitted_code,
      emitted_types: HashSet::new(),
    }
  }
}

impl super::Visitor for LambdaDeclarationVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::LambdaDeclarationVisitor
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

  fn visit_lambda_declaration(&mut self, node: &crate::ast::LambdaDeclaration) {
    if let Err(err) = node.emit_base_type(
      &mut self.current_context.borrow_mut(),
      &mut self.emitted_code,
      &mut self.emitted_types,
    ) {
      println!(
        "Error while emitting code for {}: {}",
        self.current_context.borrow().name,
        err
      );
    }
  }

  fn visit_lambda(&mut self, node: &crate::ast::Lambda) {
    if let Err(err) = node.emit_base_type(
      &mut self.current_context.borrow_mut(),
      &mut self.emitted_code,
    ) {
      println!(
        "Error while emitting code for {}: {}",
        self.current_context.borrow().name,
        err
      );
    }
  }
}

/// Visits the expressions of a lambda to find the capture parameters.
pub struct ClosureVisitor<'a> {
  pub captured_variables: Vec<(String, Type)>,

  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager,
}

impl<'a> ClosureVisitor<'a> {
  pub fn new(
    current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
    report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager,
  ) -> Self {
    Self {
      captured_variables: Vec::new(),
      current_context,
      inference_store,
      report_manager,
      span_manager,
    }
  }
}

impl<'a> super::Visitor for ClosureVisitor<'a> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::ClosureExpressionVisitor
  }

  fn visit_expression(&mut self, node: &crate::ast::Expression) {
    if let ExpressionBody::Identifier(identifier) = &node.body {
      let result = node.deduce_type(
        &self.current_context,
        &self.inference_store.types,
        &self.inference_store.types,
        self.span_manager,
      );

      if let Err(errors) = result {
        self.report_manager.push_many(errors);
      }

      self.captured_variables.push((
        identifier.text.clone(),
        node.infered_type_name.borrow().clone(),
      ));
    }
  }
}
