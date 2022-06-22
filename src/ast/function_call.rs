use super::visitor::Visited;
use super::*;

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
    visitor.visit_generic_function_call(self);

    self.accessor.accept(visitor);
    self.parameters.accept(visitor);
  }
}
