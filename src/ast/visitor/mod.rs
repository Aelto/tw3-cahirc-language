use super::*;

mod function_visitor;
pub use function_visitor::FunctionVisitor;

mod generic_call_visitor;
pub use generic_call_visitor::GenericCallsVisitor;

mod context_building_visitor;
pub use context_building_visitor::ContextBuildingVisitor;

mod library_emitter_visitor;
pub use library_emitter_visitor::LibraryEmitterVisitor;

mod variable_declaration_visitor;
pub use variable_declaration_visitor::VariableDeclarationVisitor;

mod lambda_declaration_visitor;
pub use lambda_declaration_visitor::LambdaDeclarationVisitor;

mod type_inference_visitor;
pub use type_inference_visitor::CompoundTypesVisitor;
pub use type_inference_visitor::FunctionsInferenceVisitor;

pub mod implementations;

pub trait Visitor {
  fn visit_function_declaration(&mut self, _: &FunctionDeclaration) {}
  fn visit_class_declaration(&mut self, _: &ClassDeclaration) {}
  fn visit_struct_declaration(&mut self, _: &StructDeclaration) {}
  fn visit_generic_function_call(&mut self, _: &FunctionCall) {}
  fn visit_function_call(&mut self, _: &FunctionCall) {}
  fn visit_generic_variable_declaration(&mut self, _: &TypeDeclaration) {}
  fn visit_variable_declaration(&mut self, _: &VariableDeclaration) {}
  fn visit_generic_class_instantiation(&mut self, _: &ClassInstantiation) {}
  fn visit_lambda_declaration(&mut self, _: &LambdaDeclaration) {}
  fn visit_lambda(&mut self, _: &Lambda) {}
  fn register_variable_declaration(&mut self, _: Rc<TypedIdentifier>) {}

  fn visitor_type(&self) -> VisitorType;
}

pub trait Visited {
  fn accept<T: Visitor>(&self, visitor: &mut T);
}

pub enum VisitorType {
  FunctionDeclarationVisitor,
  GenericCallsVisitor,
  ContextBuildingVisitor,
  LibraryEmitterVisitor,
  VariableDeclarationVisitor,
  LambdaDeclarationVisitor,
  TypeInferenceVisitor
}
