use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum Type {
  String,
  Name,
  Bool,
  Int,
  Float,
  Identifier(String),
  Void,
  Unknown
}

// impl ToString for Type {
//     fn to_string(&self) -> String {
//       match self {
//         Type::String => "string".to_string(),
//         Type::Name => "name".to_string(),
//         Type::Bool => "bool".to_string(),
//         Type::Int => "int".to_string(),
//         Type::Float => "float".to_string(),
//         Type::Identifier(s) => s.clone(),
//         Type::Void => "void".to_string(),
//         Type::Unknown => "unknown".to_string(),
//     }
//   }
// }

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Type::String => "string",
      Type::Name => "name",
      Type::Bool => "bool",
      Type::Int => "int",
      Type::Float => "float",
      Type::Identifier(x) => x,
      Type::Void => "void",
      Type::Unknown => "Unknown",
    })
  }
}

impl Type {
  pub fn equals_string(&self, other: &str) -> bool {
    match self {
      Type::String => other == "string",
      Type::Name => other == "name",
      Type::Bool => other == "bool",
      Type::Int => other == "int",
      Type::Float => other == "float",
      Type::Identifier(x) => x == other,
      Type::Void => other == "void",
      Type::Unknown => false,
    }
  }

  pub fn can_auto_cast(&self, other: &str) -> bool {
    match (self, other) {
      (Type::Name, "string") => true,
      (Type::Float, "int") => true,
      (Type::Int, "float") => true,
      _ => false
    }
  }
}
