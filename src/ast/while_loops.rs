use super::*;

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}

#[derive(Debug)]
pub struct DoWhileStatement {
  pub condition: Rc<Expression>,
  pub body_statements: Vec<FunctionBodyStatement>
}
