use std::collections::HashSet;
use std::rc::Rc;

use super::codegen::context::Context;
use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct ClassDeclaration {
  pub class_type: ClassType,
  pub name: String,
  pub extended_class_name: Option<String>,

  /// Mostly used by states, while defining `state Foo in parent_class_name`
  pub parent_class_name: Option<String>,
  pub generic_types: Option<Vec<String>>,
  pub body_statements: Vec<ClassBodyStatement>,

  pub span_name: Span,

  pub context: Rc<RefCell<Context>>
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
    } else {
      emit_class(self, &context, f, "")?;
    }

    Ok(())
  }
}

fn emit_class(
  this: &ClassDeclaration, context: &Context, f: &mut Vec<u8>, generic_variant_suffix: &str
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  let generic_variant_suffix_prefix = match generic_variant_suffix.is_empty() {
    true => "",
    false => "_"
  };

  if let Some(mangled_accessor) = &context.mangled_accessor {
    write!(
      f,
      "{} {}{generic_variant_suffix_prefix}{}",
      this.class_type, mangled_accessor, generic_variant_suffix
    )?;
  } else {
    write!(
      f,
      "{} {}{generic_variant_suffix_prefix}{}",
      this.class_type, this.name, generic_variant_suffix
    )?;
  }

  if let Some(parent_class_name) = &this.parent_class_name {
    write!(f, " in {parent_class_name}")?;
  }

  if let Some(extended_class_name) = &this.extended_class_name {
    write!(f, " extends {extended_class_name}")?;
  }

  writeln!(f, " {{")?;

  // // the hashset will allow us to emit duplicate variable names once.
  // let mut emitted_variable_names = HashSet::new();
  // for declaration in &context.variable_declarations {
  //   'name_emitting: for name in &declaration.names {
  //     if emitted_variable_names.contains(name.as_str()) {
  //       continue 'name_emitting;
  //     }

  //     write!(f, "var {name}: ")?;
  //     declaration.type_declaration.emit(context, f)?;
  //     writeln!(f, ";")?;

  //     emitted_variable_names.insert(name.clone());
  //   }
  // }

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
  State,
  Abstract
}

impl Display for ClassType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ClassType::Class => write!(f, "class"),
      ClassType::StatemachineClass => write!(f, "statemachine class"),
      ClassType::State => write!(f, "state"),
      ClassType::Abstract => write!(f, "abstract class")
    }
  }
}

#[derive(Debug)]
pub enum ClassBodyStatement {
  Method {
    encapsulation: Option<EncapsulationType>,
    function_declaration: Rc<FunctionDeclaration>
  },
  Property {
    encapsulation: Option<EncapsulationType>,
    property_declaration: VariableDeclaration,
    is_saved: bool,
    is_editable: bool
  },
  DefaultValue(VariableAssignment),
  Hint(VariableAssignment)
}

impl Codegen for ClassBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      ClassBodyStatement::Method {
        encapsulation,
        function_declaration
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
        is_saved,
        is_editable
      } => {
        if let Some(encapsulation) = encapsulation {
          encapsulation.emit(context, f)?;
          write!(f, " ")?;
        }

        if *is_editable {
          write!(f, "editable ")?;
        }

        if *is_saved {
          write!(f, "saved ")?;
        }

        property_declaration.emit(context, f)?;
      }
      ClassBodyStatement::DefaultValue(x) => {
        write!(f, "default ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?
      }
      ClassBodyStatement::Hint(x) => {
        write!(f, "hint ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?
      }
    };

    Ok(())
  }
}

#[derive(Debug)]
pub enum EncapsulationType {
  Private,
  Public,
  Protected
}

impl visitor::Visited for ClassBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      ClassBodyStatement::Method {
        encapsulation: _,
        function_declaration
      } => function_declaration.accept(visitor),
      ClassBodyStatement::Property {
        encapsulation: _,
        property_declaration,
        is_saved: _,
        is_editable: _
      } => property_declaration.accept(visitor),
      ClassBodyStatement::DefaultValue(_) => {}
      ClassBodyStatement::Hint(_) => {}
    }
  }
}

impl Codegen for EncapsulationType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      EncapsulationType::Private => write!(f, "private"),
      EncapsulationType::Public => write!(f, "public"),
      EncapsulationType::Protected => write!(f, "protected")
    }
  }
}
