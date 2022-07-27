use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::visitor::Visited;

pub struct ContextBuildingVisitor {
  pub current_context: Rc<RefCell<Context>>,
}

impl super::Visitor for ContextBuildingVisitor {
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    Context::set_parent_context(&node.context, &self.current_context);

    // then make a new context building visitor for the context of the
    // FunctionDeclaration node.
    let mut new_context_visitor = Self {
      current_context: node.context.clone(),
    };

    node.body_statements.accept(&mut new_context_visitor);
  }

  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    Context::set_parent_context(&node.context, &self.current_context);

    // then make a new context building visitor for the context of the
    // ClassDeclaration node.
    let mut new_context_visitor = Self {
      current_context: node.context.clone(),
    };

    node.body_statements.accept(&mut new_context_visitor);
  }

  fn visit_struct_declaration(&mut self, node: &crate::ast::StructDeclaration) {
    Context::set_parent_context(&node.context, &self.current_context);

    // then make a new context building visitor for the context of the
    // StructDeclaration node.
    let mut new_context_visitor = Self {
      current_context: node.context.clone(),
    };

    node.body_statements.accept(&mut new_context_visitor);
  }

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::ContextBuildingVisitor
  }
}
