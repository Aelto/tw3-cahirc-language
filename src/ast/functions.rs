use std::rc::Rc;

use super::codegen::context::Context;
use super::visitor::Visited;
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

  pub context: Rc<RefCell<Context>>,
}

impl visitor::Visited for FunctionDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_function_declaration(&self);

    // don't go further, the context building visitor will create a new one
    // and continue traversing using the new one.
    match visitor.visitor_type() {
      visitor::VisitorType::ContextBuildingVisitor => return,
      _ => {}
    };

    self.body_statements.accept(visitor);
  }
}

impl Display for FunctionDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(generic_context) = &mut self.context.borrow_mut().generic_context {
      for variant in generic_context.translation_variants.keys() {
        generic_context.currently_used_variant = Some(variant.clone());

        emit_function(self, f, variant)?;
      }
    }
    else {
      emit_function(self, f, "")?;
    }

    Ok(())
  }
}

fn emit_function(this: &FunctionDeclaration, f: &mut std::fmt::Formatter<'_>, generic_variant_suffix: &str) -> std::fmt::Result {
  if this.is_latent {
    write!(f, "latent")?;
  }

  write!(f, "{} {}{}(", this.function_type, this.name, generic_variant_suffix)?;

  for param in &this.parameters {
    write!(f, "{param}, ")?;
  }

  write!(f, ")")?;

  if let Some(t) = &this.type_declaration {
    write!(f, ": {t}")?;
  }

  writeln!(f, " {{")?;

  for statement in &this.body_statements {
    writeln!(f, "{statement}")?;
  }

  writeln!(f, "}}")?;

  Ok(())
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event,
}

impl Display for FunctionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FunctionType::Function => write!(f, "function"),
      FunctionType::Timer => write!(f, "timer"),
      FunctionType::Event => write!(f, "event"),
    }
  }
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
  DoWhileStatement(DoWhileStatement),
}

impl visitor::Visited for FunctionBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match &self {
      FunctionBodyStatement::VariableDeclaration(x) => x.accept(visitor),
      FunctionBodyStatement::Expression(x) => x.accept(visitor),
      FunctionBodyStatement::Return(x) => x.accept(visitor),
      FunctionBodyStatement::Assignement(x) => x.accept(visitor),
      FunctionBodyStatement::IfStatement(x) => x.accept(visitor),
      FunctionBodyStatement::ForStatement(x) => x.accept(visitor),
      FunctionBodyStatement::WhileStatement(x) => x.accept(visitor),
      FunctionBodyStatement::DoWhileStatement(x) => x.accept(visitor),
    };
  }
}

impl Display for FunctionBodyStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FunctionBodyStatement::VariableDeclaration(x) => write!(f, "{x};"),
      FunctionBodyStatement::Expression(x) => write!(f, "{x};"),
      FunctionBodyStatement::Return(x) => writeln!(f, "return {x};"),
      FunctionBodyStatement::Assignement(x) => write!(f, "{x};"),
      FunctionBodyStatement::IfStatement(x) => write!(f, "{x}"),
      FunctionBodyStatement::ForStatement(x) => write!(f, "{x}"),
      FunctionBodyStatement::WhileStatement(x) => write!(f, "{x}"),
      FunctionBodyStatement::DoWhileStatement(x) => write!(f, "{x}"),
    }
  }
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Rc<Expression>>);

impl Visited for FunctionCallParameters {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.0.accept(visitor);
  }
}

impl Display for FunctionCallParameters {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for param in &self.0 {
      write!(f, "{}, ", param)?;
    }

    Ok(())
  }
}
