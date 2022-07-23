use super::visitor::Visited;
use super::*;

use super::codegen::context::Context;

#[derive(Debug)]
pub struct ClassInstantiation {
  pub class_name: String,
  pub lifetime: String,
}

impl Visited for ClassInstantiation {
  fn accept<T: visitor::Visitor>(&self, _: &mut T) {}
}

impl Codegen for ClassInstantiation {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "new {} in {}", self.class_name, self.lifetime)
  }
}
