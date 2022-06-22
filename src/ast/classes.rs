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

impl Display for ClassDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.class_type, self.name)?;

    if let Some(extended_class_name) = &self.extended_class_name {
      write!(f, " extends {extended_class_name}")?;
    }

    writeln!(f, " {{")?;

    for statement in &self.body_statements {
      writeln!(f, "{statement}")?;
    }

    writeln!(f, "}}")?;

    Ok(())
  }
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

impl Display for ClassBodyStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ClassBodyStatement::Method {
        encapsulation,
        function_declaration,
      } => {
        if let Some(encapsulation) = encapsulation {
          write!(f, "{encapsulation} ")?;
        }

        write!(f, "{function_declaration}")?;
      }
      ClassBodyStatement::Property {
        encapsulation,
        property_declaration,
      } => {
        if let Some(encapsulation) = encapsulation {
          write!(f, "{encapsulation} ")?;
        }

        write!(f, "{property_declaration};")?;
      }
      ClassBodyStatement::DefaultValue(x) => writeln!(f, "default {x};")?,
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

impl Display for EncapsulationType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EncapsulationType::Private => write!(f, "private"),
      EncapsulationType::Public => write!(f, "public"),
      EncapsulationType::Protected => write!(f, "protected"),
    }
  }
}
