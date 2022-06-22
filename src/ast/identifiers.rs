use std::rc::Rc;

use super::*;

#[derive(Debug)]
pub struct IdentifierTerm {
  pub text: String,
  pub indexing: Vec<Rc<Expression>>,
  pub nesting: Option<Box<IdentifierTerm>>
}

impl IdentifierTerm {
  pub fn get_last_text(self) -> String {
    self.nesting
        .and_then(|terms| Some(terms.get_last_text()))
        .unwrap_or_else(|| self.text)
  }
}

#[derive(Debug)]
pub struct TypedIdentifier {
  pub name: String,
  pub type_declaration: TypeDeclaration
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
  pub generic_type_assignment: Option<Vec<TypeDeclaration>>
}
