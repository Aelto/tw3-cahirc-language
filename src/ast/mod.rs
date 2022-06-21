use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;

use self::generic_calls_register::GenericCallsRegister;

pub mod generic_calls_register;

// -----------------------------------------------------------------------------

pub struct ProgramInformation {
  pub generic_functions: RefCell<Vec<Rc<FunctionDeclaration>>>,
  pub generic_function_calls: RefCell<Vec<Rc<Expression>>>
}

impl ProgramInformation {
  pub fn new() -> Self {
    Self {
      generic_functions: RefCell::new(Vec::new()),
      generic_function_calls: RefCell::new(Vec::new())
    }
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>
}

// -----------------------------------------------------------------------------

mod statement;
pub use statement::Statement;

// -----------------------------------------------------------------------------

mod classes;
pub use classes::ClassBodyStatement;
pub use classes::EncapsulationType;
pub use classes::ClassDeclaration;
pub use classes::ClassType;

// -----------------------------------------------------------------------------

mod structs;
pub use structs::StructBodyStatement;
pub use structs::StructDeclaration;

// -----------------------------------------------------------------------------

mod functions;
pub use functions::FunctionCallParameters;
pub use functions::FunctionBodyStatement;
pub use functions::FunctionDeclaration;
pub use functions::FunctionType;

// -----------------------------------------------------------------------------

mod ifs;
pub use ifs::IfStatement;

// -----------------------------------------------------------------------------

mod variables;
pub use variables::VariableDeclarationOrAssignment;
pub use variables::VariableDeclaration;
pub use variables::VariableAssignment;

// -----------------------------------------------------------------------------

mod for_loops;
pub use for_loops::ForStatement;

// -----------------------------------------------------------------------------

mod while_loops;
pub use while_loops::DoWhileStatement;
pub use while_loops::WhileStatement;

// -----------------------------------------------------------------------------

mod identifiers;
pub use identifiers::TypedIdentifier;
pub use identifiers::TypeDeclaration;
pub use identifiers::IdentifierTerm;

// -----------------------------------------------------------------------------

mod expressions;
pub use expressions::AssignmentType;
pub use expressions::ComparisonType;
pub use expressions::OperationCode;
pub use expressions::Expression;