use std::collections::HashSet;
use std::rc::Rc;

use super::codegen::context::Context;
use super::codegen::type_inference::FunctionInferedParameterType;
use super::visitor::Visited;
use super::*;

#[derive(Debug)]
/// property.
pub struct FunctionDeclaration {
  pub function_type: FunctionType,
  pub name: String,
  pub generic_types: Option<Vec<String>>,
  pub parameters: Vec<FunctionDeclarationParameter>,
  pub type_declaration: Option<TypeDeclaration>,
  pub body_statements: Vec<FunctionBodyStatement>,

  pub span_name: Span,

  pub context: Rc<RefCell<Context>>
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

    self.parameters.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for FunctionDeclaration {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
      emit_function(self, &self.context.borrow(), f, "")?;
    }

    Ok(())
  }
}

fn emit_function(
  this: &FunctionDeclaration, context: &Context, f: &mut Vec<u8>, generic_variant_suffix: &str
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  this.function_type.emit(context, f)?;

  let generic_variant_suffix_prefix = match generic_variant_suffix.is_empty() {
    true => "",
    false => "_"
  };

  if let Some(mangled_accessor) = &context.mangled_accessor {
    write!(
      f,
      " {}{generic_variant_suffix_prefix}{}(",
      mangled_accessor, generic_variant_suffix
    )?;
  } else {
    write!(
      f,
      " {}{generic_variant_suffix_prefix}{}(",
      this.name, generic_variant_suffix
    )?;
  }

  this.parameters.emit_join(context, f, ", ")?;
  write!(f, ")")?;

  if let Some(t) = &this.type_declaration {
    write!(f, ": ")?;
    t.emit(context, f)?;
  }

  writeln!(f, " {{")?;

  // the hashset will allow us to emit duplicate variable names once.
  let mut emitted_variable_names = HashSet::new();
  for declaration in &context.variable_declarations {
    'name_emitting: for name in &declaration.names {
      if emitted_variable_names.contains(name.as_str()) {
        continue 'name_emitting;
      }

      write!(f, "var {name}: ")?;
      declaration.type_declaration.emit(context, f)?;
      writeln!(f, ";")?;

      emitted_variable_names.insert(name.clone());
    }
  }

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
  Exec
}

impl Codegen for FunctionType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      FunctionType::Function => write!(f, "function"),
      FunctionType::Timer => write!(f, "timer function"),
      FunctionType::Event => write!(f, "event"),
      FunctionType::Entry => write!(f, "entry function"),
      FunctionType::Latent => write!(f, "latent function"),
      FunctionType::Exec => write!(f, "exec function")
    }
  }
}

#[derive(Debug)]
pub enum FunctionBodyStatement {
  VariableDeclaration(VariableDeclaration),
  Expression(Rc<Expression>),
  Return(Option<Rc<Expression>>),
  Break,
  Continue,
  Assignement(VariableAssignment),
  IfStatement(IfStatement),
  ForStatement(ForStatement),
  ForInStatement(ForInStatement),
  WhileStatement(WhileStatement),
  DoWhileStatement(DoWhileStatement),
  SwitchStatement(SwitchStatement),
  Delete(Rc<Expression>)
}

impl FunctionBodyStatement {
  /// Todo:
  /// Could be improved to use the deduce_type function rather than force a cast.
  ///
  /// When None is returned it implies a Void type.
  pub fn get_return_type_from_last_statement(statements: &Vec<Self>) -> Option<&String> {
    match statements.last().unwrap() {
      FunctionBodyStatement::Expression(expression) => {
        let body = &expression.body;

        match body {
          ExpressionBody::Cast(cast_type, _) => Some(cast_type),
          _ => None
        }
      }
      FunctionBodyStatement::Return(x) => match x {
        Some(expression) => match &expression.body {
          ExpressionBody::Cast(cast_type, _) => Some(cast_type),
          _ => None
        },
        None => None
      },

      _ => None
    }
  }
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
      FunctionBodyStatement::SwitchStatement(x) => x.accept(visitor),
      FunctionBodyStatement::Delete(x) => x.accept(visitor),
      FunctionBodyStatement::Break => {}
      FunctionBodyStatement::Continue => {}
      FunctionBodyStatement::ForInStatement(x) => x.accept(visitor)
    };
  }
}

impl Codegen for FunctionBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      FunctionBodyStatement::VariableDeclaration(x) => {
        x.emit(context, f)?;
        // the semicolon is added by the variable declaration itself as it does
        // not always need to be emitted.
        // writeln!(f, ";")?;
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
      FunctionBodyStatement::ForInStatement(x) => {
        x.emit(context, f)?;
        write!(f, "")?;
      }
      FunctionBodyStatement::WhileStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::DoWhileStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::SwitchStatement(x) => {
        x.emit(context, f)?;
        writeln!(f, "")?;
      }
      FunctionBodyStatement::Break => {
        writeln!(f, "break;")?;
      }
      FunctionBodyStatement::Continue => {
        writeln!(f, "continue;")?;
      }
      FunctionBodyStatement::Delete(x) => {
        write!(f, "delete ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?;
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
    self.0.emit_join(context, f, ", ")
  }
}

#[derive(Debug)]
pub struct FunctionDeclarationParameter {
  pub parameter_type: ParameterType,
  pub typed_identifier: TypedIdentifier,
  pub span: Span
}

impl FunctionDeclarationParameter {
  pub fn flat_type_names<'a>(parameters: &'a Vec<Self>) -> Vec<&'a str> {
    let mut output = vec![];

    for param in parameters {
      let subtypes = match &param.typed_identifier.type_declaration {
        TypeDeclaration::Regular {
          type_name,
          generic_type_assignment,
          mangled_accessor: _
        } => TypeDeclaration::flat_type_names(&type_name, &generic_type_assignment),
        TypeDeclaration::Lambda(x) => Self::flat_type_names(&x.parameters)
      };

      for t in subtypes {
        output.push(t);
      }
    }

    output
  }

  pub fn to_function_infered_parameter_types(
    parameters: &Vec<Self>
  ) -> Vec<FunctionInferedParameterType> {
    parameters
      .iter()
      .map(|param| FunctionInferedParameterType {
        infered_type: param.typed_identifier.type_declaration.to_string(),
        parameter_type: param.parameter_type,
        span: param.span
      })
      .collect()
  }
}

impl Codegen for FunctionDeclarationParameter {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.parameter_type.emit(context, f)?;
    self.typed_identifier.emit(context, f)
  }
}

impl Visited for FunctionDeclarationParameter {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_function_declaration_parameter(self);
    self.typed_identifier.accept(visitor);
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ParameterType {
  Copy,
  Optional,
  Reference
}

impl Codegen for ParameterType {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      ParameterType::Copy => Ok(()),
      ParameterType::Optional => write!(f, "optional "),
      ParameterType::Reference => write!(f, "out ")
    }
  }
}
