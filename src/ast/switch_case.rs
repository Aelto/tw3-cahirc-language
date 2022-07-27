use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct SwitchStatement {
  pub compared: Rc<Expression>,
  pub cases: Vec<SwitchCaseStatement>,
}

impl Visited for SwitchStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.cases.accept(visitor);
  }
}

impl Codegen for SwitchStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "switch (")?;
    self.compared.emit(context, f)?;
    writeln!(f, ") {{")?;

    self.cases.emit_join(context, f, "\n")?;
    write!(f, "}}")
  }
}

#[derive(Debug)]
pub enum SwitchCaseStatement {
  Default {
    body_statements: Vec<FunctionBodyStatement>,
  },
  Case {
    cases: Vec<Rc<Expression>>,
    body_statements: Vec<FunctionBodyStatement>,
  },
}

impl Visited for SwitchCaseStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      SwitchCaseStatement::Default { body_statements } => {
        body_statements.accept(visitor);
      }
      SwitchCaseStatement::Case {
        cases,
        body_statements,
      } => {
        cases.accept(visitor);
        body_statements.accept(visitor);
      }
    };
  }
}

impl Codegen for SwitchCaseStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      SwitchCaseStatement::Default { body_statements } => {
        writeln!(f, "default:")?;
        body_statements.emit(context, f)?;
      }
      SwitchCaseStatement::Case {
        cases,
        body_statements,
      } => {
        for case in cases {
          write!(f, "case ")?;
          case.emit(context, f)?;
          writeln!(f, ":")?;
        }

        body_statements.emit(context, f)?;
      }
    };

    writeln!(f, "break;")?;

    Ok(())
  }
}
