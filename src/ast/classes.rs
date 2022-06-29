use std::rc::Rc;

use super::codegen::context::Context;
use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,
  pub generic_types: Option<Vec<String>>,
  pub body_statements: Vec<ClassBodyStatement>,

  pub context: Rc<RefCell<Context>>,
}

impl Visited for ClassDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_class_declaration(&self);

    // don't go further, the context building visitor will create a new one
    // and continue traversing using the new one.
    match visitor.visitor_type() {
      visitor::VisitorType::ContextBuildingVisitor => return,
      _ => {}
    };

    self.body_statements.accept(visitor);
  }
}

impl Codegen for ClassDeclaration {
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

        emit_class(self, &self.context.borrow(), f, &variant)?;
      }
    }
    else {
      emit_class(self, &context, f, "")?;
    }

    Ok(())
  }
}

fn emit_class(this: &ClassDeclaration, context: &Context, f: &mut Vec<u8>, generic_variant_suffix: &str) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  write!(f, "{} {}", this.class_type, this.name)?;

  if let Some(extended_class_name) = &this.extended_class_name {
    write!(f, " extends {extended_class_name}")?;
  }

  writeln!(f, " {{")?;

  for statement in &this.body_statements {
    statement.emit(context, f)?;
    writeln!(f, "")?;
  }

  writeln!(f, "}}")
}

#[derive(Debug)]
pub enum ClassType {
  Class,
  StatemachineClass,
}

impl Display for ClassType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ClassType::Class => write!(f, "class"),
      ClassType::StatemachineClass => write!(f, "statemachine class"),
    }
  }
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

impl Codegen for ClassBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      ClassBodyStatement::Method {
        encapsulation,
        function_declaration,
      } => {
        if let Some(encapsulation) = encapsulation {
          encapsulation.emit(context, f)?;
          write!(f, " ")?;
        }

        function_declaration.emit(context, f)?;
      }
      ClassBodyStatement::Property {
        encapsulation,
        property_declaration,
      } => {
        if let Some(encapsulation) = encapsulation {
          encapsulation.emit(context, f)?;
          write!(f, " ")?;
        }

        property_declaration.emit(context, f)?;
        writeln!(f, ";");
      }
      ClassBodyStatement::DefaultValue(x) => {

        write!(f, "default ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?
      },
    };

    Ok(())
  }
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

impl Codegen for EncapsulationType {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      EncapsulationType::Private => write!(f, "private"),
      EncapsulationType::Public => write!(f, "public"),
      EncapsulationType::Protected => write!(f, "protected"),
    }
  }
}
