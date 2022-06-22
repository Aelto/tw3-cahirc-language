use crate::ast::ProgramInformation;


pub struct FunctionVisitor<'a> {
  pub program_information: &'a ProgramInformation,
}

impl super::Visitor for FunctionVisitor<'_> {
  fn visit_function_declaration(&mut self, node: &mut crate::ast::FunctionDeclaration) {
    println!("FunctionVisitor: {:?}", node.name);
  }

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::FunctionDeclarationVisitor
  }
}