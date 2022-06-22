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
