pub fn strip_comments(input: String) -> String {
  let mut output = String::new();

  for line in input.lines() {
    output.push_str("\n");

    if let Some(comment_start) = line.find("//") {
      output.push_str(&line[..comment_start]);
    } else {
      output.push_str(&line[..]);
    }
  }

  while let Some(begin) = output.find("/*") {
    if let Some(end) = output.find("*/") {
      output.replace_range(begin..end + 2, "");
    } else {
      panic!("Untermined comment");
    }
  }

  output
}
