use core::panic;

use ariadne::{Report, Source};

use crate::{ast::{Span, SpanManager}, preprocessor::types::PreprocessorOutput};

pub struct ReportManager {
  reports: Vec<(Report, Span)>
}

impl ReportManager {
  pub fn new() -> Self {
    Self {
      reports: Vec::new()
    }
  }

  pub fn push(&mut self, report: Report, span: Span) {
    self.reports.push((report, span));
  }

  pub fn push_many(&mut self, reports: Vec<(Report, Span)>) {
    for pair in reports {
      self.push(pair.0, pair.1);
    }
  }

  pub fn flush_reports(&mut self) {
    self.reports.clear();
  }

  pub fn consume(&mut self, content: &str) {
    for (report, _) in &self.reports {
      if let Err(err) = report.print(Source::from(&content)) {
        panic!("{}", err);
      }
    }
    
    self.flush_reports();
  }

  pub fn consume_multiple_sources<'a>(&mut self, span_manager: &'a mut SpanManager, preprocessor_output: &PreprocessorOutput) {
    for (report, span) in &self.reports {
      let source: &String = span_manager.get_source(span);
      let source_content = if let Some(s) = preprocessor_output.source_files_content.get(source) {
        s.content.borrow()
      } else {
        let mut output = None;

        for (_name, value) in preprocessor_output.dependencies_files_content.iter() {
          if let Some(s) = value.get(source) {

            output = Some(s.content.borrow())
          }
        }

        match output {
          Some(s) => s,
          None => panic!("attempt at reporting error but source {source} from span {} does not exist in neither the dependencies and the sources", span.0)
        }
      };

      if let Err(err) = report.print(Source::from(source_content.as_str())) {
        panic!("{}", err);
      }
    }
  }
}