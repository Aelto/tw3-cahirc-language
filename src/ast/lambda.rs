use std::borrow::Borrow;
use std::collections::HashSet;

use crate::ast::codegen::context::GenericContext;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct LambdaDeclaration {
  pub parameters: Vec<FunctionDeclarationParameter>,
  pub type_declaration: Option<Rc<TypeDeclaration>>,
}

impl LambdaDeclaration {
  pub fn flat_type_names<'a>(&'a self) -> Vec<&'a str> {
    let mut output = vec![];

    for param in &self.parameters {
      let subtypes = match &param.typed_identifier.type_declaration {
        TypeDeclaration::Regular {
          type_name,
          generic_type_assignment,
          mangled_accessor: _,
        } => TypeDeclaration::flat_type_names(&type_name, &generic_type_assignment),
        TypeDeclaration::Lambda(x) => x.flat_type_names(),
      };

      for t in subtypes {
        output.push(t);
      }
    }

    output
  }

  /// emits the base abstract class the lambdas will extend to finally implement
  /// the run method.
  pub fn emit_base_type(
    &self, context: &mut Context, f: &mut Vec<u8>, emitted_types: &mut HashSet<String>,
  ) -> Result<(), std::io::Error> {
    let has_generic_context = context.generic_context.is_some();
    if has_generic_context {
      let mut variants = Vec::new();

      if let Some(generic_context) = &context.generic_context {
        for variant in generic_context.translation_variants.keys() {
          variants.push(String::from(variant));
        }
      }

      for variant in variants {
        {
          if let Some(generic_context) = &mut context.generic_context {
            generic_context.currently_used_variant = Some(variant.clone());
          }
        }

        emit_lambda_declaration(self, &context, f, &variant, emitted_types)?;
      }
    } else {
      emit_lambda_declaration(self, &context, f, "", emitted_types)?;
    }

    Ok(())
  }
}

fn emit_lambda_declaration(
  this: &LambdaDeclaration, context: &Context, f: &mut Vec<u8>, _generic_variant_suffix: &str,
  emitted_types: &mut HashSet<String>,
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  let parameter_types = {
    let mut list = Vec::new();

    for child in &this.parameters {
      list.push(&child.typed_identifier.type_declaration);
    }

    list
  };

  // we generate a more complete generic variant suffix that includes all types
  // and not just the generic ones.
  let mut this_generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
    &TypeDeclaration::stringified_generic_types(&parameter_types, &context),
  );

  let return_type_suffix = if let Some(returntype) = &this.type_declaration {
    let returntype: &TypeDeclaration = &returntype.borrow();
    GenericContext::generic_variant_suffix_from_types(&TypeDeclaration::stringified_generic_types(
      &vec![returntype],
      &context,
    ))
  } else {
    String::from("_void")
  };

  this_generic_variant_suffix.push_str("_rt_");
  this_generic_variant_suffix.push_str(&return_type_suffix);

  // a similar type was already emitted
  if emitted_types.contains(&this_generic_variant_suffix) {
    return Ok(());
  }

  writeln!(f, "abstract class lambda_{this_generic_variant_suffix} {{")?;
  write!(f, "  function call(")?;
  this.parameters.emit_join(context, f, ", ")?;
  write!(f, ")")?;

  if let Some(returntype) = &this.type_declaration {
    write!(f, ": ")?;
    returntype.emit(context, f)?;
  }

  writeln!(f, " {{}}\n}}")?;

  emitted_types.insert(this_generic_variant_suffix);

  Ok(())
}

impl Visited for LambdaDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_lambda_declaration(self);

    self.parameters.accept(visitor);
    self.type_declaration.accept(visitor);
  }
}

impl Codegen for LambdaDeclaration {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let parameter_types = {
      let mut list = Vec::new();

      for child in &self.parameters {
        list.push(&child.typed_identifier.type_declaration);
      }

      list
    };

    let generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
      &TypeDeclaration::stringified_generic_types(&parameter_types, &context),
    );

    write!(f, "lambda_{generic_variant_suffix}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub struct Lambda {
  pub lambda_type: LambdaType,
  pub parameters: Vec<FunctionDeclarationParameter>,
  pub body_statements: Vec<FunctionBodyStatement>,
  pub span: Span,

  pub mangled_accessor: RefCell<Option<String>>,
}

#[derive(Debug)]
pub enum LambdaType {
  SingleLine,
  MultiLine,
}

impl Lambda {
  /// emits the base abstract class the lambdas will extend to finally implement
  /// the run method.
  pub fn emit_base_type(
    &self, context: &mut Context, f: &mut Vec<u8>,
  ) -> Result<(), std::io::Error> {
    let has_generic_context = context.generic_context.is_some();
    if has_generic_context {
      let mut variants = Vec::new();

      if let Some(generic_context) = &context.generic_context {
        for variant in generic_context.translation_variants.keys() {
          variants.push(String::from(variant));
        }
      }

      for variant in variants {
        {
          if let Some(generic_context) = &mut context.generic_context {
            generic_context.currently_used_variant = Some(variant.clone());
          }
        }

        emit_lambda(self, &context, f, &variant)?;
      }
    } else {
      emit_lambda(self, &context, f, "")?;
    }

    Ok(())
  }
}

fn emit_lambda(
  this: &Lambda, context: &Context, f: &mut Vec<u8>, _generic_variant_suffix: &str,
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  let parameter_types = {
    let mut list = Vec::new();

    for child in &this.parameters {
      list.push(&child.typed_identifier.type_declaration);
    }

    list
  };

  // we generate a more complete generic variant suffix that includes all types
  // and not just the generic ones.
  let this_generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
    &TypeDeclaration::stringified_generic_types(&parameter_types, &context),
  );

  // get the return type from the last body statement, if it is a type cast then
  // we use it as the return type, otherwise it defaults to void (None).
  let return_type = match this.body_statements.last().unwrap() {
    FunctionBodyStatement::Expression(expression) => match expression.borrow() {
      Expression::Cast(cast_type, _) => Some(cast_type),
      _ => None,
    },
    FunctionBodyStatement::Return(x) => match x {
      Some(expression) => match expression.borrow() {
        Expression::Cast(cast_type, _) => Some(cast_type),
        _ => None,
      },
      None => None,
    },

    _ => None,
  };

  if let Some(mangled_suffix) = this.mangled_accessor.borrow().as_ref() {
    writeln!(
      f,
      "class lambda_{mangled_suffix} extends lambda_{this_generic_variant_suffix} {{"
    )?;
    write!(f, "function call(")?;
    this.parameters.emit_join(context, f, ", ")?;
    write!(f, ")")?;

    if let Some(returntype) = &return_type {
      write!(f, ": {returntype}")?;
    }

    writeln!(f, " {{")?;
    match this.lambda_type {
      LambdaType::SingleLine => {
        write!(f, "return ")?;
        this.body_statements.emit(context, f)?;
      }
      LambdaType::MultiLine => this.body_statements.emit(context, f)?,
    };
    writeln!(f, "}}")?;
    writeln!(f, "}}")?;
  }

  Ok(())
}

impl Visited for Lambda {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_lambda(self);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for Lambda {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let suffix = format!("wss{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    write!(f, "new lambda_{suffix} in thePlayer")?;

    self.mangled_accessor.replace(Some(suffix));

    Ok(())
  }
}
