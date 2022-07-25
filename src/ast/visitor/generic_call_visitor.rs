use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::ProgramInformation;
use crate::ast::TypeDeclaration;

/// Looks for generic calls and register them to the GenericCallRegister
pub struct GenericCallsVisitor<'a> {
  pub program_information: &'a ProgramInformation,
  pub current_context: Rc<RefCell<Context>>,
}

impl<'a> GenericCallsVisitor<'a> {
  pub fn new(program_information: &'a ProgramInformation) -> Self {
    Self {
      program_information,
      current_context: Rc::new(RefCell::new(Context::new("empty", None))),
    }
  }
}

impl super::Visitor for GenericCallsVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::GenericCallsVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();
  }

  fn visit_generic_function_call(&mut self, node: &crate::ast::FunctionCall) {
    let function_name = node.accessor.text.to_string();
    let function_context =
      Context::find_global_function_declaration(&self.current_context, &function_name);

    if let Some(generic_types) = &node.generic_types {
      if let Some(function_context) = function_context {
        let response = function_context
          .borrow_mut()
          .register_generic_call(&generic_types);

        if response.is_some() {
          node.mangled_accessor.replace(response);
        }
      }
    }
  }

  fn visit_generic_variable_declaration(&mut self, node: &crate::ast::TypeDeclaration) {
    match node {
      TypeDeclaration::Regular {
        type_name,
        generic_type_assignment,
        mangled_accessor,
      } => {
        let class_name = &type_name;
        let class_context =
          Context::find_global_class_declaration(&self.current_context, class_name);

        if let Some(_) = &generic_type_assignment {
          if let Some(class_context) = class_context {
            let stringified_generic_types = match &generic_type_assignment {
              Some(x) => {
                let types = {
                  let mut list = Vec::new();

                  for child in x {
                    list.push(child);
                  }

                  list
                };

                TypeDeclaration::stringified_generic_types(&types, &class_context.borrow())
              }
              None => Vec::new(),
            };

            let still_contains_generic_types = match &self.current_context.borrow().generic_context
            {
              Some(gen) => gen.contains_generic_identifier(&TypeDeclaration::flat_type_names(
                &type_name,
                &generic_type_assignment,
              )),
              None => false,
            };

            if still_contains_generic_types {
              return;
            }

            let response = class_context
              .borrow_mut()
              .register_generic_call(&stringified_generic_types);

            if response.is_some() {
              mangled_accessor.replace(response);
            }
          }
        }
      }
      TypeDeclaration::Lambda(_) => todo!(),
    }
  }

  fn visit_generic_class_instantiation(&mut self, node: &crate::ast::ClassInstantiation) {
    let class_name = &node.class_name;
    let class_context = Context::find_global_class_declaration(&self.current_context, class_name);

    if let Some(_) = &node.generic_type_assignment {
      if let Some(class_context) = class_context {
        let stringified_generic_types = match &node.generic_type_assignment {
          Some(x) => {
            let types = {
              let mut list = Vec::new();

              for child in x {
                list.push(child);
              }

              list
            };

            TypeDeclaration::stringified_generic_types(&types, &class_context.borrow())
          }
          None => Vec::new(),
        };

        let still_contains_generic_types = match &self.current_context.borrow().generic_context {
          Some(gen) => gen.contains_generic_identifier(&TypeDeclaration::flat_type_names(
            &node.class_name,
            &node.generic_type_assignment,
          )),
          None => false,
        };

        if still_contains_generic_types {
          return;
        }

        let response = class_context
          .borrow_mut()
          .register_generic_call(&stringified_generic_types);

        if response.is_some() {
          // TODO: perhaps? Not sure it is needed tbh
          // node.mangled_accessor.replace(response);
        }
      }
    }
  }
}
