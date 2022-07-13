use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use self::codegen::Codegen;
use self::generic_calls_register::GenericFunctionsRegister;

pub mod codegen;
pub mod generic_calls_register;
pub mod visitor;

pub use codegen::context::Context;

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
  pub statements: Vec<Statement>,
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
pub use classes::ClassBodyStatement;
pub use classes::ClassDeclaration;
pub use classes::ClassType;
pub use classes::EncapsulationType;

// -----------------------------------------------------------------------------

mod structs;
pub use structs::StructBodyStatement;
pub use structs::StructDeclaration;

// -----------------------------------------------------------------------------

mod enums;
pub use enums::EnumBodyStatement;
pub use enums::EnumDeclaration;

// -----------------------------------------------------------------------------

mod functions;
pub use functions::FunctionBodyStatement;
pub use functions::FunctionCallParameters;
pub use functions::FunctionDeclaration;
pub use functions::FunctionType;

// -----------------------------------------------------------------------------

mod ifs;
pub use ifs::IfStatement;

// -----------------------------------------------------------------------------

mod variables;
pub use variables::VariableAssignment;
pub use variables::VariableDeclaration;
pub use variables::VariableDeclarationOrAssignment;

// -----------------------------------------------------------------------------

mod for_loops;
pub use for_loops::ForStatement;

// -----------------------------------------------------------------------------

mod while_loops;
pub use while_loops::DoWhileStatement;
pub use while_loops::WhileStatement;

// -----------------------------------------------------------------------------

mod identifiers;
pub use identifiers::IdentifierTerm;
pub use identifiers::TypeDeclaration;
pub use identifiers::TypedIdentifier;

// -----------------------------------------------------------------------------

mod expressions;
pub use expressions::AssignmentType;
pub use expressions::ComparisonType;
pub use expressions::Expression;
pub use expressions::OperationCode;

// -----------------------------------------------------------------------------

mod function_call;
pub use function_call::FunctionCall;
