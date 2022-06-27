use generic_call_visitor::GenericCallsVisitor;

use crate::ast::visitor::generic_call_visitor;
use crate::ast::visitor::Visited;
use crate::ast::ProgramInformation;

pub struct FunctionVisitor<'a> {
  pub program_information: &'a ProgramInformation,
}

impl super::Visitor for FunctionVisitor<'_> {
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    // println!("FunctionVisitor: {:?}", node.name);

    let mut generic_call_visitor = GenericCallsVisitor::new(self.program_information);

    node.accept(&mut generic_call_visitor);
  }

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::FunctionDeclarationVisitor
  }
}
