use visitor::Visited;

use super::*;

#[derive(Debug)]
pub enum Annotation {
  ReplaceMethod {
    target_parent: Option<String>,
    encapsulation: Option<EncapsulationType>,
    function: Rc<FunctionDeclaration>
  },
  WrapMethod {
    target_parent: String,
    encapsulation: Option<EncapsulationType>,
    function: Rc<FunctionDeclaration>
  },
  AddMethod {
    target_parent: String,
    encapsulation: Option<EncapsulationType>,
    function: Rc<FunctionDeclaration>
  },
  AddField {
    target_parent: String,
    declaration: VariableDeclaration
  }
}

impl Visited for Annotation {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      Self::AddField {
        target_parent: _,
        declaration
      } => {
        declaration.accept(visitor);
      }
      Self::ReplaceMethod {
        target_parent: _,
        encapsulation: _,
        function
      } => {
        function.accept(visitor);
      }
      Self::WrapMethod {
        target_parent: _,
        encapsulation: _,
        function
      } => function.accept(visitor),
      Self::AddMethod {
        target_parent: _,
        encapsulation: _,
        function
      } => function.accept(visitor)
    }
  }
}

impl Codegen for Annotation {
  fn emit(
    &self, context: &codegen::context::Context, f: &mut Vec<u8>
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      Annotation::ReplaceMethod {
        target_parent,
        encapsulation,
        function
      } => {
        write!(f, "@replaceMethod(")?;
        if let Some(p) = target_parent {
          write!(f, "{p}")?;
        }
        writeln!(f, ")")?;
        if let Some(e) = encapsulation {
          e.emit(context, f)?;
          write!(f, " ")?;
        }
        function.emit(context, f)?;
      }
      Annotation::WrapMethod {
        target_parent,
        encapsulation,
        function
      } => {
        writeln!(f, "@wrapMethod({target_parent})")?;
        if let Some(e) = encapsulation {
          e.emit(context, f)?;
          write!(f, " ")?;
        }
        function.emit(context, f)?;
      }
      Annotation::AddMethod {
        target_parent,
        encapsulation,
        function
      } => {
        writeln!(f, "@addMethod({target_parent})")?;
        if let Some(e) = encapsulation {
          e.emit(context, f)?;
          write!(f, " ")?;
        }
        function.emit(context, f)?;
      }
      Annotation::AddField {
        target_parent,
        declaration
      } => {
        writeln!(f, "@addField({target_parent})")?;
        declaration.emit(context, f)?;
      }
    }

    Ok(())
  }

  fn emit_join(
    &self, _: &codegen::context::Context, _: &mut Vec<u8>, _: &'static str
  ) -> Result<(), std::io::Error> {
    unimplemented!("default emit_join impl called");
  }
}
