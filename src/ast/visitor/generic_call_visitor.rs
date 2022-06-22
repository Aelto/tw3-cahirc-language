use crate::ast::ProgramInformation;


pub struct GenericCallsVisitor<'a> {
  pub program_information: &'a ProgramInformation,
}

impl super::Visitor for GenericCallsVisitor<'_> {
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    println!("visitor: {:?}", node.name);
}

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::GenericCallsVisitor
  }
}