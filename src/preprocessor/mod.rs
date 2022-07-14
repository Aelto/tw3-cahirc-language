use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use regex::RegexBuilder;

mod pragma_replace;

use crate::utils;

use self::pragma_replace::get_pragma_replace_directives;

/// Entry point for the pre-processor,
/// It takes as input the source directory and the list of dependencies and their
/// names. And returns as output the files content from the source directory
/// and the files content from the dependencies.
pub fn preprocess(
  source_directory: &str,
  dependencies: &HashMap<String, String>,
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

  // TODO:
  // - add a while loop over the whole function that loops until none of the
  // files have a macro call. To support recursive macros and macros that
  // generate other macros.
  // - each iteration expand the macro calls

  let macro_call_regex = Regex::new(r"(\w+)!").unwrap();
  let macro_function_regex = RegexBuilder::new(r"#define function (\w+)\((.*?)\) \{(.*)\};")
    .multi_line(true)
    .dot_matches_new_line(true)
    .build()
    .unwrap();
  let macro_const_regex = Regex::new(r"#define const (\w+) = (.*);").unwrap();
  let mut registered_macros = HashMap::new();
  let mut contains_macro_call = true;
  while contains_macro_call {
    contains_macro_call = false;

    for (_name, files) in output.dependencies_files_content.iter() {
      for (_filename, content) in files.iter() {
        let mut new_content = content.content.borrow().to_string();

        loop {
          let cap = macro_function_regex.captures(&new_content);

          if cap.is_none() {
            break;
          }

          let cap = cap.unwrap();
          let macro_name = &cap[1];
          let macro_parameters = &cap[2].trim();
          let macro_body = &cap[3].trim();

          let f = parse_macro_function(macro_parameters, macro_body);
          let body_length = f.body.len();
          let macro_end = new_content.find(&f.body).unwrap();

          registered_macros.insert(macro_name.to_string(), MacroDefinition::Function(f));

          let macro_start_pattern = format!("#define function {macro_name}");
          let macro_start = new_content.find(&macro_start_pattern).unwrap();
          let closing_bracket_index = &new_content[macro_end + body_length..].find("};").unwrap();

          new_content.replace_range(
            macro_start..macro_end + body_length + closing_bracket_index + 2,
            "",
          );
        }

        loop {
          let cap = macro_const_regex.captures(&new_content);

          if cap.is_none() {
            break;
          }

          let cap = cap.unwrap();
          let macro_name = &cap[1];
          let macro_value = &cap[2].trim();

          println!("registering macro const: {macro_name} = {macro_value}");

          registered_macros.insert(
            macro_name.to_string(),
            MacroDefinition::Constant(MacroConstant {
              name: macro_name.to_string(),
              value: macro_value.to_string(),
            }),
          );

          let macro_start = new_content
            .find(&format!("#define const {macro_name}"))
            .unwrap();
          let macro_end = macro_start + &cap[0].len();

          new_content.replace_range(macro_start..macro_end + 1, "");
        }

        for cap in macro_call_regex.captures_iter(&content.content.borrow()) {
          let macro_name = &cap[1];

          // println!("found macro call: {macro_name}");

          if !registered_macros.contains_key(macro_name) {
            println!("Warning, macro call to unkown macro: {}", &macro_name);

            continue;
          }

          new_content = expand_macro_call(new_content.to_string(), macro_name, &registered_macros);
        }

        contains_macro_call =
          contains_macro_call || macro_call_regex.find(&content.content.borrow()).is_some();

        content.content.replace(new_content);
      }
    }

    for (_filename, content) in output.source_files_content.iter() {
      let mut new_content = content.content.borrow().to_string();

      loop {
        let cap = macro_function_regex.captures(&new_content);

        if cap.is_none() {
          break;
        }

        let cap = cap.unwrap();
        let macro_name = &cap[1];
        let macro_parameters = &cap[2].trim();
        let macro_body = &cap[3].trim();

        let f = parse_macro_function(macro_parameters, macro_body);
        let body_length = f.body.len();
        let macro_end = new_content.find(&f.body).unwrap();

        registered_macros.insert(macro_name.to_string(), MacroDefinition::Function(f));

        let macro_start_pattern = format!("#define function {macro_name}");
        let macro_start = new_content.find(&macro_start_pattern).unwrap();
        let closing_bracket_index = &new_content[macro_end + body_length..].find("};").unwrap();

        new_content.replace_range(
          macro_start..macro_end + body_length + closing_bracket_index + 2,
          "",
        );
      }

      loop {
        let cap = macro_const_regex.captures(&new_content);

        if cap.is_none() {
          break;
        }

        let cap = cap.unwrap();
        let macro_name = &cap[1];
        let macro_value = &cap[2].trim();

        println!("registering macro const: {macro_name} = {macro_value}");

        registered_macros.insert(
          macro_name.to_string(),
          MacroDefinition::Constant(MacroConstant {
            name: macro_name.to_string(),
            value: macro_value.to_string(),
          }),
        );

        let macro_start = new_content
          .find(&format!("#define const {macro_name}"))
          .unwrap();
        let macro_end = macro_start + &cap[0].len();

        new_content.replace_range(macro_start..macro_end + 1, "");
      }

      for cap in macro_call_regex.captures_iter(&content.content.borrow()) {
        let macro_name = &cap[1];

        // println!("found macro call: {macro_name}");

        if !registered_macros.contains_key(macro_name) {
          println!("Warning, macro call to unkown macro: {}", &macro_name);

          continue;
        }

        new_content = expand_macro_call(new_content.to_string(), macro_name, &registered_macros);
      }

      contains_macro_call =
        contains_macro_call || macro_call_regex.find(&content.content.borrow()).is_some();

      content.content.replace(new_content);
    }
  }

  Ok(output)
}

