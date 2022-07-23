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

impl Codegen for FunctionDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    let has_generic_context = self.context.borrow().generic_context.is_some();
    if has_generic_context {
      let mut variants = Vec::new();

      if let Some(generic_context) = &self.context.borrow().generic_context {
        for variant in generic_context.translation_variants.keys() {
          variants.push(String::from(variant));
        }
      }

      for variant in variants {
        {
          if let Some(generic_context) = &mut self.context.borrow_mut().generic_context {
            generic_context.currently_used_variant = Some(variant.clone());
          }
        }

        emit_function(self, &self.context.borrow(), f, &variant)?;
      }
    } else {
      emit_function(self, &context, f, "")?;
    }

    Ok(())
  }
}

fn emit_function(
  this: &FunctionDeclaration, context: &Context, f: &mut Vec<u8>, generic_variant_suffix: &str,
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  this.function_type.emit(context, f)?;

  if let Some(mangled_accessor) = &context.mangled_accessor {
    write!(f, " {}{}(", mangled_accessor, generic_variant_suffix)?;
  } else {
    write!(f, " {}{}(", this.name, generic_variant_suffix)?;
  }

  this.parameters.emit(context, f)?;
  write!(f, ")")?;

  if let Some(t) = &this.type_declaration {
    write!(f, ": ")?;
    t.emit(context, f)?;
  }

  writeln!(f, " {{")?;

  for statement in &this.body_statements {
    statement.emit(context, f)?;
    // writeln!(f, "");
  }

  writeln!(f, "}}")?;

  Ok(())
}

#[derive(Debug)]
pub enum FunctionType {
  Function,
  Timer,
  Event,
  Entry,
  Latent,
}

impl Codegen for FunctionType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      FunctionType::Function => write!(f, "function"),
      FunctionType::Timer => write!(f, "timer"),
      FunctionType::Event => write!(f, "event"),
      FunctionType::Entry => write!(f, "entry function"),
      FunctionType::Latent => write!(f, "latent function"),
    }
  }
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Rc<Expression>),
  Return(Option<Rc<Expression>>),
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

impl Codegen for FunctionBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      FunctionBodyStatement::VariableDeclaration(x) => {
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
      FunctionBodyStatement::Expression(x) => {
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
      FunctionBodyStatement::Return(x) => {
        write!(f, "return ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
      FunctionBodyStatement::Assignement(x) => {
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
      FunctionBodyStatement::IfStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::ForStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::WhileStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::DoWhileStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
    };

    Ok(())
  }
}

#[derive(Debug)]
pub struct FunctionCallParameters(pub Vec<Option<Rc<Expression>>>);

impl Visited for FunctionCallParameters {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.0.accept(visitor);
  }
}

impl Codegen for FunctionCallParameters {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    for param in &self.0 {
      param.emit(context, f)?;
      write!(f, ", ")?;
    }

    Ok(())
  }
}
