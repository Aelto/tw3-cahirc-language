use std::rc::Rc;

use crate::ast::codegen::context::GenericContext;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct IdentifierTerm {
  pub text: String,
  pub indexing: Vec<Rc<Expression>>,
}

impl Visited for IdentifierTerm {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.indexing.accept(visitor);
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

    Ok(())
  }
}

#[derive(Debug)]
pub struct TypedIdentifier {
  pub names: Vec<String>,
  pub type_declaration: TypeDeclaration,
}

impl Codegen for TypedIdentifier {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "{}: ", self.names.join(", "))?;
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

  pub mangled_accessor: RefCell<Option<String>>,
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

    if let Some(mangled_accessor) = self.mangled_accessor.borrow().as_deref() {
      write!(f, "{}", mangled_accessor)?;
    } else {
      context.transform_if_generic_type(f, &self.type_name)?;
    }

    if let Some(comma_separated_types) = &self.generic_type_assignment {
      // special case: array is the only generic type supported by vanilla WS
      if self.type_name == "array" {
        write!(f, "<")?;
        comma_separated_types.emit_join(context, f, ", ")?;
        write!(f, ">")?;
      } else {
        let generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
          &TypeDeclaration::stringified_generic_types(&self.generic_type_assignment, &context),
        );

        write!(f, "{generic_variant_suffix}")?;
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

  pub fn flat_type_names<'a>(
    type_name: &'a String, generic_type_assignment: &'a Option<Vec<TypeDeclaration>>,
  ) -> Vec<&'a str> {
    let mut output = vec![type_name.as_str()];

    if let Some(gen) = generic_type_assignment {
      for subtype in gen {
        for t in Self::flat_type_names(&subtype.type_name, &subtype.generic_type_assignment) {
          output.push(t);
        }
      }
    }

    output
  }

  pub fn stringified_generic_types(
    generic_type_assignment: &Option<Vec<TypeDeclaration>>, context: &Context,
  ) -> Vec<String> {
    match generic_type_assignment {
      None => Vec::new(),
      Some(generic_types) => generic_types
        .iter()
        .map(|t| {
          let mut type_name: Vec<u8> = Vec::new();
          let stringified_type = if t.generic_type_assignment.is_some() {
            let mut output = t.to_string();

            output.push_str(
              &Self::stringified_generic_types(&t.generic_type_assignment, context).join(""),
            );

            output
          } else {
            t.to_string()
          };

          let result = context.transform_if_generic_type(&mut type_name, &stringified_type);

          match result {
            Ok(_) => std::str::from_utf8(&type_name)
              .and_then(|s| Ok(s.to_string()))
              .unwrap_or_else(|err| stringified_type),
            Err(_) => stringified_type,
          }
        })
        .collect::<Vec<String>>(),
    }
  }
}
