use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::ProgramInformation;

/// Looks for generic calls and register them to the GenericCallRegister
pub struct GenericCallsVisitor<'a> {
  pub program_information: &'a ProgramInformation,
  pub current_context: Rc<RefCell<Context>>,
}

impl<'a> GenericCallsVisitor<'a> {
  pub fn new(program_information: &'a ProgramInformation) -> Self {
    Self {
      program_information,
      current_context: Rc::new(RefCell::new(Context::new("empty"))),
    }
  }
}

impl super::Visitor for GenericCallsVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::GenericCallsVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  fn visit_generic_function_call(&mut self, node: &crate::ast::FunctionCall) {
    if let Some(generic_types) = &node.generic_types {
      self
        .program_information
        .generic_functions_register
        .borrow_mut()
        .register_call(node.get_function_name(), generic_types.clone());
    }
  }
}
