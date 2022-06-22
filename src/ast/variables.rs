use std::borrow::BorrowMut;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: Box<IdentifierTerm>,
  pub assignment_type: AssignmentType,
  pub following_expression: Rc<Expression>,
}

impl Visited for VariableAssignment {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.variable_name.accept(visitor);
    self.following_expression.accept(visitor);
  }
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignment {
  Declaration(VariableDeclaration),
  Assignement(VariableAssignment),
}

impl Visited for VariableDeclarationOrAssignment {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      VariableDeclarationOrAssignment::Declaration(x) => x.accept(visitor),
      VariableDeclarationOrAssignment::Assignement(x) => x.accept(visitor),
    }
  }
}

#[derive(Debug)]
pub struct VariableDeclaration {
  pub declaration: TypedIdentifier,
  pub following_expression: Option<Rc<Expression>>,
}

impl visitor::Visited for VariableDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.following_expression.accept(visitor);
  }
}
