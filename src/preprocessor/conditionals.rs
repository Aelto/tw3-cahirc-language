use std::collections::HashMap;

use regex::Captures;

use super::types::*;

pub fn filter_conditionals(
  registered_macros: &HashMap<String, MacroDefinition>,
  new_content: &mut String,
  regex_collection: &RegexCollection,
  condition_type: ConditionType,
) {
  loop {
    let content_copy = new_content.clone();
    let regex = match &condition_type {
      ConditionType::IfDefined => &regex_collection.macro_ifdef,
      ConditionType::IfNotDefined => &regex_collection.macro_ifndef,
    };
    let cap = regex.captures(&content_copy);

    if cap.is_none() {
      break;
    }

    let cap = cap.unwrap();
    let start = new_content.find(&cap[0]).unwrap();

    filter_conditional(
      registered_macros,
      new_content,
      regex_collection,
      &condition_type,
      cap,
      start,
    );
  }
}

fn filter_conditional(
  registered_macros: &HashMap<String, MacroDefinition>,
  new_content: &mut String,
  regex_collection: &RegexCollection,
  condition_type: &ConditionType,
  capture: Captures,
  start: usize,
) {
  let full_match = &capture[0];

  // if we were able to find another nested ifdef macro inside the current macro,
  // we will proceed from bottom to top, from the most nested to the global macros
  // while expanding.
  //
  // we go forward in the fullmatch by just 1 to go past the # in #ifdef so it
  // doesn't match with itself infinitely.
  let regex = match condition_type {
    ConditionType::IfDefined => &regex_collection.macro_ifdef,
    ConditionType::IfNotDefined => &regex_collection.macro_ifndef,
  };

  if let Some(sub_capture) = regex.captures(&full_match[1..]) {
    let sub_start = full_match.find(&sub_capture[0]).unwrap();

    filter_conditional(
      registered_macros,
      new_content,
      regex_collection,
      condition_type,
      sub_capture,
      start + sub_start,
    );

    // we let the next iteration of the pre processor parse the current macro,
    // if it doesn't have more nested macros in it that is.
    return;
  }

  let constant_name = &capture[1];
  let body = &capture[2];
  let is_defined = registered_macros.contains_key(constant_name);

  new_content.replace_range(
    start..start + full_match.len(),
    match condition_type {
      ConditionType::IfDefined => {
        if is_defined {
          body
        } else {
          ""
        }
      }
      ConditionType::IfNotDefined => {
        if is_defined {
          ""
        } else {
          body
        }
      }
    },
  );
}

pub enum ConditionType {
  IfDefined,
  IfNotDefined,
}
