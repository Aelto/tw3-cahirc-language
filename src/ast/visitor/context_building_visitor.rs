use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::codegen::context::Context;
use crate::ast::visitor::Visited;

pub struct ContextBuildingVisitor {
  pub current_context: Rc<RefCell<Context>>,
}

impl super::Visitor for ContextBuildingVisitor {
  fn visit_function_declaration(&mut self, node: &crate::ast::FunctionDeclaration) {
    println!("ContextBuildingVisitor, function: {:?}", node.name);

    Context::set_parent_context(&node.context, &self.current_context);

    // then make a new context building visitor for the context of the
    // FunctionDelcaration node.
    let mut new_context_visitor = Self {
      current_context: node.context.clone(),
    };

    node.body_statements.accept(&mut new_context_visitor);
  }

  fn visit_class_declaration(&mut self, node: &crate::ast::ClassDeclaration) {
    println!("ContextBuildingVisitor, class: {:?}", node.name);

    Context::set_parent_context(&node.context, &self.current_context);

    // then make a new context building visitor for the context of the
    // FunctionDelcaration node.
    let mut new_context_visitor = Self {
      current_context: node.context.clone(),
    };

    node.body_statements.accept(&mut new_context_visitor);
  }

  fn visitor_type(&self) -> super::VisitorType {
    super::VisitorType::ContextBuildingVisitor
  }
}
