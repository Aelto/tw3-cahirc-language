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

impl Display for WhileStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "while ({}) {{", self.condition)?;

    for statement in &self.body_statements {
      writeln!(f, "{statement}");
    }

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

impl Display for DoWhileStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "do {{")?;

    for statement in &self.body_statements {
      writeln!(f, "{statement}")?;
    }

    writeln!(f, "}} while ({});", self.condition)?;

    Ok(())
  }
}
