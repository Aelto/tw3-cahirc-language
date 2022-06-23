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

impl Display for StructDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "struct {} {{", self.name)?;

    for statement in &self.body_statements {
      write!(f, "{statement}")?;
    }

    writeln!(f, "}}")?;

    Ok(())
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

impl Display for StructBodyStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StructBodyStatement::Property(x) => write!(f, "{x};"),
      StructBodyStatement::DefaultValue(x) => write!(f, "default {x};"),
    }
  }
}
