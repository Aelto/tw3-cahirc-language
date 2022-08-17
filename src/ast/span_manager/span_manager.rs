use std::{collections::HashMap, ops::Range};

use super::SpanMaker;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span(pub usize);

pub type FilePath = String;
pub type FilePathRef = usize;


pub struct SpanRange {
  #[allow(dead_code)]
  source_ref: FilePathRef,

  left: usize,
  right: usize
}

pub struct SpanManager {
  /// Stores the path for the sources
  pub paths: Vec<FilePath>,

  pub spans: Vec<SpanRange>
}

impl SpanManager {
  pub fn new() -> Self {
    Self {
      paths: Vec::new(),
      spans: Vec::new()
    }
  }

  pub fn new_span(&mut self, source_ref: FilePathRef, left: usize, right: usize) -> Span {
    let i = self.spans.len();

    self.spans.push(SpanRange {
      source_ref,
      left,
      right
    });

    Span(i)
  }

  pub fn add_source(&mut self, source: FilePath) -> SpanMaker {
    let source_ref = self.paths.len();

    self.paths.push(source);

    SpanMaker {
      parent: self,
      source_ref,
      pool: HashMap::new()
    }
  }

  pub fn add_fake_source(&mut self) -> SpanMaker {
    let source_ref = self.paths.len();

    self.paths.push(String::new());

    SpanMaker {
      parent: self,
      source_ref,
      pool: HashMap::new()
    }
  }

  pub fn get_left(&self, source_ref: Span) -> usize {
    self.spans[source_ref.0].left
  }

  pub fn get_right(&self, source_ref: Span) -> usize {
    self.spans[source_ref.0].right
  }

  pub fn get_range(&self, source_ref: Span) -> Range<usize> {
    self.get_left(source_ref)..self.get_right(source_ref)
  }

  pub fn get_source(&self, source_ref: &Span) -> &FilePath {
    let span = &self.spans[source_ref.0];

    &self.paths[span.source_ref]
  }
}
