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

    println!("flat_type_names");

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

        emit_lambda_declaration(self, &context, f, &variant)?;
      }
    } else {
      emit_lambda_declaration(self, &context, f, "")?;
    }

    Ok(())
  }
}

fn emit_lambda_declaration(
  this: &LambdaDeclaration, context: &Context, f: &mut Vec<u8>, generic_variant_suffix: &str,
) -> Result<(), std::io::Error> {
  use std::io::Write as IoWrite;

  let parameter_types = {
    let mut list = Vec::new();

    for child in &this.parameters {
      list.push(&child.typed_identifier.type_declaration);
    }

    list
  };

  let this_generic_variant_suffix = GenericContext::generic_variant_suffix_from_types(
    &TypeDeclaration::stringified_generic_types(&parameter_types, &context),
  );

  dbg!(&generic_variant_suffix);
  dbg!(&this_generic_variant_suffix);

  writeln!(f, "abstract class lambda_{this_generic_variant_suffix} {{")?;
  write!(f, "  function run(")?;
  this.parameters.emit_join(context, f, ", ")?;
  writeln!(f, ") {{}}")?;
  writeln!(f, "}}")?;

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
pub enum StructBodyStatement {
  #[allow(dead_code)]
  Property(VariableDeclaration),
  #[allow(dead_code)]
  DefaultValue(VariableAssignment),
}

impl Visited for StructBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      StructBodyStatement::Property(x) => x.accept(visitor),
      StructBodyStatement::DefaultValue(x) => x.accept(visitor),
    }
  }
}

impl Codegen for StructBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      StructBodyStatement::Property(x) => {
        x.emit(context, f)?;
        write!(f, ";")?;
      }
      StructBodyStatement::DefaultValue(x) => {
        write!(f, "default ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
    };

    Ok(())
  }
}
