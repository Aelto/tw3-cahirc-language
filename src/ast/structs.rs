use super::visitor::Visited;
use super::*;

#[derive(Debug)]
pub struct StructDeclaration {
  pub name: String,
  pub body_statements: Vec<StructBodyStatement>,

  pub span_name: Span,

  pub context: Rc<RefCell<Context>>
}

impl Visited for StructDeclaration {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    visitor.visit_struct_declaration(&self);

    // don't go further, the context building visitor will create a new one
    // and continue traversing using the new one.
    match visitor.visitor_type() {
      visitor::VisitorType::ContextBuildingVisitor => return,
      _ => {}
    };

    self.body_statements.accept(visitor);
  }
}

impl Codegen for StructDeclaration {
  fn emit(&self, _: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    writeln!(f, "struct {} {{", self.name)?;

    for statement in &self.body_statements {
      statement.emit(&self.context.borrow(), f)?;
      writeln!(f, "")?;
    }

    writeln!(f, "}}")?;

    Ok(())
  }
}

#[derive(Debug)]
pub enum StructBodyStatement {
  Property {
    property_declaration: VariableDeclaration,
    is_saved: bool,
    is_editable: bool
  },
  DefaultValue(VariableAssignment)
}

impl Visited for StructBodyStatement {
  fn accept<T: visitor::Visitor>(&self, visitor: &mut T) {
    match self {
      StructBodyStatement::Property {
        property_declaration,
        is_saved: _,
        is_editable: _
      } => property_declaration.accept(visitor),
      StructBodyStatement::DefaultValue(x) => x.accept(visitor)
    }
  }
}

impl Codegen for StructBodyStatement {
  fn emit(&self, context: &Context, f: &mut Vec<u8>) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    match self {
      StructBodyStatement::Property {
        property_declaration,
        is_saved,
        is_editable
      } => {
        if *is_editable {
          write!(f, "editable ")?;
        }

        if *is_saved {
          write!(f, "saved ")?;
        }

        property_declaration.emit(context, f)?;
      }
      StructBodyStatement::DefaultValue(x) => {
        write!(f, "default ")?;
        x.emit(context, f)?;
        writeln!(f, ";")?;
      }
    };

    Ok(())
  }
}
