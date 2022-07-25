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
pub enum TypeDeclaration {
  Regular {
    type_name: String,
    generic_type_assignment: Option<Vec<TypeDeclaration>>,

    mangled_accessor: RefCell<Option<String>>,
  },
  Lambda(LambdaDeclaration),
}

impl Visited for TypeDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      TypeDeclaration::Regular {
        type_name: _,
        generic_type_assignment,
        mangled_accessor: _,
      } => {
        if let Some(generic_types) = &generic_type_assignment {
          visitor.visit_generic_variable_declaration(self);

          for t in generic_types {
            t.accept(visitor);
          }
        }
      }
      TypeDeclaration::Lambda(x) => x.accept(visitor),
    }
  }
}

impl Codegen for TypeDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      TypeDeclaration::Regular {
        type_name,
        generic_type_assignment,
        mangled_accessor,
      } => {
        if let Some(mangled_accessor) = mangled_accessor.borrow().as_deref() {
          write!(f, "{}", mangled_accessor)?;
        } else {
          context.transform_if_generic_type(f, &type_name)?;
        }

        if let Some(comma_separated_types) = &generic_type_assignment {
          // special case: array is the only generic type supported by vanilla WS
          if type_name == "array" {
            write!(f, "<")?;
            comma_separated_types.emit_join(context, f, ", ")?;
            write!(f, ">")?;
          } else {
            let stringified_types = match &generic_type_assignment {
              Some(x) => {
                let types = {
                  let mut list = Vec::new();

                  for child in x {
                    list.push(child);
                  }

                  list
                };

                TypeDeclaration::stringified_generic_types(&types, &context)
              }
              None => Vec::new(),
            };
            let generic_variant_suffix =
              GenericContext::generic_variant_suffix_from_types(&stringified_types);

            write!(f, "{generic_variant_suffix}")?;
          }
        }
      }
      TypeDeclaration::Lambda(x) => x.emit(context, f)?,
    };

    Ok(())
  }
}

impl TypeDeclaration {
  pub fn to_string(&self) -> String {
    match self {
      TypeDeclaration::Regular {
        type_name,
        generic_type_assignment,
        mangled_accessor: _,
      } => {
        if let Some(generics) = &generic_type_assignment {
          let mut output = type_name.clone();

          for generic in generics {
            output.push_str(&generic.to_string());
          }

          output
        } else {
          type_name.clone()
        }
      }
      TypeDeclaration::Lambda(_) => todo!(),
    }
  }

  /// Returns a flattened list of all the type names this type declaration contains.
  /// Mainly used for generic type declarations with nested types.
  pub fn flat_type_names<'a>(
    type_name: &'a String, generic_type_assignment: &'a Option<Vec<TypeDeclaration>>,
  ) -> Vec<&'a str> {
    let mut output = vec![type_name.as_str()];

    if let Some(gen) = generic_type_assignment {
      for subtype in gen {
        let subtypes = match subtype {
          TypeDeclaration::Regular {
            type_name,
            generic_type_assignment,
            mangled_accessor: _,
          } => Self::flat_type_names(&type_name, &generic_type_assignment),
          TypeDeclaration::Lambda(lambda) => lambda.flat_type_names(),
        };

        for t in subtypes {
          output.push(t);
        }
      }
    }

    output
  }

  pub fn stringified_generic_types<'a>(
    generic_types: &Vec<&'a TypeDeclaration>, context: &Context,
  ) -> Vec<String> {
    generic_types
      .into_iter()
      .map(|t| match t {
        TypeDeclaration::Regular {
          type_name: _,
          generic_type_assignment,
          mangled_accessor: _,
        } => {
          let mut type_name: Vec<u8> = Vec::new();
          let stringified_type = if let Some(subtypes) = generic_type_assignment {
            let mut output = t.to_string();
            let mut list = Vec::new();

            for subtype in subtypes {
              list.push(subtype);
            }

            output.push_str(&Self::stringified_generic_types(&list, context).join(""));

            output
          } else {
            t.to_string()
          };

          let result = context.transform_if_generic_type(&mut type_name, &stringified_type);

          match result {
            Ok(_) => std::str::from_utf8(&type_name)
              .and_then(|s| Ok(s.to_string()))
              .unwrap_or_else(|_| stringified_type),
            Err(_) => stringified_type,
          }
        }
        TypeDeclaration::Lambda(_) => todo!(),
      })
      .collect::<Vec<String>>()
  }
}
