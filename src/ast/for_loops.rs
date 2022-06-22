use std::rc::Rc;

use super::*;
use super::visitor::Visited;

#[derive(Debug)]
pub struct ForStatement {
  pub initialization: Option<VariableDeclarationOrAssignment>,
  pub condition: Rc<Expression>,
  pub iteration: VariableAssignment,
  pub body_statements: Vec<FunctionBodyStatement>
}

impl Visited for ForStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    if let Some(initialization) = &self.initialization {
      initialization.accept(visitor);
    }

    self.condition.accept(visitor);
    self.iteration.accept(visitor);
    
    for statement in &self.body_statements {
      statement.accept(visitor);
    }
  }
}