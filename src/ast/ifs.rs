use super::*;
use super::visitor::Visited;

#[derive(Debug)]
pub enum IfStatement {
  If {
    condition: Rc<Expression>,
    body_statements: Vec<FunctionBodyStatement>,
    else_statements: Vec<Box<IfStatement>>
  },
  Else {
    condition: Option<Rc<Expression>>,
    body_statements: Vec<FunctionBodyStatement>
  }
}

impl Visited for IfStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
        IfStatement::If { condition, body_statements, else_statements } => {
          condition.accept(visitor);

          for statement in body_statements {
            statement.accept(visitor);
          }

          for else_statement in else_statements {
            else_statement.accept(visitor);
          }
        },
        IfStatement::Else { condition, body_statements } => {
          if let Some(condition) = condition {
            condition.accept(visitor);
          }

          for statement in body_statements {
            statement.accept(visitor);
          }
        },
    }
  }
}
