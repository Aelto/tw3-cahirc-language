use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

mod ast;

extern crate lalrpop_util;

use ast::Program;
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
  let global_context = Rc::new(RefCell::new(Context::new("Program", None)));
  let wss_files = children
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .filter(|file| {
      file
        .path()
        .extension()
        .and_then(|ext| Some(ext == "wss"))
        .unwrap_or(false)
    });

  // 1.
  // Build the list of AST from the files
  let mut ast_list = Vec::new();
  for file in wss_files {
    let content = std::fs::read_to_string(file.path())?;

    let expr = parser::ProgramParser::new()
      .parse(&program_information, &content)
      .unwrap();

    ast_list.push(ParsedFile {
      ast: expr,
      file_path: file.path(),
    });
  }

  // 2.
  // Traverse the AST to collect information about it
  for parsed_file in &ast_list {
    let mut function_visitor = FunctionVisitor {
      program_information: &program_information,
    };

    // create a context for this file, and register it into the global context
    let file_context = Rc::new(RefCell::new(Context::new(
      &format!("file: {:#?}", parsed_file.file_path.file_name()),
      None,
    )));
    Context::set_parent_context(&file_context, &global_context);

    let mut context_builder = ContextBuildingVisitor {
      current_context: file_context.clone(),
    };

    use ast::visitor::Visited;

    parsed_file.ast.accept(&mut context_builder);
    parsed_file.ast.accept(&mut function_visitor);
  }

  // 3.
  // Emit code using the information we collected in the previous step
  for parsed_file in &ast_list {
    let mut new_path = parsed_file.file_path.clone();
    new_path.set_extension("ws");

    use ast::codegen::Codegen;
    let mut output_code = Vec::new();
    parsed_file
      .ast
      .emit(&global_context.borrow(), &mut output_code)
      .expect("failed to emit code");

    match std::str::from_utf8(&output_code) {
      Ok(s) => fs::write(new_path, format_code(s)).expect("failed to write output file"),
      Err(e) => println!("{}", e),
    };

    (*global_context).borrow().print(0);
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

struct ParsedFile {
  file_path: PathBuf,
  ast: Program,
}
