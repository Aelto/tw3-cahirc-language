use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: Rc<Expression>,
  pub assignment_type: AssignmentType,
  pub following_expression: Rc<Expression>,
}

impl Visited for VariableAssignment {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.variable_name.accept(visitor);
    self.following_expression.accept(visitor);
  }
}

impl Codegen for VariableAssignment {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    self.variable_name.emit(context, f)?;
    write!(f, " ")?;
    self.assignment_type.emit(context, f)?;
    write!(f, " ")?;
    self.following_expression.emit(context, f)
  }
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignment {
  Declaration(VariableDeclaration),
  Assignement(VariableAssignment),
}

impl Visited for VariableDeclarationOrAssignment {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      VariableDeclarationOrAssignment::Declaration(x) => x.accept(visitor),
      VariableDeclarationOrAssignment::Assignement(x) => x.accept(visitor),
    }
  }
}

impl Codegen for VariableDeclarationOrAssignment {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    match self {
      VariableDeclarationOrAssignment::Declaration(x) => x.emit(context, f),
      VariableDeclarationOrAssignment::Assignement(x) => x.emit(context, f),
    }
  }
}

#[derive(Debug)]
pub struct VariableDeclaration {
  pub declaration: Rc<TypedIdentifier>,
  pub following_expression: Option<Rc<Expression>>,
}

impl visitor::Visited for VariableDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_variable_declaration(self);

    self.declaration.accept(visitor);
    self.following_expression.accept(visitor);
  }
}

impl Codegen for VariableDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;
    // variables are emitted manually by the functions, it is part of the feature
    // allowing variable declarations anywhere in function bodies.
    //
    // write!(f, "var ")?;
    // self.declaration.emit(context, f)?;

    if let Some(expr) = &self.following_expression {
      if let Some(variable_name) = self.declaration.names.first() {
        write!(f, "{variable_name}")?;
      }

      write!(f, " = ")?;
      expr.emit(context, f)?;
      writeln!(f, ";")?;
    }

    Ok(())
  }
}
