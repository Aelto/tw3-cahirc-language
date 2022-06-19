use std::path::Path;
use std::fs;

mod ast;

extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

fn main() {
  let source_directory = Path::new("example");

  compile_source_directory(source_directory).expect("main error");
}

fn compile_source_directory(directory: &Path) -> std::io::Result<()> {
  let children = fs::read_dir(directory)?;

  for file in children {
    let file = file?;
    let content = std::fs::read_to_string(file.path())?;
    
    let expr = parser::ProgramParser::new()
      .parse(&content)
      .unwrap();

    dbg!(&expr);
  }

  Ok(())
}
