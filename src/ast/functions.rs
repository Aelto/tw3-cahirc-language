use std::rc::Rc;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
/// property.
pub struct FunctionDeclaration {
  pub function_type: FunctionType,
  pub name: String,
  pub generic_types: Option<Vec<String>>,
  pub parameters: Vec<TypedIdentifier>,
  pub type_declaration: Option<TypeDeclaration>,
  pub body_statements: Vec<FunctionBodyStatement>,
  pub is_latent: bool,
}

impl visitor::Visited for FunctionDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_function_declaration(&self);

    self.body_statements.accept(visitor);
  }
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event,
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Rc<Expression>),
  Return(Rc<Expression>),
  Assignement(VariableAssignment),
  IfStatement(IfStatement),
  ForStatement(ForStatement),
  WhileStatement(WhileStatement),
  DoWhileStatement(DoWhileStatement),
}

impl visitor::Visited for FunctionBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match &self {
      FunctionBodyStatement::VariableDeclaration(x) => x.accept(visitor),
      FunctionBodyStatement::Expression(x) => x.accept(visitor),
      FunctionBodyStatement::Return(x) => x.accept(visitor),
      FunctionBodyStatement::Assignement(x) => x.accept(visitor),
      FunctionBodyStatement::IfStatement(x) => x.accept(visitor),
      FunctionBodyStatement::ForStatement(x) => x.accept(visitor),
      FunctionBodyStatement::WhileStatement(x) => x.accept(visitor),
      FunctionBodyStatement::DoWhileStatement(x) => x.accept(visitor),
    };
  }
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Rc<Expression>>);

impl Visited for FunctionCallParameters {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    &self.0.accept(visitor);
  }
}
