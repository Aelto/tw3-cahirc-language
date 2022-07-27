use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct EnumDeclaration {
  pub name: String,
  pub body_statements: Vec<EnumBodyStatement>,
}

impl Visited for EnumDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}

impl Codegen for EnumDeclaration {
  fn emit(
    &self, context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    writeln!(f, "enum {} {{", self.name)?;

    for statement in &self.body_statements {
      statement.emit(context, f, data)?;
      writeln!(f, ",")?;
    }

    writeln!(f, "}}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub struct EnumBodyStatement {
  pub name: String,
  pub number: Option<String>,
}

impl Visited for EnumBodyStatement {
  fn accept<T: visitor::Visitor>(&self, _visitor: &mut T) {}
}

impl Codegen for EnumBodyStatement {
  fn emit(
    &self, _context: &Context, f: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "{}", self.name)?;

    if let Some(number) = &self.number {
      write!(f, " = {}", number)?;
    }

    Ok(())
  }
}
