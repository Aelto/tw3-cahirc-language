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

impl Codegen for StructDeclaration {
  fn emit(
    &self, context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    writeln!(f, "struct {} {{", self.name)?;

    for statement in &self.body_statements {
      statement.emit(context, f, data)?;
      writeln!(f, "")?;
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

impl Codegen for StructBodyStatement {
  fn emit(
    &self, context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      StructBodyStatement::Property(x) => {
        x.emit(context, f, data)?;
      }
      StructBodyStatement::DefaultValue(x) => {
        write!(f, "default ")?;
        x.emit(context, f, data)?;
        writeln!(f, ";")?;
      }
    };

    Ok(())
  }
}
