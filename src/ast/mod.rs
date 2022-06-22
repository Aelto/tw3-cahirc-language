use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;

use self::generic_calls_register::GenericFunctionsRegister;

pub mod generic_calls_register;
pub mod codegen;
pub mod visitor;

// -----------------------------------------------------------------------------

pub struct ProgramInformation {
  pub generic_functions_register: RefCell<GenericFunctionsRegister>
}

impl ProgramInformation {
  pub fn new() -> Self {
    Self {
      generic_functions_register: RefCell::new(GenericFunctionsRegister::new())
    }
  }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>
}

impl visitor::Visited for Program {
    fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
        for statement in &self.statements {
          statement.accept(visitor);
        }
    }
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