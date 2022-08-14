use super::codegen::type_inference::FunctionInferedType;
use super::visitor::Visited;
use super::*;

use super::codegen::context::Context;
use super::codegen::context::GenericContext;

#[derive(Debug)]
pub struct FunctionCall {
  pub accessor: Box<IdentifierTerm>,
  pub generic_types: Option<Vec<String>>,
  pub parameters: FunctionCallParameters,
  pub span: Span,

  pub mangled_accessor: RefCell<Option<String>>,
  pub infered_function_type: RefCell<Option<Rc<FunctionInferedType>>>
}

impl FunctionCall {
  pub fn get_function_name(&self) -> String {
    self.accessor.text.to_string()
  }
}

impl Visited for FunctionCall {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    if self.generic_types.is_some() {
      visitor.visit_generic_function_call(self);
    }

    visitor.visit_function_call(self);

    self.accessor.accept(visitor);
    self.parameters.accept(visitor);
  }
}

impl Codegen for FunctionCall {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    if let Some(mangled_accessor) = &self.mangled_accessor.borrow().as_deref() {
      let mut accessor = Vec::new();
      self.accessor.emit(context, &mut accessor)?;

      if let Ok(accessor) = &std::str::from_utf8(&accessor) {
        write!(
          f,
          "{}",
          accessor.replace(&self.get_function_name(), &mangled_accessor)
        )?;
      }
    } else {
      self.accessor.emit(context, f)?;
    }

    if let Some(generic_types) = &self.generic_types {
      let generic_variant_suffix =
        GenericContext::generic_variant_suffix_from_types(&generic_types);
      write!(f, "{generic_variant_suffix}")?;

      write!(f, "/*")?;

      for gtype in generic_types {
        write!(f, "{gtype}, ")?;
      }

      write!(f, "*/")?;
    }

    write!(f, "(")?;
    self.parameters.emit(context, f)?;
    write!(f, ")")?;

    Ok(())
  }
}
