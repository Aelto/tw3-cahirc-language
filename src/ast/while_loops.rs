use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>,
}

impl Visited for WhileStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.condition.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for WhileStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "while (")?;
    self.condition.emit(context, f)?;
    writeln!(f, ") {{")?;
    self.body_statements.emit(context, f)?;
    writeln!(f, "}}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub struct DoWhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>,
}

impl Visited for DoWhileStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.condition.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for DoWhileStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    writeln!(f, "do {{")?;

    for statement in &self.body_statements {
      statement.emit(context, f)?;
    }

    write!(f, "}} while (")?;
    self.condition.emit(context, f)?;
    writeln!(f, ");");

    Ok(())
  }
}
