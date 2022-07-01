use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

mod ast;
mod config;

extern crate lalrpop_util;

use ast::Program;
use ast::ProgramInformation;
use config::read_config;
use config::Config;
use lalrpop_util::lalrpop_mod;

use crate::ast::codegen::context::Context;
use crate::ast::visitor::ContextBuildingVisitor;
use crate::ast::visitor::FunctionVisitor;
use crate::ast::visitor::LibraryEmitterVisitor;

lalrpop_mod!(pub parser);

fn main() {
  let config = read_config().expect("Could not read the config cahir.toml file");

  compile_source_directory(&config).expect("main error");
}

fn compile_source_directory(config: &Config) -> std::io::Result<()> {
  let program_information = ProgramInformation::new();
  let global_context = Rc::new(RefCell::new(Context::new("Program", None)));
  let wss_files = walkdir::WalkDir::new(&config.package.src)
    .into_iter()
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
  let mut dependency_ast_list = Vec::new();
  let mut ast_list = Vec::new();

  // starting with the dependencies
  for (name, value) in config.dependencies.iter() {
    if value.starts_with("https://") {
      todo!();
    } else {
      println!("parsing dependency {}", name);

      let wss_files = walkdir::WalkDir::new(&Path::new(value))
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(|file| {
          file
            .path()
            .extension()
            .and_then(|ext| Some(ext == "wss"))
            .unwrap_or(false)
        });

      for file in wss_files {
        let content = std::fs::read_to_string(file.path())?;

        let expr = parser::ProgramParser::new()
          .parse(&program_information, &content)
          .unwrap();

        dependency_ast_list.push(ParsedFile {
          ast: expr,
          file_path: file.path().to_path_buf(),
        });
      }
    }
  }

  for file in wss_files {
    let content = std::fs::read_to_string(file.path())?;

    let expr = parser::ProgramParser::new()
      .parse(&program_information, &content)
      .unwrap();

    ast_list.push(ParsedFile {
      ast: expr,
      file_path: file.path().to_path_buf(),
    });
  }

  // 2.
  // Traverse the AST to collect information about it
  for parsed_file in &dependency_ast_list {
    let mut function_visitor = FunctionVisitor {
      program_information: &program_information,
    };

    // create a context for this file, and register it into the global context
    let file_context = Rc::new(RefCell::new(Context::new(
      &format!("file: {:#?}", parsed_file.file_path.file_name()),
      None,
    )));

    file_context.borrow_mut().set_as_library();

    Context::set_parent_context(&file_context, &global_context);

    let mut context_builder = ContextBuildingVisitor {
      current_context: file_context.clone(),
    };

    use ast::visitor::Visited;

    parsed_file.ast.accept(&mut context_builder);
    parsed_file.ast.accept(&mut function_visitor);
  }

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
  if let Err(_) = std::fs::remove_dir_all(&config.package.dist) {}

  for parsed_file in &ast_list {
    let new_path = parsed_file
      .file_path
      .strip_prefix(&config.package.src)
      .expect(&format!(
        "could not form the path to {:?}'s output file",
        &parsed_file.file_path,
      ))
      .to_path_buf();

    let mut new_path = Path::new(&config.package.dist).join(new_path);

    new_path.set_extension("ws");

    use ast::codegen::Codegen;
    let mut output_code = Vec::new();
    parsed_file
      .ast
      .emit(&global_context.borrow(), &mut output_code)
      .expect("failed to emit code");

    std::fs::create_dir_all(&new_path.parent().unwrap())
      .expect("failed to recursively make the outoput directories");

    match std::str::from_utf8(&output_code) {
      Ok(s) => fs::write(new_path, format_code(s)).expect("failed to write output file"),
      Err(e) => println!("{}", e),
    };

    // (*global_context).borrow().print(0);
  }

  // 4.
  // emit code for the libraries code, especially the generic functions that
  // were used.
  for parsed_file in &dependency_ast_list {
    let new_path = Path::new(&config.package.dist)
      .join(uuid::Uuid::new_v4().to_string())
      .with_extension("ws");

    use ast::visitor::Visited;
    let mut visitor = LibraryEmitterVisitor::new(&global_context);
    parsed_file.ast.accept(&mut visitor);

    std::fs::create_dir_all(&new_path.parent().unwrap())
      .expect("failed to recursively make the outoput directories");

    match std::str::from_utf8(&visitor.emitted_code) {
      Ok(s) => fs::write(new_path, format_code(s)).expect("failed to write output file"),
      Err(e) => println!("{}", e),
    };

    // (*global_context).borrow().print(0);
  }

  // for file_context in &global_context.borrow().children_contexts {
  //   if !file_context.borrow().is_library {
  //     continue;
  //   }

  //   let filename = uuid::Uuid::new_v4().to_string();
  //   let mut content = String::new();

  //   for context in file_context.borrow().children_contexts {
  //     if let Some(context.borrow().generic_context
  //   }
  // }

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
