use crate::ast::codegen::Codegen;
use crate::ast::Context;

use super::*;

#[derive(Debug)]
pub struct Spanned<T> {
  pub span: Span,
  value: T
}

impl<T> Spanned<T> {
  pub fn new(value: T, span: Span) -> Self {
    Self { span, value }
  }
}

impl<T> Codegen for Spanned<T>
where
  T: Codegen
{
  fn emit(&self, context: &Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.value.emit(context, output)
  }
}
