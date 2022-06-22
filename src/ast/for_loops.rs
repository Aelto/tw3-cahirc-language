use std::rc::Rc;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct ForStatement {
  pub initialization: Option<VariableDeclarationOrAssignment>,
  pub condition: Rc<Expression>,
  pub iteration: VariableAssignment,
  pub body_statements: Vec<FunctionBodyStatement>,
}

impl Visited for ForStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.initialization.accept(visitor);
    self.condition.accept(visitor);
    self.iteration.accept(visitor);
    self.body_statements.accept(visitor);
  }
}
