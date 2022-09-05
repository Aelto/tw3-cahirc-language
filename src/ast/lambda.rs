use std::borrow::Borrow;
use std::collections::HashSet;

use crate::ast::codegen::context::GenericContext;

use super::inference::Type;
use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct LambdaDeclaration {
  pub parameters: Vec<FunctionDeclarationParameter>,
  pub type_declaration: Option<Rc<TypeDeclaration>>,
}

impl LambdaDeclaration {
  /// Returns the stringified representation for the given lambda.
  ///
  /// A type representation is meant to act as a unique identifier.
  /// A way to differentiate different lambdas if their definitions
  /// differ.
  pub fn stringified_type_representation<'a>(
    parameters: &'a Vec<FunctionDeclarationParameter>, return_type: &'a Option<&'a String>,
  ) -> String {
    let flattened_types = FunctionDeclarationParameter::flat_type_names(&parameters).join("_");
    let void = String::from("_void");
    let return_type_suffix = return_type.unwrap_or_else(|| &void);

    format!("lambda__{flattened_types}_rt__{return_type_suffix}")
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

    let return_type_suffix = if let Some(returntype) = &self.type_declaration {
      let returntype: &TypeDeclaration = &returntype.borrow();
      GenericContext::generic_variant_suffix_from_types(
        &TypeDeclaration::stringified_generic_types(&vec![returntype], &context),
      )
    } else {
      String::from("_void")
    };

    write!(f, "lambda_{generic_variant_suffix}_rt_{return_type_suffix}")?;

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
  pub captured_variables: RefCell<Vec<(String, Type)>>,
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
  let return_type =
    FunctionBodyStatement::get_return_type_from_last_statement(&this.body_statements);

  let return_type_suffix = if let Some(returntype) = return_type {
    returntype
  } else {
    "void"
  };

  if let Some(mangled_suffix) = this.mangled_accessor.borrow().as_ref() {
    let lambda_type_name = format!("lambda_{mangled_suffix}");
    writeln!(
      f,
      "class {lambda_type_name} extends lambda_{this_generic_variant_suffix}_rt__{return_type_suffix} {{"
    )?;
    write!(f, "function call(")?;
    this.parameters.emit_join(context, f, ", ")?;
    write!(f, ")")?;

    if let Some(returntype) = &return_type {
      write!(f, ": {returntype}")?;
    }

    // before emitting the body of the lambda/closure, make sure to replace all
    // occurences of "this" with the new generated identifier:
    context
      .replace_this_with_self
      .replace(Some(uuid::Uuid::new_v4().to_string().replace("-", "")));

    writeln!(f, " {{")?;
    match this.lambda_type {
      LambdaType::SingleLine => {
        write!(f, "return ")?;
        this.body_statements.emit(context, f)?;
      }
      LambdaType::MultiLine => this.body_statements.emit(context, f)?,
    };
    writeln!(f, "}}")?;

    {
      // the capture method
      let captured_variables = this.captured_variables.borrow();
      let captured_variables: &Vec<(String, Type)> = &captured_variables.as_ref();

      // emit the properties:
      // unwrap here since we know it exists as it was created above.
      let replacer = context.replace_this_with_self.borrow();
      let replacer = replacer.as_ref().unwrap();
      for var in captured_variables {
        let var_name = if &var.0 == "this" { &replacer } else { &var.0 };
        let var_type = &var.1;
        writeln!(f, "var {var_name}: {var_type};")?;
      }

      write!(f, "function capture(")?;

      // emit the parameters:
      for var in captured_variables {
        let var_name = if &var.0 == "this" { &replacer } else { &var.0 };
        let var_type = &var.1;
        write!(f, "out {var_name}: {var_type},")?;
      }

      writeln!(f, "): {lambda_type_name} {{")?;

      // emit the variable assignments:
      for var in captured_variables {
        let var_name = if &var.0 == "this" { &replacer } else { &var.0 };
        writeln!(f, "this.{var_name} = {var_name};")?;
      }

      writeln!(f, "return this;")?;
      writeln!(f, "}}")?;
    }

    context.replace_this_with_self.replace(None);

    writeln!(f, "}}")?;
  }

  Ok(())
}

impl Visited for Lambda {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_lambda(self);
    self.parameters.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for Lambda {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let suffix = format!("wss{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    write!(f, "(new lambda_{suffix} in thePlayer).capture(")?;

    let captures = self.captured_variables.borrow();
    let captures: &Vec<(String, Type)> = &captures.as_ref();
    for captured_variable in captures {
      write!(f, "{}, ", captured_variable.0)?;
    }
    write!(f, ")")?;

    self.mangled_accessor.replace(Some(suffix));

    Ok(())
  }
}
