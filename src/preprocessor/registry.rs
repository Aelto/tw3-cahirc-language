use std::collections::{HashMap, HashSet};

pub use nom::bytes::complete::{tag, take_until1};
pub use nom::character::complete::char;
pub use nom::sequence::delimited;
pub use nom::IResult;
use nom::Offset;

use super::PreprocessorOutput;

pub fn handle_registers(output: &mut PreprocessorOutput) {
  let registers = collect_registers(output);
  emit_registers(output, &registers);
}

fn collect_registers(output: &mut PreprocessorOutput) -> HashMap<String, Vec<String>> {
  use nom::Offset;

  let mut registers = HashMap::new();

  'fileloop: for (_filename, file) in &output.source_files_content {
    loop {
      let content = file.content.borrow();
      let content_ref = content.as_str();

      let Ok((start, _)) = Register::parse_find_register(content_ref) else {
        // no @register was found
        continue 'fileloop;
      };

      let start_idx = content_ref.offset(start);

      match Register::parse(start) {
        Ok((new_i, register)) => {
          registers
            .entry(register.name.to_owned())
            .or_insert(Vec::new())
            .push(register.body.to_owned());

          let end_idx = content_ref.offset(new_i);
          std::mem::drop(content);

          // remove the register code from the file
          file
            .content
            .borrow_mut()
            .replace_range(start_idx..end_idx, "");
        }
        Err(e) => {
          println!("ERROR parsing @register: {e}");
          continue 'fileloop;
        }
      };
    }
  }

  registers
}

fn emit_registers(output: &mut PreprocessorOutput, registers: &HashMap<String, Vec<String>>) {
  // memorize which register was used to emit a log about the unused ones
  let mut used_registers: HashSet<String> = HashSet::new();

  for (_filename, file) in &output.source_files_content {
    'parsing_loop: loop {
      let content = file.content.borrow();
      let content_ref = content.as_str();

      let Ok((start, _)) = RegisterEmitter::parse_find_register(content_ref) else {
        // no @registry was found
        break 'parsing_loop;
      };

      let start_idx = content_ref.offset(start);

      match RegisterEmitter::parse(start) {
        Ok((new_i, register_emitter)) => {
          let end_idx = content_ref.offset(new_i);

          // replace the registerEmitter's code with the emitted code
          let output = register_emitter.emit(registers, &mut used_registers);

          std::mem::drop(content);
          file
            .content
            .borrow_mut()
            .replace_range(start_idx..end_idx, &output);
        }
        Err(e) => {
          println!("ERROR parsing @register: {e}");
          break 'parsing_loop;
        }
      };
    }
  }

  for register in registers.keys() {
    if !used_registers.contains(register) {
      println!("register [{register}] defined but unused, no matching @registry found.");

      if let Some(values) = registers.get(register) {
        println!("  values:");

        for value in values {
          println!("  - {value}");
        }
      }
    }
  }
}

trait RegisterParser<'a>
where
  Self: Sized
{
  fn prefix() -> &'static str;
  fn from_name_and_body(name: &'a str, body: &'a str) -> Self;

  fn parse(start: &'a str) -> IResult<&'a str, Self> {
    let (i, _) = tag(Self::prefix())(start)?;
    let (i, _) = char('(')(i)?;
    let (i, name) = delimited(tag("'"), take_until1("'"), tag("'"))(i.trim_start())?;
    let (i, _) = char(',')(i.trim_start())?;
    let (i, body) = Self::parse_body(i.trim_start())?;

    Ok((i, Self::from_name_and_body(name, body)))
  }

  fn parse_find_register(i: &'a str) -> IResult<&'a str, ()> {
    let (i, _) = take_until1(Self::prefix())(i)?;

    Ok((i, ()))
  }

  fn parse_body(i: &'a str) -> IResult<&'a str, &'a str> {
    if i.starts_with("{{") {
      let (i, _) = tag("{{")(i)?;
      let (i, body) = take_until1("}}")(i)?;
      let (i, _) = tag("}}")(i)?;
      let (i, _) = tag(")")(i.trim())?;

      Ok((i, body))
    } else {
      let (i, body) = take_until1(")")(i)?;
      let (i, _) = tag(")")(i)?;

      Ok((i, body))
    }
  }
}

struct Register<'a> {
  pub name: &'a str,
  pub body: &'a str
}

impl<'a> RegisterParser<'a> for Register<'a> {
  fn prefix() -> &'static str {
    "@register"
  }

  fn from_name_and_body(name: &'a str, body: &'a str) -> Self {
    Self { name, body }
  }
}

struct RegisterEmitter<'a> {
  pub name: &'a str,
  pub body: &'a str
}

impl<'a> RegisterParser<'a> for RegisterEmitter<'a> {
  fn prefix() -> &'static str {
    "@registry"
  }

  fn from_name_and_body(name: &'a str, body: &'a str) -> Self {
    Self { name, body }
  }
}

impl<'a> RegisterEmitter<'a> {
  pub fn emit(
    &self, registers: &HashMap<String, Vec<String>>, used_register: &mut HashSet<String>
  ) -> String {
    let Some(values) = registers.get(self.name) else {
      return String::new();
    };

    let mut output = String::new();

    for value in values {
      output.push_str(&self.body.replace("REGISTER", value));
    }

    used_register.insert(self.name.to_owned());

    output
  }
}
