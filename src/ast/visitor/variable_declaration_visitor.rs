use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::codegen::context::ContextType;
use crate::ast::ProgramInformation;
use crate::ast::TypedIdentifier;

/// Looks variable declarations and register them to the context of the current
/// function. Allows for variable declarations anywhere in function bodies.
pub struct VariableDeclarationVisitor<'a> {
  pub program_information: &'a ProgramInformation,
  pub current_context: Rc<RefCell<Context>>,
}

impl<'a> VariableDeclarationVisitor<'a> {
  pub fn new(program_information: &'a ProgramInformation) -> Self {
    Self {
      program_information,
      current_context: Rc::new(RefCell::new(Context::new(
        "empty",
        None,
        ContextType::Global,
      ))),
    }
  }
}

impl super::Visitor for VariableDeclarationVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::VariableDeclarationVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    self.current_context = node.context.clone();
  }

  fn visit_variable_declaration(&mut self, node: &crate::ast::VariableDeclaration) {
    match &node {
      crate::ast::VariableDeclaration::Explicit {
        declaration,
        following_expression: _,
      } => {
        self.register_variable_declaration(declaration.clone());
      }
      // implicit variables are registered by the type inference visitor
      crate::ast::VariableDeclaration::Implicit {
        names: _,
        following_expression: _,
      } => {}
    };
  }

  fn register_variable_declaration(&mut self, declaration: Rc<TypedIdentifier>) {
    for variable_name in &declaration.names {
      self
        .current_context
        .borrow_mut()
        .local_variables_inference
        .insert(
          variable_name.clone(),
          declaration.type_declaration.to_string(),
        );
    }

    self
      .current_context
      .borrow_mut()
      .variable_declarations
      .push(declaration);
  }
}
