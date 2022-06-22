use super::*;
use super::visitor::Visited;

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

impl Visited for WhileStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.condition.accept(visitor);

    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}

#[derive(Debug)]
pub struct DoWhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

impl Visited for DoWhileStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.condition.accept(visitor);

    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}