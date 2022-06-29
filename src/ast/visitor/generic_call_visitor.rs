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
      current_context: Rc::new(RefCell::new(Context::new("empty", None))),
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
    let function_name = node.accessor.get_last_text();
    let function_context = Context::find_global_function_declaration(&self.current_context, &function_name);

    if let Some(generic_types) = &node.generic_types {
      if let Some(function_context) = function_context {
        function_context.borrow_mut().register_generic_call(&generic_types);
      }
    }
  }

  fn visit_generic_variable_declaration(&mut self, node: &crate::ast::TypeDeclaration) {
    let class_name = &node.type_name;
    let class_context = Context::find_global_class_declaration(&self.current_context, class_name);

    if let Some(generic_types) = &node.generic_type_assignment {
      if let Some(class_context) = class_context {
        class_context.borrow_mut().register_generic_call(&node.stringified_generic_types());
      }
    }
  }
}
