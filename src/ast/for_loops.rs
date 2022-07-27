use std::borrow::Borrow;
use std::rc::Rc;

use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct ForStatement {
  pub initialization: Option<VariableDeclarationOrAssignment>,
  pub condition: Rc<Expression>,
  pub iteration: VariableAssignment,
  pub body_statements: Vec<FunctionBodyStatement>,
}

impl Visited for ForStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    self.initialization.accept(visitor);
    self.condition.accept(visitor);
    self.iteration.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for ForStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(f, "for (")?;
    self.initialization.emit(context, f)?;
    write!(f, "; ")?;
    self.condition.emit(context, f)?;
    write!(f, "; ")?;
    self.iteration.emit(context, f)?;
    writeln!(f, ") {{")?;

    self.body_statements.emit_join(context, f, "\n")?;

    writeln!(f, "}}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub struct ForInStatement {
  pub child: Rc<TypedIdentifier>,
  pub parent: Rc<Expression>,

  pub body_statements: Vec<FunctionBodyStatement>,

  pub indexor_name: RefCell<String>,
}

impl Visited for ForInStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match visitor.visitor_type() {
      visitor::VisitorType::VariableDeclarationVisitor => {
        // we generate a new random intermediate indexing variable for this for loop
        let indexor = format!("idx{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        let indexor_variable = TypedIdentifier {
          names: vec![indexor.clone()],
          type_declaration: TypeDeclaration::Regular {
            type_name: "int".to_string(),
            generic_type_assignment: None,
            mangled_accessor: RefCell::new(None),
          },
        };

        visitor.register_variable_declaration(self.child.clone());
        visitor.register_variable_declaration(Rc::new(indexor_variable));
        self.indexor_name.replace(indexor);
      }
      _ => {}
    };

    self.child.accept(visitor);
    self.parent.accept(visitor);
    self.body_statements.accept(visitor);
  }
}

impl Codegen for ForInStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(
      f,
      "for ({} = 0; {} < ",
      self.indexor_name.borrow(),
      self.indexor_name.borrow(),
    )?;
    self.parent.emit(context, f)?;
    writeln!(f, ".Size(); {} += 1) {{", self.indexor_name.borrow())?;

    if let Some(variable_name) = self.child.names.first() {
      write!(f, "{} = ", variable_name)?;
      self.parent.emit(context, f)?;
      writeln!(f, "[{}];", self.indexor_name.borrow())?;
    }

    self.body_statements.emit_join(context, f, "\n")?;
    writeln!(f, "}}")?;

    Ok(())
  }
}
