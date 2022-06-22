use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::FunctionDeclaration;

type GenericFunctionName = String;

#[derive(Debug)]
pub struct GenericFunctionsRegister {
  pub functions: HashMap<GenericFunctionName, Rc<FunctionDeclaration>>,
  pub calls: HashMap<GenericFunctionName, GenericCallsRegister>,
}

impl GenericFunctionsRegister {
  pub fn new() -> Self {
    Self {
      functions: HashMap::new(),
      calls: HashMap::new(),
    }
  }

  pub fn register_new_generic_function(
    &mut self,
    function_name: &str,
    function: Rc<FunctionDeclaration>,
  ) {
    if !self.calls.contains_key(function_name) {
      self
        .calls
        .insert(function_name.to_string(), GenericCallsRegister::new());
    }

    self.functions.insert(function_name.to_string(), function);
  }

  pub fn register_call(&mut self, function_name: String, call: Vec<String>) {
    if !self.calls.contains_key(&function_name) {
      self
        .calls
        .insert(function_name.to_string(), GenericCallsRegister::new());
    }

    if let Some(call_register) = self.calls.get_mut(&function_name) {
      println!("registering generic call: {}<{:?}>", &function_name, &call);

      call_register.register_call(call);
    }
  }
}

#[derive(Debug)]
pub struct GenericCallsRegister {
  pub calls: Vec<Vec<String>>,
}

impl GenericCallsRegister {
  pub fn new() -> Self {
    Self { calls: Vec::new() }
  }

  pub fn register_call(&mut self, call: Vec<String>) {
    if self.has_call_already(&call) {
      return;
    }

    self.calls.push(call);
  }

  fn has_call_already(&self, call: &Vec<String>) -> bool {
    self
      .calls
      .iter()
      .any(|self_call| Self::are_calls_identical(self_call, call))
  }

  fn are_calls_identical(a: &Vec<String>, b: &Vec<String>) -> bool {
    !a.iter().zip(b).any(|(a, b)| a != b)
  }
}
