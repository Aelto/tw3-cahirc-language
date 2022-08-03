use std::collections::{HashMap};

/// TODO: the store only holds Strings, this means a lot of allocations since
/// the no&des also hold the strings. Ideally the store would only store
//&/ references as we know its lifetime is shorter than the AST itself.
/// 
/// TODO 2: do a final pass over all the variables and verify the types
/// they have are either unknown or correct.
#[derive(Debug)]
pub struct TypeInferenceStore {
  pub types: TypeInferenceMap,
}

impl TypeInferenceStore {
  pub fn new() -> Self {
    let mut map = HashMap::new();

    map.insert("int".to_string(), InferedType::Scalar);
    map.insert("array".to_string(), InferedType::Unknown);
    map.insert("float".to_string(), InferedType::Scalar);
    map.insert("string".to_string(), InferedType::Scalar);
    map.insert("name".to_string(), InferedType::Scalar);

    Self {
      types: map
    }
  }

  pub fn register_compound(&mut self, name: String) {
    println!("registering compound type {}", &name);

    if self.types.contains_key(&name) {
      println!("warning: compound type {} was registered twice", &name);
    }

    self.types.insert(name, InferedType::Compound(HashMap::new()));
  }

  pub fn register_function(&mut self, name: String, parameters: Vec<String>, return_type: Option<String>) {
    println!("registering function type {}({:?}): {:?}", &name, &parameters, &return_type);

    if self.types.contains_key(&name) {
      println!("warning: function {} was registered twice", &name);
    }

    // TODO: store the return type
    self.types.insert(name, InferedType::Function { parameters, return_type });
  }

  pub fn register_method(&mut self, parent_compound_name: String, name: String, parameters: Vec<String>, return_type: Option<String>) {
    println!("registering method {parent_compound_name}::{name}({:?}): {:?}", &parameters, &return_type);

    self.types.entry(parent_compound_name).and_modify(|class_type| {
        match class_type {
          InferedType::Compound(ref mut class) => {
            if class.contains_key(&name) {
              println!("warning: method {name} was registered twice");
            }
            
            // TODO: store the return type
            class.insert(name, InferedType::Function { parameters, return_type });
          },
          _ => {}
        };
    });
  }
}

pub type TypeInferenceMap = HashMap<String, InferedType>;

#[derive(Debug)]
pub enum InferedType {
  /// Primitive types, or types that hold a single value
  Scalar,

  /// Structs, classes, types that hold multiple values
  /// 
  /// The TypeInferenceMap it holds is for its methods
  Compound(TypeInferenceMap),

  /// The vector of string it holds is for the parameters
  /// of the function. It is the string representation of
  /// this types. Can be obtained using
  /// ```
  /// TypedDeclaration::to_string()
  /// ```
  Function{
    parameters: Vec<String>,
    return_type: Option<String>
  },

  /// For unknown types, coming from a different source,
  /// such as the game sources.
  Unknown,
}
