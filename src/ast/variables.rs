use super::*;

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: Box<IdentifierTerm>,
  pub assignment_type: AssignmentType,
  pub following_expression: Rc<Expression>
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignment {
  Declaration(VariableDeclaration),
  Assignement(VariableAssignment)
}

#[derive(Debug)]
pub struct VariableDeclaration {
  pub declaration: TypedIdentifier,
  pub following_expression: Option<Rc<Expression>>
}
