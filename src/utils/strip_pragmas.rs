pub fn strip_pragmas(input: &String) -> String {
  input.lines().filter(|line| !contains_pragma(line)).collect::<Vec<&str>>().join("\n")
}

fn contains_pragma(line: &str) -> bool {
  line.starts_with("#pragma ")
}