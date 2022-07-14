pub fn get_pragma_replace_directives(body: &str) -> Vec<PragmaReplace> {
  let find_lines = body
    .lines()
    .map(|line| line.trim_start())
    .filter(|line| line.starts_with("#pragma find "));

  let replace_lines = body
    .lines()
    .map(|line| line.trim_start())
    .filter(|line| line.starts_with("#pragma replace"));

  find_lines
    .zip(replace_lines)
    .map(|(find, replace)| parse_pragma_findreplace(find, replace))
    .collect()
}

fn parse_pragma_findreplace(find: &str, replace: &str) -> PragmaReplace {
  PragmaReplace {
    find: find.replacen("#pragma find ", "", 1),
    replace: replace.replacen("#pragma replace ", "", 1),
  }
}

#[derive(Debug)]
pub struct PragmaReplace {
  pub find: String,
  pub replace: String,
}
