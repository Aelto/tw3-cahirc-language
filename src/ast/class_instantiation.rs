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
  fn accept<T: visitor::Visitor>(&self, _: &mut T) {}
}

impl Codegen for ClassInstantiation {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
      &TypeDeclaration::stringified_generic_types(&self.generic_type_assignment, &context),
    );

    write!(
      f,
      "new {}{generic_variant_suffix} in {}",
      self.class_name, self.lifetime
    )
  }
}
