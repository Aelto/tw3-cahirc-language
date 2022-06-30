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

impl Codegen for Statement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    match self {
      Statement::Expression(x) => x.emit(context, f),
      Statement::FunctionDeclaration(x) => x.emit(context, f),
      Statement::ClassDeclaration(x) => x.emit(context, f),
      Statement::StructDeclaration(x) => x.emit(context, f),
    }
  }
}
