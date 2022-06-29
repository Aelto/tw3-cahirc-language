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

impl Codegen for ForStatement {
  fn emit(&self, context: &mut Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "for (")?;
    self.initialization.emit(context, f)?;
    write!(f, "; ")?;
    self.condition.emit(context, f)?;
    write!(f, "; ")?;
    self.iteration.emit(context, f)?;
    writeln!(f, ") {{")?;

    
    for statement in &self.body_statements {
      statement.emit(context, f)?;
      writeln!(f, "")?;
    }

    writeln!(f, "}}")?;

    Ok(())
  }
}
