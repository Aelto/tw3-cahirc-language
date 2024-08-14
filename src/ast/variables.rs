use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct VariableAssignment {
  pub variable_name: Rc<Expression>,
  pub assignment_type: AssignmentType,
  pub following_expression: Rc<Expression>
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
  Assignement(VariableAssignment)
}

impl Visited for VariableDeclarationOrAssignment {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      VariableDeclarationOrAssignment::Declaration(x) => x.accept(visitor),
      VariableDeclarationOrAssignment::Assignement(x) => x.accept(visitor)
    }
  }
}

impl Codegen for VariableDeclarationOrAssignment {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    match self {
      VariableDeclarationOrAssignment::Declaration(x) => x.emit(context, f),
      VariableDeclarationOrAssignment::Assignement(x) => x.emit(context, f)
    }
  }
}

#[derive(Debug)]
pub enum VariableDeclaration {
  Explicit {
    declaration: Rc<TypedIdentifier>,
    following_expression: Option<Rc<Expression>>
  },
  Implicit {
    names: Vec<String>,
    following_expression: Rc<Expression>
  }
}

impl visitor::Visited for VariableDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_variable_declaration(self);

    match &self {
      VariableDeclaration::Explicit {
        declaration,
        following_expression
      } => {
        declaration.accept(visitor);
        following_expression.accept(visitor);
      }
      VariableDeclaration::Implicit {
        names: _,
        following_expression
      } => {
        following_expression.accept(visitor);
      }
    }
  }
}

impl Codegen for VariableDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;
    match context.context_type {
      ContextType::Global
      | ContextType::ClassOrStruct
      | ContextType::State {
        parent_class_name: _
      } => {
        match &self {
          VariableDeclaration::Explicit {
            declaration,
            following_expression
          } => {
            write!(f, "var ")?;
            declaration.emit(context, f)?;
            writeln!(f, ";")?;

            if let Some(expr) = &following_expression {
              if let Some(variable_name) = declaration.names.first() {
                write!(f, "default {variable_name}")?;
              }

              write!(f, " = ")?;
              expr.emit(context, f)?;
              writeln!(f, ";")?;
            }
          }
          VariableDeclaration::Implicit {
            names: _,
            following_expression: _
          } => {
            panic!("The compiler does not support implicit types for class attributes, please write the types of your attributes explicitly.");
          }
        };
      }

      // variables are emitted manually by the functions, it is part of the feature
      // allowing variable declarations anywhere in function bodies.
      //
      ContextType::Function => {
        match &self {
          VariableDeclaration::Explicit {
            declaration,
            following_expression
          } => {
            if let Some(expr) = &following_expression {
              if let Some(variable_name) = declaration.names.first() {
                write!(f, "{variable_name}")?;
              }

              write!(f, " = ")?;
              expr.emit(context, f)?;
              writeln!(f, ";")?;
            }
          }
          VariableDeclaration::Implicit {
            names,
            following_expression
          } => {
            if let Some(variable_name) = names.first() {
              write!(f, "{variable_name}")?;
            }

            write!(f, " = ")?;
            following_expression.emit(context, f)?;
            writeln!(f, ";")?;
          }
        };
      }
    }

    Ok(())
  }
}
