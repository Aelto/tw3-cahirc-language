use std::borrow::BorrowMut;

use super::*;

#[derive(Debug)]
pub enum Statement {
  Expression(Rc<Expression>),
  FunctionDeclaration(Rc<RefCell<FunctionDeclaration>>),
  ClassDeclaration(ClassDeclaration),
  StructDeclaration(StructDeclaration)
}

impl visitor::Visited for Statement {
    fn accept<T: visitor::Visitor>(&mut self, visitor: &mut T) {
        match self {
            Statement::Expression(_) => {
              // at the moment stop all visitors for expressions
              
            },
            Statement::FunctionDeclaration(declaration) => match visitor.visitor_type() {
              // accept only the function declaration visitor
              visitor::VisitorType::FunctionDeclarationVisitor => {

                (**declaration).borrow_mut().accept(visitor)
              },
              _ => {}
            },
            Statement::ClassDeclaration(declaration) => match visitor.visitor_type() {
                visitor::VisitorType::FunctionDeclarationVisitor => {
                  for body_statement in &mut declaration.body_statements {
                    body_statement.accept(visitor);
                  }
                },
            },
            Statement::StructDeclaration(_) => {},
        }
    }
}