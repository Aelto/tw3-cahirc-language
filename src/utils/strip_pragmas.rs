pub fn strip_pragmas(input: &String) -> String {
  input
    .lines()
    .map(|line| if contains_pragma(line) { "" } else { line })
    .collect::<Vec<&str>>()
    .join("\n")
}

fn contains_pragma(line: &str) -> bool {
  line.trim_start().starts_with("#pragma ")
}
