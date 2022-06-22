use super::*;

mod function_visitor;
pub use function_visitor::FunctionVisitor;

mod generic_call_visitor;
pub use generic_call_visitor::GenericCallsVisitor;

pub mod implementations;

pub trait Visitor {
  fn visit_function_declaration(&mut self, node: &FunctionDeclaration) {}
  fn visit_generic_function_call(&mut self, node: &FunctionCall) {}
  fn visitor_type(&self) -> VisitorType;
}

pub trait Visited {
  fn accept<T: Visitor>(&self, visitor: &mut T);
}

pub enum VisitorType {
  FunctionDeclarationVisitor,
  GenericCallsVisitor,
}
