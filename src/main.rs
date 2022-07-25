use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

mod ast;
mod config;
mod preprocessor;
mod utils;

extern crate lalrpop_util;

use ariadne::Source;
use ast::Program;
use ast::ProgramInformation;
use config::read_config;
use config::Config;
use lalrpop_util::lalrpop_mod;

use crate::ast::codegen::context::Context;
use crate::ast::visitor::ContextBuildingVisitor;
use crate::ast::visitor::FunctionVisitor;
use crate::ast::visitor::LambdaDeclarationVisitor;
use crate::ast::visitor::LibraryEmitterVisitor;
use crate::ast::visitor::VariableDeclarationVisitor;
use crate::utils::strip_pragmas;

lalrpop_mod!(pub parser);

fn main() {
  let config = read_config().expect("Could not read the config cahirc.toml file");

  compile_source_directory(&config).expect("main error");
}

fn compile_source_directory(config: &Config) -> std::io::Result<()> {
  let preprocessed_content = preprocessor::preprocess(&config.package.src, &config.dependencies)?;

  let program_information = ProgramInformation::new();
  let global_context = Rc::new(RefCell::new(Context::new("Program", None)));

  // 1.
  // Build the list of AST from the files
  let mut dependency_ast_list = Vec::new();
  let mut ast_list = Vec::new();

  // starting with the dependencies
  for (_name, value) in preprocessed_content.dependencies_files_content.iter() {
    for (_filename, file) in value.iter() {
      let content = strip_pragmas(&file.content.borrow());

      if file
        .content
        .borrow()
        .contains("#pragma cahirc-preprocessor-print")
      {
        println!("{}", &file.content.borrow());
      }

      let expr = parser::ProgramParser::new()
        .parse(&program_information, &content)
        .unwrap();

      dependency_ast_list.push(ParsedFile {
        ast: expr,
        file_path: file.path.clone(),
      });
    }
  }

  for (filename, file) in preprocessed_content.source_files_content.iter() {
    use ariadne::{ColorGenerator, Label, Report, ReportKind};
    let content = strip_pragmas(&file.content.borrow());

    if file
      .content
      .borrow()
      .contains("#pragma cahirc-preprocessor-print")
    {
      println!("{}", &file.content.borrow());
    }

    let expr = parser::ProgramParser::new().parse(&program_information, &content);
    let expr = match expr {
      Ok(expr) => expr,
      Err(error) => {
        match error {
          lalrpop_util::ParseError::InvalidToken { location } => {
            let mut colors = ColorGenerator::new();
            let a = colors.next();
            let absolute_path =
              dunce::canonicalize(std::env::current_dir().unwrap().join(&file.path)).unwrap();

            Report::build(ReportKind::Error, (), location)
              .with_message(&format!(
                "Invalid token in file://{}",
                absolute_path.to_str().unwrap().replace("\\", "/")
              ))
              .with_label(
                Label::new(location..location.checked_add(1).unwrap_or(location))
                  .with_message("The invalid token")
                  .with_color(a),
              )
              .finish()
              .print(Source::from(&content))
              .unwrap();
          }
          lalrpop_util::ParseError::UnrecognizedEOF {
            location: _,
            expected: _,
          } => {
            println!("Unrecognized EOF in {}", filename);
          }
          lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            let mut colors = ColorGenerator::new();
            let a = colors.next();
            let absolute_path =
              dunce::canonicalize(std::env::current_dir().unwrap().join(&file.path)).unwrap();

            Report::build(ReportKind::Error, (), token.0)
              .with_message(&format!(
                "Unrecognized token in file://{}",
                absolute_path.to_str().unwrap().replace("\\", "/")
              ))
              .with_label(
                Label::new(token.0..token.2)
                  .with_message(format!("Expected {}", expected.join(" | ")))
                  .with_color(a),
              )
              .finish()
              .print(Source::from(&content))
              .unwrap();
          }
          lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
          lalrpop_util::ParseError::User { error: _ } => todo!(),
        };

        continue;
      }
    };

    ast_list.push(ParsedFile {
      ast: expr,
      file_path: file.path.clone(),
    });
  }

  // 2.
  // Traverse the AST to collect information about it
  for parsed_file in &dependency_ast_list {
    let mut variable_declaration_visitor = VariableDeclarationVisitor::new(&program_information);
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
    parsed_file.ast.accept(&mut variable_declaration_visitor);
  }

  for parsed_file in &ast_list {
    let mut variable_declaration_visitor = VariableDeclarationVisitor::new(&program_information);
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
    parsed_file.ast.accept(&mut variable_declaration_visitor);
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
      Ok(s) => {
        if !s.trim().is_empty() {
          fs::write(new_path, format_code(s)).expect("failed to write output file")
        }
      }
      Err(e) => println!("{}", e),
    };

    // (*global_context).borrow().print(0);
  }

  // 4.
  // emit code for the libraries code, especially the generic functions that
  // were used.
  let generated_code_file = Path::new(&config.package.dist)
    .join(uuid::Uuid::new_v4().to_string())
    .with_extension("ws");

  let mut file_content = Vec::new();

  std::fs::create_dir_all(&generated_code_file.parent().unwrap())
    .expect("failed to recursively make the output directories");

  for parsed_file in &dependency_ast_list {
    use ast::visitor::Visited;
    let mut visitor = LibraryEmitterVisitor::new(&global_context, &mut file_content);
    parsed_file.ast.accept(&mut visitor);

    let mut visitor = LambdaDeclarationVisitor::new(&mut file_content);
    parsed_file.ast.accept(&mut visitor);
  }

  for parsed_file in &ast_list {
    use ast::visitor::Visited;

    let mut visitor = LambdaDeclarationVisitor::new(&mut file_content);
    parsed_file.ast.accept(&mut visitor);
  }

  match std::str::from_utf8(&file_content) {
    Ok(s) => {
      if !s.trim().is_empty() {
        fs::write(&generated_code_file, format_code(s)).expect("failed to write output file")
      }
    }
    Err(e) => println!("{}", e),
  };

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

  lines.join("\n")
}

struct ParsedFile {
  file_path: PathBuf,
  ast: Program,
}
