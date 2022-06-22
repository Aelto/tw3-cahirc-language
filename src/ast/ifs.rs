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
