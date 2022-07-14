pub fn strip_pragmas(input: &String) -> String {
  input
    .lines()
    .filter(|line| !contains_pragma(line))
    .collect::<Vec<&str>>()
    .join("\n")
}

fn contains_pragma(line: &str) -> bool {
  line.trim_start().starts_with("#pragma ")
}
