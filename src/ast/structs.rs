use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct StructDeclaration {
  pub name: String,
  pub body_statements: Vec<StructBodyStatement>,
}

impl Visited for StructDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}

#[derive(Debug)]
pub enum StructBodyStatement {
  Property(VariableDeclaration),
  DefaultValue(VariableAssignment),
}

impl Visited for StructBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      StructBodyStatement::Property(x) => x.accept(visitor),
      StructBodyStatement::DefaultValue(x) => x.accept(visitor),
    }
  }
}
