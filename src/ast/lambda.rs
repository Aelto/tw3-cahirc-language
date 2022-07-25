use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct Lambda {
  pub name: String,
  pub body_statements: Vec<StructBodyStatement>,
}

impl Visited for Lambda {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}

impl Codegen for Lambda {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    writeln!(f, "struct {} {{", self.name)?;

    for statement in &self.body_statements {
      statement.emit(context, f)?;
      writeln!(f, "")?;
    }

    writeln!(f, "}}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub enum StructBodyStatement {
  #[allow(dead_code)]
  Property(VariableDeclaration),
  #[allow(dead_code)]
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
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      StructBodyStatement::Property(x) => {
        x.emit(context, f)?;
        write!(f, ";")?;
      }
      StructBodyStatement::DefaultValue(x) => {
        write!(f, "default ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
    };

    Ok(())
  }
}
