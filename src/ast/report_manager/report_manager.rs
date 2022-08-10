use core::panic;

use ariadne::{Report, Source};

pub struct ReportManager {
  reports: Vec<Report>
}

impl ReportManager {
  pub fn new() -> Self {
    Self {
      reports: Vec::new()
    }
  }

  pub fn push(&mut self, report: Report) {
    self.reports.push(report);
  }

  pub fn push_many(&mut self, reports: Vec<Report>) {
    for report in reports {
      self.push(report)
    }
  }

  pub fn flush_reports(&mut self) {
    self.reports.clear();
  }

  pub fn consume(&mut self, content: &str) {
    for report in &self.reports {
      if let Err(err) = report.print(Source::from(&content)) {
        panic!("{}", err);
      }
    }
    
    self.flush_reports();
  }

  pub fn consume_multiple_sources(&mut self) {
    for report in &self.reports {
      // todo: get the corresponding source from the span
      // todo: give the span with the reports.
      let source: String = todo!();

      if let Err(err) = report.print(Source::from(&source)) {
        panic!("{}", err);
      }
    }
  }
}