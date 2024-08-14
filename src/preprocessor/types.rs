use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;

pub type FileName = String;
pub type DependencyName = String;

pub struct ProcessedFile {
  pub content: RefCell<String>,
  pub path: PathBuf
}

pub struct PreprocessorOutput {
  pub source_files_content: HashMap<FileName, ProcessedFile>,

  pub dependencies_files_content: HashMap<DependencyName, HashMap<FileName, ProcessedFile>>
}

#[derive(Debug)]
pub struct MacroFunction {
  pub parameters: Vec<String>,
  pub body: String
}

#[derive(Debug)]
pub struct MacroConstant {
  pub name: String,
  pub value: String
}

#[derive(Debug)]
pub enum MacroDefinition {
  Function(MacroFunction),
  Constant(MacroConstant)
}

pub struct RegexCollection {
  pub macro_const: Regex,
  pub macro_const_value: Regex,
  pub macro_function: Regex,
  pub macro_call: Regex,
  pub macro_ifdef: Regex,
  pub macro_ifndef: Regex
}
