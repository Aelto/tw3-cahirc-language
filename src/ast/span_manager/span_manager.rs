use std::{collections::HashMap};

use super::SpanMaker;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span(usize);

pub type FilePath = String;
pub type FilePathRef = usize;


pub struct SpanRange {
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
}
