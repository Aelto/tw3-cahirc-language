pub fn strip_comments(input: String) -> String {
  let mut output = String::new();

  for line in input.lines() {
    output.push_str("\n");

    if let Some(comment_start) = line.find("//") {
      // before stripping the comment, we must check it is not in a string.
      let mut quotes_indices = Vec::new();
      let mut slice = &line[..];
      let mut absolute_index = 0;

      while let Some(index) = slice.find(r#"""#) {
        quotes_indices.push(absolute_index + index);
        absolute_index += index;
        slice = &slice[index + 1..];
      }

      let mut is_in_string = false;
      for chunk in quotes_indices.chunks(2) {
        if chunk.len() > 1 {
          let start = chunk[0];
          let end = chunk[1];

          is_in_string = is_in_string || start < comment_start && comment_start < end;
        } else {
          println!("Warning: unterminated string found while stripping comments");
          continue;
        }
      }

      if is_in_string {
        output.push_str(&line[..]);
      } else {
        output.push_str(&line[..comment_start]);
      }
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
