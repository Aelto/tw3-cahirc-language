use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::codegen::Codegen;

/// Traverse the AST of a library and emit only the code that is coming from
/// generic types with generic calls.
pub struct LibraryEmitterVisitor {
  pub current_context: Rc<RefCell<Context>>,
  pub emitted_code: Vec<u8>,
}

impl LibraryEmitterVisitor {
  pub fn new(context: &Rc<RefCell<Context>>) -> Self {
    Self {
      current_context: context.clone(),
      emitted_code: Vec::new(),
    }
  }
}

impl super::Visitor for LibraryEmitterVisitor {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::LibraryEmitterVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    let has_generic_context = node.context.borrow().generic_context.is_some();

    println!("\n\n\n\n\n######");

    if !has_generic_context {
      return;
    }

    println!("\n\n\n\n\n######");

    if let Err(err) = node.emit(&self.current_context.borrow(), &mut self.emitted_code) {
      println!(
        "Error while emitting code for {}: {}",
        self.current_context.borrow().name,
        err
      );
    }

    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    let has_generic_context = node.context.borrow().generic_context.is_some();

    if !has_generic_context {
      return;
    }

    if let Err(err) = node.emit(&self.current_context.borrow(), &mut self.emitted_code) {
      println!(
        "Error while emitting code for {}: {}",
        self.current_context.borrow().name,
        err
      );
    }

    self.current_context = node.context.clone();
  }
}
