use std::rc::Rc;

use crate::ast::codegen::context::GenericContext;

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

    write!(f, "{}: ", self.name,)?;
    self.type_declaration.emit(context, f)
  }
}

impl Visited for TypedIdentifier {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.type_declaration.accept(visitor);
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

impl Visited for TypeDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    if let Some(generic_types) = &self.generic_type_assignment {
      visitor.visit_generic_variable_declaration(self);

      for t in generic_types {
        t.accept(visitor);
      }
    }
  }
}

impl Codegen for TypeDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    context.transform_if_generic_type(f, &self.type_name)?;

    if let Some(comma_separated_types) = &self.generic_type_assignment {
      let generic_variant_suffix =
        GenericContext::generic_variant_suffix_from_types(&self.stringified_generic_types());

      write!(f, "{generic_variant_suffix}")?;

      // special case: array is the only generic type support by vanilla WS
      if self.type_name == "array" {
        write!(f, "<")?;

        let mut types = comma_separated_types.iter().peekable();
        while let Some(t) = types.next() {
          t.emit(context, f)?;

          if types.peek().is_some() {
            write!(f, ", ")?;
          }
        }

        write!(f, ">")?;
      }
    }

    Ok(())
  }
}

impl TypeDeclaration {
  pub fn to_string(&self) -> String {
    if let Some(generics) = &self.generic_type_assignment {
      let mut output = self.type_name.clone();

      for generic in generics {
        output.push_str(&generic.to_string());
      }

      output
    } else {
      self.type_name.clone()
    }
  }

  pub fn stringified_generic_types(&self) -> Vec<String> {
    match &self.generic_type_assignment {
      None => Vec::new(),
      Some(generic_types) => generic_types
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>(),
    }
  }
}
