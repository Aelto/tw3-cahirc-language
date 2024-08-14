use std::ops::Deref;

use super::Visited;

impl<A> Visited for Vec<A>
where
  A: Visited
{
  fn accept<T: super::Visitor>(&self, visitor: &mut T) {
    for child in self {
      child.accept(visitor);
    }
  }
}

impl<A> Visited for std::boxed::Box<A>
where
  A: Visited
{
  fn accept<T: super::Visitor>(&self, visitor: &mut T) {
    self.deref().accept(visitor);
  }
}

impl<A> Visited for std::rc::Rc<A>
where
  A: Visited
{
  fn accept<T: super::Visitor>(&self, visitor: &mut T) {
    self.deref().accept(visitor);
  }
}

impl<A> Visited for Option<A>
where
  A: Visited
{
  fn accept<T: super::Visitor>(&self, visitor: &mut T) {
    if let Some(child) = self {
      child.accept(visitor);
    }
  }
}
