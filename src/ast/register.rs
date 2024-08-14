use std::rc::Rc;

use super::codegen::Codegen;
use super::visitor::{self, Visited};
use super::{Context, Expression};

#[derive(Debug)]
pub struct Register {
  pub expression: Rc<Expression>,
  pub register_name: String
}

impl Visited for Register {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.expression.accept(visitor);
  }
}

impl Codegen for Register {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    Ok(())
  }
}
