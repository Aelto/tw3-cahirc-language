use crate::ast::codegen::context::GenericContext;

use super::visitor::Visited;
use super::*;

use super::codegen::context::Context;

#[derive(Debug)]
pub struct ClassInstantiation {
  pub class_name: String,
  pub generic_type_assignment: Option<Vec<TypeDeclaration>>,
  pub lifetime: String,
}

impl ClassInstantiation {
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

impl Visited for ClassInstantiation {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    if let Some(generic_types) = &self.generic_type_assignment {
      visitor.visit_generic_class_instantiation(self);

      for t in generic_types {
        t.accept(visitor);
      }
    };
  }
}

impl Codegen for ClassInstantiation {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let stringified_types = match &self.generic_type_assignment {
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

    write!(
      f,
      "new {}{generic_variant_suffix} in {}",
      self.class_name, self.lifetime
    )
  }
}
