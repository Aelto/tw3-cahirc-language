use std::ops::Deref;

pub trait Codegen {
  fn emit(&self, output: &mut Vec<u8>) {
    use std::io::Write as IoWrite;

    write!(output, "##default Codegen::emit impl##").unwrap();
  }
}

impl<A> Codegen for Vec<A>
where
  A: Codegen,
{
  fn emit(&self, output: &mut Vec<u8>) {
    for child in self {
      child.emit(output);
    }
  }
}

impl<A> Codegen for std::boxed::Box<A>
where
  A: Codegen,
{
  fn emit(&self, output: &mut Vec<u8>) {
    self.deref().emit(output);
  }
}

impl<A> Codegen for std::rc::Rc<A>
where
  A: Codegen,
{
  fn emit(&self, output: &mut Vec<u8>) {
    self.deref().emit(output);
  }
}

impl<A> Codegen for Option<A>
where
  A: Codegen,
{
  fn emit(&self, output: &mut Vec<u8>) {
    if let Some(inner) = self {
      inner.emit(output);
    }
  }
}
