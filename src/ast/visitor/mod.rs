use super::*;

mod function_visitor;
pub use function_visitor::FunctionVisitor;

pub trait Visitor {
  fn visit_function_declaration(&mut self, node: &mut FunctionDeclaration);
  fn visitor_type(&self) -> VisitorType;
}

pub trait Visited {
  fn accept<T: Visitor>(&mut self, visitor: &mut T);
}

pub enum VisitorType {
  FunctionDeclarationVisitor
}