type FileName = String;
type DependencyName = String;

pub struct ProcessedFile {
  pub content: RefCell<String>,
  pub path: PathBuf,
}

pub struct PreprocessorOutput {
  pub source_files_content: HashMap<FileName, ProcessedFile>,

  pub dependencies_files_content: HashMap<DependencyName, HashMap<FileName, ProcessedFile>>,
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
    let content = RefCell::new(utils::strip_comments(std::fs::read_to_string(
      filename.path(),
    )?));

    output.push((
      filename
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string(),
      ProcessedFile {
        content,
        path: filename.path().to_path_buf(),
      },
    ));
  }

  Ok(output)
}

fn parse_macro_function(macro_parameters: &str, macro_body: &str) -> MacroFunction {
  let parameters = if !macro_parameters.is_empty() {
    let mut slice = &macro_parameters[..];
    let mut output = Vec::new();

    loop {
      if slice.starts_with(",") {
        slice = &slice[1..];
      }

      if slice.starts_with(")") {
        break;
      }

      let end = slice.len();
      let comma_index = slice.find(",");
      let paren_index = slice.find(")");
      let parameter = &slice[..comma_index.unwrap_or(end).min(paren_index.unwrap_or(end))];

      output.push(parameter.trim().to_string());

      slice = &slice[parameter.len()..];

      if comma_index.is_none() {
        break;
      }
    }

    output
  } else {
    Vec::new()
  };

  // before starting to parse, we must find the actual end of the body since the
  // regex is greedy. It is done on purpose to allow recursive macros and macros
  // that expand into #define calls.
  let mut slice = &macro_body[..];
  loop {
    let define_index = slice.find("#define function");
    let macro_end_index = slice.find("};");

    let macro_end_index = match macro_end_index {
      Some(i) => i,
      None => slice.len(),
    };

    if define_index.is_none() || macro_end_index < define_index.unwrap() {
      // let macro_end_index = slice.find("};");

      slice = &slice[..macro_end_index];

      break;
    }

    slice = &slice[define_index.unwrap() + "#define function".len()..];

    let macro_end_index = slice.find("};");

    if macro_end_index.is_none() {
      panic!("2, unfinished macro");
    }

    slice = &slice[macro_end_index.unwrap() + "};".len()..];
  }

  let end = slice.as_ptr() as usize - macro_body.as_ptr() as usize + slice.len();

  MacroFunction {
    parameters,
    body: macro_body[..end].to_string(),
  }
}

fn expand_macro_call(
  content: String,
  macro_name: &str,
  registered_macros: &HashMap<String, MacroDefinition>,
) -> String {
  let macro_call_index = content.find(&format!("{macro_name}!"));

  if macro_call_index.is_none() {
    println!("could not find macro call {macro_name}!");

    return content;
  }

  let macro_call_index = macro_call_index.unwrap();
  let definition = registered_macros.get(macro_name).unwrap();

  match definition {
    MacroDefinition::Function(function) => {
      // +1 to remove the opening parenthesis and the !
      let mut slice = &content[macro_call_index + macro_name.len() + 2..];
      let mut parameters = Vec::new();

      loop {
        if slice.starts_with(",") {
          slice = &slice[1..];
        }

        while slice.starts_with(" ") {
          slice = &slice[1..];
        }

        if parameters.len() == function.parameters.len() || slice.starts_with(")") {
          break;
        }

        if slice.trim().starts_with("{{") {
          // it's a body of code,

          let body_end_index = slice.find("}}");

          if body_end_index.is_none() {
            panic!("Unterminated body of code in macro call {}", macro_name);
          }

          let body_end_index = body_end_index.unwrap();

          parameters.push(&slice[2..body_end_index]);

          slice = &slice[2 + body_end_index + 2..];
        } else {
          let end = slice.len();
          let comma_index = slice.find(",").unwrap_or(end);
          let paren_index = slice.find(")").unwrap_or(end);

          parameters.push(&slice[..comma_index.min(paren_index)]);
          slice = &slice[comma_index.min(paren_index)..];
        }
      }

      let mut body = function.body.clone();
      for i in 0..function.parameters.len() {
        let parameter = &function.parameters[i];
        let value = &parameters[i];

        body = body.replace(parameter, value.trim());
      }

      let findreplace_directives = get_pragma_replace_directives(&body);

      for directive in findreplace_directives {
        body = body.replace(&directive.find, &directive.replace);
      }

      let end = slice.as_ptr() as usize - content.as_ptr() as usize;
      let start = if body.contains("$") {
        let line_before_macro_call = &content[..macro_call_index];

        let start = if let Some(line) = line_before_macro_call.rfind(";") {
          line + 1
        } else {
          0
        };

        body = body.replace("$", &content[start..macro_call_index]);

        // +1 to remove the ";"
        start + 1
      } else {
        macro_call_index
      };

      content.replace(&content[start..end + 1], &body)
    }
    MacroDefinition::Constant(constant) => {
      content.replacen(&format!("{}!", constant.name), &constant.value, 1)
    }
  }
}

#[derive(Debug)]
struct MacroFunction {
  parameters: Vec<String>,
  body: String,
}

#[derive(Debug)]
struct MacroConstant {
  name: String,
  value: String,
}

#[derive(Debug)]
enum MacroDefinition {
  Function(MacroFunction),
  Constant(MacroConstant),
}
