use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use self::codegen::Codegen;

pub mod codegen;
pub mod inference;
pub mod report_manager;
pub mod span_manager;
pub mod visitor;

pub use codegen::context::{Context, ContextType};

pub use report_manager::*;
pub use span_manager::*;

// -----------------------------------------------------------------------------

pub struct ProgramInformation {}

impl ProgramInformation {
  pub fn new() -> Self {
    Self {}
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>
}

impl visitor::Visited for Program {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.statements.accept(visitor);
  }
}

impl Codegen for Program {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    for statement in &self.statements {
      statement.emit(context, f)?;
      writeln!(f, "")?;
      writeln!(f, "")?;
    }

    Ok(())
  }
}

// -----------------------------------------------------------------------------

mod statement;
pub use statement::Statement;

// -----------------------------------------------------------------------------

mod classes;
pub use classes::{ClassBodyStatement, ClassDeclaration, ClassType, EncapsulationType};

// -----------------------------------------------------------------------------

mod class_instantiation;
pub use class_instantiation::ClassInstantiation;

// -----------------------------------------------------------------------------

mod structs;
pub use structs::{StructBodyStatement, StructDeclaration};

// -----------------------------------------------------------------------------

mod enums;
pub use enums::{EnumBodyStatement, EnumDeclaration};

// -----------------------------------------------------------------------------

mod annotations;
pub use annotations::Annotation;

// -----------------------------------------------------------------------------

mod functions;
pub use functions::{
  FunctionBodyStatement, FunctionCallParameters, FunctionDeclaration, FunctionDeclarationParameter,
  FunctionType, ParameterType
};

// -----------------------------------------------------------------------------

mod ifs;
pub use ifs::IfStatement;

// -----------------------------------------------------------------------------

mod variables;
pub use variables::{VariableAssignment, VariableDeclaration, VariableDeclarationOrAssignment};

// -----------------------------------------------------------------------------

mod for_loops;
pub use for_loops::{ForInStatement, ForStatement};

// -----------------------------------------------------------------------------

mod while_loops;
pub use while_loops::{DoWhileStatement, WhileStatement};

// -----------------------------------------------------------------------------

mod switch_case;
pub use switch_case::{SwitchCaseStatement, SwitchStatement};

// -----------------------------------------------------------------------------

mod identifiers;
pub use identifiers::{IdentifierTerm, TypeDeclaration, TypedIdentifier};

// -----------------------------------------------------------------------------

mod expressions;
pub use expressions::*;

// -----------------------------------------------------------------------------

mod function_call;
pub use function_call::FunctionCall;

// -----------------------------------------------------------------------------

mod lambda;
pub use lambda::{Lambda, LambdaDeclaration, LambdaType};
