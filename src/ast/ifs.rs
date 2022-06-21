use super::*;

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
