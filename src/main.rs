use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

mod ast;

extern crate lalrpop_util;

use ast::ProgramInformation;
use lalrpop_util::lalrpop_mod;

use crate::ast::codegen::context::Context;
use crate::ast::visitor::ContextBuildingVisitor;
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

    if file.path().extension().unwrap() != "wss" {
      continue;
    }
    println!("");
    println!("");

    let content = std::fs::read_to_string(file.path())?;

    let expr = parser::ProgramParser::new()
      .parse(&program_information, &content)
      .unwrap();

    // dbg!(&expr);

    let mut function_visitor = FunctionVisitor {
      program_information: &program_information,
    };

    let global_context = Rc::new(RefCell::new(Context::new("Program", None)));
    let mut context_builder = ContextBuildingVisitor {
      current_context: global_context.clone(),
    };

    use ast::visitor::Visited;

    expr.accept(&mut context_builder);
    expr.accept(&mut function_visitor);

    let mut new_path = file.path();
    new_path.set_extension("ws");

    use ast::codegen::Codegen;
    let mut output_code = Vec::new();
    expr.emit(&global_context.borrow(), &mut output_code)
      .expect("failed to emit code");

    match std::str::from_utf8(&output_code) {
      Ok(s) => fs::write(new_path, format_code(s)).expect("failed to write output file"),
      Err(e) => println!("{}", e)
    };

    (*global_context).borrow().print(0);

    // let functions = program_information.generic_functions.borrow();
    // for function in functions.iter() {
    //   dbg!(function);
    // }
  }

  Ok(())
}

fn format_code(origin: &str) -> String {
  let mut lines: Vec<String> = origin.lines().map(|s| s.to_string()).collect();
  let mut depth = 0;

  for i in 0..lines.len() {
    if lines[i].starts_with("}") && depth > 0 {
      depth -= 1;
    }

    let formated_line = format!("{}{}", "  ".repeat(depth), lines[i]);

    if lines[i].ends_with("{") {
      depth += 1;
    }

    lines[i] = formated_line;
  }

  lines.join("\r\n")
}
