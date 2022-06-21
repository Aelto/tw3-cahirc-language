use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::rc::Rc;

use super::*;

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,
  pub body_statements: Vec<ClassBodyStatement>
}

#[derive(Debug)]
pub enum ClassType {
  Class,
  StatemachineClass
}

#[derive(Debug)]
pub enum ClassBodyStatement {
  Method {
    encapsulation: Option<EncapsulationType>,
    function_declaration: Rc<RefCell<FunctionDeclaration>>
  },
  Property {
    encapsulation: Option<EncapsulationType>,
    property_declaration: VariableDeclaration
  },
  DefaultValue(VariableAssignment)
}

#[derive(Debug)]
pub enum EncapsulationType {
  Private,
  Public,
  Protected
}

impl visitor::Visited for ClassBodyStatement {
    fn accept<T: visitor::Visitor>(&mut self, visitor: &mut T) {
        match self {
            ClassBodyStatement::Method { encapsulation, function_declaration } => match visitor.visitor_type() {
                visitor::VisitorType::FunctionDeclarationVisitor => {
                  (**function_declaration).borrow_mut().accept(visitor);
                },
                _ => {}
            },
            ClassBodyStatement::Property { encapsulation, property_declaration } => {},
            ClassBodyStatement::DefaultValue(_) => {},
        }
    }
}