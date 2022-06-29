use super::visitor::Visited;
use super::*;

use super::codegen::context::GenericContext;
use super::codegen::context::Context;

#[derive(Debug)]
pub struct FunctionCall {
  pub accessor: Box<IdentifierTerm>,
  pub generic_types: Option<Vec<String>>,
  pub parameters: FunctionCallParameters,
}

impl FunctionCall {
  pub fn get_function_name(&self) -> String {
    self.accessor.get_last_text()
  }
}

impl Visited for FunctionCall {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    if self.generic_types.is_some() {
      visitor.visit_generic_function_call(self);
    }

    self.accessor.accept(visitor);
    self.parameters.accept(visitor);
  }
}

impl Codegen for FunctionCall {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    self.accessor.emit(context, f)?;

    if let Some(generic_types) = &self.generic_types {
      let generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(&generic_types);
      write!(f, "{generic_variant_suffix}")?;

      write!(f, "/*")?;

      for gtype in generic_types {
        write!(f, "{gtype}")?;
      }

      write!(f, "*/")?;
    }

    write!(f, "(")?;
    self.parameters.emit(context, f)?;
    write!(f, ")")?;

    Ok(())
  }
}
