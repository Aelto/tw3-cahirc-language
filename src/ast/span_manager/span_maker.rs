use std::collections::HashMap;

use super::{FilePathRef, Span, SpanManager};

pub struct SpanMaker<'a> {
  pub parent: &'a mut SpanManager,
  pub source_ref: FilePathRef,
  pub pool: HashMap<(usize, usize), Span>
}

impl<'a> SpanMaker<'a> {
  pub fn span(&mut self, left: usize, right: usize, _origin: &'static str) -> Span {
    *self
      .pool
      .entry((left, right))
      .or_insert_with(|| self.parent.new_span(self.source_ref, left, right))
  }
}
