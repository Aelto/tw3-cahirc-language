use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub enum IfStatement {
  If {
    condition: Rc<Expression>,
    body_statements: Vec<FunctionBodyStatement>,
    else_statements: Vec<Box<IfStatement>>,
  },
  Else {
    condition: Option<Rc<Expression>>,
    body_statements: Vec<FunctionBodyStatement>,
  },
}

impl Visited for IfStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      IfStatement::If {
        condition,
        body_statements,
        else_statements,
      } => {
        condition.accept(visitor);
        body_statements.accept(visitor);
        else_statements.accept(visitor);
      }
      IfStatement::Else {
        condition,
        body_statements,
      } => {
        condition.accept(visitor);
        body_statements.accept(visitor);
      }
    }
  }
}

impl Codegen for IfStatement {
  fn emit(&self, context: &mut Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      IfStatement::If {
        condition,
        body_statements,
        else_statements,
      } => {
        write!(f, "if (")?;
        condition.emit(context, f)?;
        writeln!(f, ") {{")?;
        body_statements.emit(context, f)?;
        writeln!(f, "}}")?;
        else_statements.emit(context, f)?;
      }
      IfStatement::Else {
        condition,
        body_statements,
      } => {
        write!(f, "else ")?;

        if let Some(condition) = condition {
          write!(f, "if (")?;
          condition.emit(context, f)?;
          write!(f, ")")?;
        }

        writeln!(f, " {{")?;

        for statement in body_statements {
          statement.emit(context, f)?;
          writeln!(f, "")?;
        }

        writeln!(f, "}}")?;
      }
    }

    Ok(())
  }
}
