use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;

use crate::ast::Expression;
use crate::ast::ReportManager;
use crate::ast::SpanManager;
use crate::ast::TypeDeclaration;
use crate::ast::TypedIdentifier;
use crate::ast::codegen::context::Context;
use crate::ast::codegen::context::ContextType;
use crate::ast::codegen::type_inference::TypeInferenceStore;
use crate::ast::inference::ToType;

/// Registers all the compound types from the program
pub struct CompoundTypesVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> CompoundTypesVisitor<'a> {
  pub fn new(current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
  report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for CompoundTypesVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
  }

  /// Update the current context with the latest context met in the AST
  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    let result = self.inference_store.register_compound(node.name.clone());

    if let Err(reason) = result {
      let span = node.span_name;

      self.report_manager.push(
        Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
        .with_message(&"Invalid class definition")
        .with_label(
          Label::new(self.span_manager.get_range(span))
          .with_message(reason)
        )
        .finish()
      );
    }

    self.current_context = node.context.clone();
  }

  /// Update the current context with the latest context met in the AST
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    self.current_context = node.context.clone();

    let parent_context = &Context::get_ref(&node.context).parent_context;
    let parent_context_type = if let Some(parent_context) = parent_context {
      Context::get_ref(&parent_context).context_type.clone()
    }
    else {
      ContextType::Global
    };

    let parameters: Vec<String> = node.parameters
      .iter()
      .map(|param| param.typed_identifier.type_declaration.to_string())
      .collect();

    // we try to see if the function is inside a struct or class or if
    // it is a global function.
    match parent_context_type {
      ContextType::ClassOrStruct | ContextType::State { parent_class_name: _ } => {
        if let Some(parent_context) = parent_context {
          let parent_context = Context::get_ref(&parent_context);
          let compound_parent_name = parent_context.get_class_name()
            .expect("could not get the name of the parent compound type while analysing a method definition");
  
          let result = self.inference_store.register_method(
            compound_parent_name, node.name.clone(),
            parameters,
            match &node.type_declaration {
              Some(decl) => Some(decl.to_string()),
              None => None,
            }
          );

          if let Err(reason) = result {
            let span = node.span_name;

            self.report_manager.push(
              Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
              .with_message(&"Invalid method definition")
              .with_label(
                Label::new(self.span_manager.get_range(span))
                .with_message(reason)
              )
              .finish()
            );
          }
        }
      },
      _ => {
        let result = self.inference_store.register_function(
          node.name.clone(),
          parameters,
          match &node.type_declaration {
            Some(decl) => Some(decl.to_string()),
            None => None,
          }
        );

        if let Err(reason) = result {
          let span = node.span_name;

          self.report_manager.push(
            Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
            .with_message(&"Invalid function definition")
            .with_label(
              Label::new(self.span_manager.get_range(span))
              .with_message(reason)
            )
            .finish()
          );
        }
      },
    };
  }

  /// Update the current context with the latest context met in the AST
  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    let result = self.inference_store.register_compound(node.name.clone());

    if let Err(reason) = result {
      let span = node.span_name;

      self.report_manager.push(
        Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
        .with_message(&"Invalid struct definition")
        .with_label(
          Label::new(self.span_manager.get_range(span))
          .with_message(reason)
        )
        .finish()
      );
    }

    self.current_context = node.context.clone();
  }
}

/// Does type inference for the local variables in the functions
pub struct FunctionsInferenceVisitor<'a> {
  pub current_context: Rc<RefCell<Context>>,
  pub inference_store: &'a mut TypeInferenceStore,
  pub report_manager: &'a mut ReportManager,
  pub span_manager: &'a mut SpanManager
}

impl<'a> FunctionsInferenceVisitor<'a> {
  pub fn new(current_context: Rc<RefCell<Context>>, inference_store: &'a mut TypeInferenceStore,
  report_manager: &'a mut ReportManager, span_manager: &'a mut SpanManager) -> Self {
    Self {
      current_context,
      inference_store,
      report_manager,
      span_manager
    }
  }
}

impl super::Visitor for FunctionsInferenceVisitor<'_> {
  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::TypeInferenceVisitor
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
        crate::ast::VariableDeclaration::Explicit { declaration, following_expression: _ } => {
          for variable_name in &declaration.names {
            let type_declaration_string = declaration.type_declaration.to_string();
      
            println!("registering local variable {variable_name}: {type_declaration_string}");
      
            self.current_context.borrow_mut().local_variables_inference.insert(
              variable_name.clone(),
              type_declaration_string
            );
          }
        },
        crate::ast::VariableDeclaration::Implicit { names, following_expression } => {
          let expression: &Expression = &following_expression.borrow();
          let the_type = match expression.resulting_type(&self.current_context, &self.inference_store.types, &self.span_manager) {
            Ok(t) => t,
            Err(reports) => {
              self.report_manager.push_many(reports);

              return;
            }
          };

          match the_type {
            crate::ast::inference::Type::Void => {
              let span = following_expression.get_span();

              self.report_manager.push(
                Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
                .with_message(&"Cannot infer variable type")
                .with_label(
                  Label::new(self.span_manager.get_range(span))
                  .with_message(&"Implicit variable declaration but resulting type is void")
                )
                .finish()
              );

              return;
            },
            crate::ast::inference::Type::Unknown => {
              let span = following_expression.get_span();

              self.report_manager.push(
                Report::build(ReportKind::Error, (), self.span_manager.get_left(span))
                .with_message(&"Cannot infer variable type")
                .with_label(
                  Label::new(self.span_manager.get_range(span))
                  .with_message(&"Implicit variable declaration but resulting type is unknown at the time")
                )
                .with_help(&"Prefer an explicit type annotation here")
                .finish()
              );

              return;
            },
            _ => {}
          };

          let the_type = the_type.to_string();

          for name in names {
            self.current_context.borrow_mut().local_variables_inference.insert(
              name.clone(),
              the_type.clone()
            );
          }

          self.register_variable_declaration(Rc::new(TypedIdentifier {
            names: names.clone(),
            type_declaration: TypeDeclaration::Regular {
              type_name: the_type,
              generic_type_assignment: None,
              mangled_accessor: RefCell::new(None)
            }
          }));
        },
    };
  }

  fn register_variable_declaration(&mut self, declaration: Rc<TypedIdentifier>) {
    self
      .current_context
      .borrow_mut()
      .variable_declarations
      .push(declaration);
  }
}
