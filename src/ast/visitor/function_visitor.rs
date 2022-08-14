use std::cell::RefCell;
use std::rc::Rc;

use generic_call_visitor::GenericCallsVisitor;

use crate::ast::Context;
use crate::ast::visitor::generic_call_visitor;
use crate::ast::visitor::Visited;
use crate::ast::ProgramInformation;

pub struct FunctionVisitor<'a> {
  pub program_information: &'a ProgramInformation,
  pub current_context: Rc<RefCell<Context>>,
}

impl super::Visitor for FunctionVisitor<'_> {
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    let mut generic_call_visitor = GenericCallsVisitor::new(self.program_information);

    node.accept(&mut generic_call_visitor);

    self.current_context = node.context.clone();
  }

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::FunctionDeclarationVisitor
  }

  fn visit_function_declaration_parameter(&mut self, node: &crate::ast::FunctionDeclarationParameter) {
    let type_string_representation = node.typed_identifier.type_declaration.to_string();

    for parameter_name in &node.typed_identifier.names {
      self.current_context.as_ref().borrow_mut().local_parameters_inference.insert(
        parameter_name.clone(),
        type_string_representation.clone()
      );
    }
  }
}
