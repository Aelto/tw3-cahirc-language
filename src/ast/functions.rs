use std::rc::Rc;

use super::*;

#[derive(Debug)]
/// property.
pub struct FunctionDeclaration {
  pub function_type: FunctionType,
  pub name: String,
  pub generic_types: Option<Vec<String>>,
  pub parameters: Vec<TypedIdentifier>,
  pub type_declaration: Option<TypeDeclaration>,
  pub body_statements: Vec<FunctionBodyStatement>,
  pub is_latent: bool,

  pub generic_calls: Rc<Vec<GenericCallsRegister>>
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Rc<Expression>),
  Return(Rc<Expression>),
  Assignement(VariableAssignment),
  IfStatement(IfStatement),
  ForStatement(ForStatement),
  WhileStatement(WhileStatement),
  DoWhileStatement(DoWhileStatement)
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Rc<Expression>>);

impl visitor::Visited for FunctionDeclaration {
    fn accept<T: visitor::Visitor>(&mut self, visitor: &mut T) {
        match visitor.visitor_type() {
            visitor::VisitorType::FunctionDeclarationVisitor => visitor.visit_function_declaration(self),
        }
    }
}