
/// A list of the types used for a generic call.
type GenericCall = Vec<String>;

#[derive(Debug)]
pub struct GenericCallsRegister {
  pub calls: Vec<GenericCall>,
}

impl GenericCallsRegister {
  pub fn new() -> Self {
    Self {
      calls: Vec::new()
    }
  }

  pub fn register_call(&mut self,call: GenericCall) {
    if self.has_call_already(&call) {
      return;
    }

    self.calls.push(call);
  }

  fn has_call_already(&self, call: &GenericCall) -> bool {
    self.calls.iter()
      .any(|self_call| Self::are_calls_identical(self_call, call))
  }

  fn are_calls_identical(a: &GenericCall, b: &GenericCall) -> bool {
    !a.iter().zip(b).any(|(a, b)| a != b) 
  }
}