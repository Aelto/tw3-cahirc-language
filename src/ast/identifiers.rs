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

impl Display for IdentifierTerm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)?;

    for indexing in &self.indexing {
      write!(f, "[{}]", indexing)?;
    }

    if let Some(nesting) = &self.nesting {
      write!(f, ".{}", nesting)?;
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

impl Display for TypedIdentifier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.type_declaration)
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

impl Display for TypeDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.type_name)?;

    if let Some(comma_separated_types) = &self.generic_type_assignment {
      write!(f, "<")?;

      for t in comma_separated_types {
        write!(f, "{t}")?;
      }

      write!(f, ">")?;
    }

    Ok(())
  }
}
