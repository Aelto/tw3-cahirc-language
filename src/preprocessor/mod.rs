use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

use regex::Regex;
use regex::RegexBuilder;

mod conditionals;
mod expand_macros;
mod pragma_replace;
pub mod types;

use crate::utils::convert_line_endings;

use self::conditionals::filter_conditionals;
use self::types::*;

/// Entry point for the pre-processor,
/// It takes as input the source directory and the list of dependencies and their
/// names. And returns as output the files content from the source directory
/// and the files content from the dependencies.
pub fn preprocess(
  source_directory: &str, dependencies: &HashMap<String, String>,
) -> std::io::Result<PreprocessorOutput> {
  let mut output = PreprocessorOutput {
    dependencies_files_content: HashMap::new(),
    source_files_content: HashMap::new(),
  };

  for (name, content) in get_wss_files_content_for_directory(&Path::new(source_directory))? {
    output.source_files_content.insert(name, content);
  }

  for (name, value) in dependencies.iter() {
    if value.starts_with("https://") {
      todo!();
    } else {
      output.dependencies_files_content.insert(
        name.to_string(),
        HashMap::from_iter(get_wss_files_content_for_directory(&Path::new(value))?.into_iter()),
      );
    }
  }

  let regex_collection = RegexCollection {
    macro_call: Regex::new(r"(\w+)!").unwrap(),
    macro_function: RegexBuilder::new(r"#define function (\w+)\((.*?)\) \{(.*)\};")
      .multi_line(true)
      .dot_matches_new_line(true)
      .build()
      .unwrap(),
    macro_const: Regex::new(r"#define const (\w+).*;").unwrap(),
    macro_const_value: Regex::new(r"= (.*);").unwrap(),
    macro_ifdef: RegexBuilder::new(r"#ifdef (\w*) \{(.*?)\};")
      .multi_line(true)
      .dot_matches_new_line(true)
      .build()
      .unwrap(),
    macro_ifndef: RegexBuilder::new(r"#ifndef (\w*) \{(.*?)\};")
      .multi_line(true)
      .dot_matches_new_line(true)
      .build()
      .unwrap(),
  };

  let mut registered_macros = HashMap::new();
  let mut contains_macro_call = true;
  while contains_macro_call {
    contains_macro_call = false;

    for (_name, files) in output.dependencies_files_content.iter() {
      for (_filename, content) in files.iter() {
        let mut new_content = content.content.borrow().to_string();

        let file_still_contains_macro_calls =
          expand_macros::expand_macros(&mut registered_macros, &mut new_content, &regex_collection);

        contains_macro_call = contains_macro_call || file_still_contains_macro_calls;

        content.content.replace(new_content);
      }
    }

    for (_filename, content) in output.source_files_content.iter() {
      let mut new_content = content.content.borrow().to_string();

      let file_still_contains_macro_calls =
        expand_macros::expand_macros(&mut registered_macros, &mut new_content, &regex_collection);

      contains_macro_call = contains_macro_call || file_still_contains_macro_calls;

      content.content.replace(new_content);
    }
  }

  // a final pass over the files to remove the conditional macros
  for (_name, files) in output.dependencies_files_content.iter() {
    for (_filename, content) in files.iter() {
      let mut new_content = content.content.borrow().to_string();

      filter_conditionals(
        &registered_macros,
        &mut new_content,
        &regex_collection,
        conditionals::ConditionType::IfDefined,
      );

      filter_conditionals(
        &registered_macros,
        &mut new_content,
        &regex_collection,
        conditionals::ConditionType::IfNotDefined,
      );

      content.content.replace(new_content);
    }
  }

  for (_filename, content) in output.source_files_content.iter() {
    let mut new_content = content.content.borrow().to_string();

    filter_conditionals(
      &registered_macros,
      &mut new_content,
      &regex_collection,
      conditionals::ConditionType::IfDefined,
    );

    filter_conditionals(
      &registered_macros,
      &mut new_content,
      &regex_collection,
      conditionals::ConditionType::IfNotDefined,
    );

    content.content.replace(new_content);
  }

  Ok(output)
}

fn get_wss_files_content_for_directory(
  dir: &Path,
) -> std::io::Result<Vec<(FileName, ProcessedFile)>> {
  let files = walkdir::WalkDir::new(&dir)
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

  let mut output = Vec::new();
  for filename in files {
    let content = RefCell::new(convert_line_endings(std::fs::read_to_string(
      filename.path(),
    )?));

    output.push((
      filename.path().to_str().unwrap().to_string(),
      ProcessedFile {
        content,
        path: filename.path().to_path_buf(),
      },
    ));
  }

  Ok(output)
}
