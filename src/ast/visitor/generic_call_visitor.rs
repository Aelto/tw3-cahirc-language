use crate::ast::ProgramInformation;

/// Looks for generic calls and register them to the GenericCallRegister
pub struct GenericCallsVisitor<'a> {
  pub program_information: &'a ProgramInformation,
}

impl super::Visitor for GenericCallsVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::GenericCallsVisitor
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
