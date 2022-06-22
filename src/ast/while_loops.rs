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
