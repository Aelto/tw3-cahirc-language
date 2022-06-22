use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;

mod ast;

extern crate lalrpop_util;

use ast::ProgramInformation;
use lalrpop_util::lalrpop_mod;

use crate::ast::visitor::FunctionVisitor;

lalrpop_mod!(pub parser);

fn main() {
  let source_directory = Path::new("example");

  compile_source_directory(source_directory).expect("main error");
}

fn compile_source_directory(directory: &Path) -> std::io::Result<()> {
  let children = fs::read_dir(directory)?;
  let program_information = ProgramInformation::new();

  for file in children {
    let file = file?;
    let content = std::fs::read_to_string(file.path())?;

    let mut expr = parser::ProgramParser::new()
      .parse(&program_information, &content)
      .unwrap();

    dbg!(&expr);

    let mut visitor = FunctionVisitor {
      program_information: &program_information,
    };

    use ast::visitor::Visited;

    expr.accept(&mut visitor);

    // let functions = program_information.generic_functions.borrow();
    // for function in functions.iter() {
    //   dbg!(function);
    // }
  }

  Ok(())
}
