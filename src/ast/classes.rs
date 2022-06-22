use std::rc::Rc;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,
  pub body_statements: Vec<ClassBodyStatement>,
}

impl Visited for ClassDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.body_statements.accept(visitor);
  }
}

#[derive(Debug)]
pub enum ClassType {
  Class,
  StatemachineClass,
}

#[derive(Debug)]
pub enum ClassBodyStatement {
  Method {
    encapsulation: Option<EncapsulationType>,
    function_declaration: Rc<FunctionDeclaration>,
  },
  Property {
    encapsulation: Option<EncapsulationType>,
    property_declaration: VariableDeclaration,
  },
  DefaultValue(VariableAssignment),
}

#[derive(Debug)]
pub enum EncapsulationType {
  Private,
  Public,
  Protected,
}

impl visitor::Visited for ClassBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      ClassBodyStatement::Method {
        encapsulation: _,
        function_declaration,
      } => function_declaration.accept(visitor),
      ClassBodyStatement::Property {
        encapsulation: _,
        property_declaration,
      } => property_declaration.accept(visitor),
      ClassBodyStatement::DefaultValue(_) => {}
    }
  }
}
