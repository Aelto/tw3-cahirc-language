use std::ops::Deref;

pub mod context;

pub trait Codegen {
  fn emit(&self, context: &mut context::Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(output, "##default Codegen::emit impl##")?;

    Ok(())
  }
}

impl<A> Codegen for Vec<A>
where
  A: Codegen,
{
  fn emit(&self, context: &mut context::Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    for child in self {
      child.emit(context, output)?;
    }

    Ok(())
  }
}

impl<A> Codegen for std::boxed::Box<A>
where
  A: Codegen,
{
  fn emit(&self, context: &mut context::Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.deref().emit(context, output)
  }
}

impl<A> Codegen for std::rc::Rc<A>
where
  A: Codegen,
{
  fn emit(&self, context: &mut context::Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    self.deref().emit(context, output)
  }
}

impl<A> Codegen for Option<A>
where
  A: Codegen,
{
  fn emit(&self, context: &mut context::Context, output: &mut Vec<u8>) -> Result<(), std::io::Error> {
    if let Some(inner) = self {
      inner.emit(context, output)?;
    }

    Ok(())
  }
}
