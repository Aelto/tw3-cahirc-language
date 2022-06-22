use super::*;

#[derive(Debug)]
pub enum Statement {
  Expression(Rc<Expression>),
  FunctionDeclaration(Rc<FunctionDeclaration>),
  ClassDeclaration(ClassDeclaration),
  StructDeclaration(StructDeclaration),
}

impl visitor::Visited for Statement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      Statement::Expression(x) => x.accept(visitor),
      Statement::FunctionDeclaration(x) => x.accept(visitor),
      Statement::ClassDeclaration(x) => x.accept(visitor),
      Statement::StructDeclaration(x) => x.accept(visitor),
    }
  }
}

impl Display for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Statement::Expression(x) => write!(f, "{x}"),
      Statement::FunctionDeclaration(x) => write!(f, "{x}"),
      Statement::ClassDeclaration(x) => write!(f, "{x}"),
      Statement::StructDeclaration(x) => write!(f, "{x}"),
    }
  }
}
