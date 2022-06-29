use std::rc::Rc;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct IdentifierTerm {
  pub text: String,
  pub indexing: Vec<Rc<Expression>>,
  pub nesting: Option<Box<IdentifierTerm>>,
}

impl Visited for IdentifierTerm {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.indexing.accept(visitor);
    self.nesting.accept(visitor);
  }
}

impl Codegen for IdentifierTerm {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "{}", self.text)?;

    for indexing in &self.indexing {
      write!(f, "[")?;
      indexing.emit(context, f)?;
      write!(f, "]")?;
    }

    if let Some(nesting) = &self.nesting {
      write!(f, ".")?;
      nesting.emit(context, f)?;
    }

    Ok(())
  }
}

impl IdentifierTerm {
  pub fn get_last_text(&self) -> String {
    if let Some(nesting) = &self.nesting {
      nesting.get_last_text()
    } else {
      self.text.clone()
    }
  }
}

#[derive(Debug)]
pub struct TypedIdentifier {
  pub name: String,
  pub type_declaration: TypeDeclaration,
}

impl Codegen for TypedIdentifier {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "{}: ", self.name, )?;
    self.type_declaration.emit(context, f)
  }
}

/// Represents a type declaration that could be after anything, for example
/// ```
/// a: int
/// ```
///
/// `: int` is the typeDeclaration
#[derive(Debug)]
pub struct TypeDeclaration {
  pub type_name: String,
  pub generic_type_assignment: Option<Vec<TypeDeclaration>>,
}

impl Codegen for TypeDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    // TODO: find a way to access the context and use Context::transform_if_generic_type
    context.transform_if_generic_type(f, &self.type_name)?;
    // write!(f, "{}", self.type_name)?;

    if let Some(comma_separated_types) = &self.generic_type_assignment {
      write!(f, "<")?;

      for t in comma_separated_types {
        t.emit(context, f)?;
      }

      write!(f, ">")?;
    }

    Ok(())
  }
}
