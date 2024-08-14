use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{ParameterType, Span};

/// TODO: the store only holds Strings, this means a lot of allocations since
/// the nodes also hold the strings. Ideally the store would only store
//&/ references as we know its lifetime is shorter than the AST itself.
#[derive(Debug)]
pub struct TypeInferenceStore {
  pub types: TypeInferenceMap
}

impl TypeInferenceStore {
  pub fn new() -> Self {
    let mut map = HashMap::new();

    map.insert("int".to_string(), Rc::new(InferedType::Scalar));
    map.insert("array".to_string(), Rc::new(InferedType::Unknown));
    map.insert("float".to_string(), Rc::new(InferedType::Scalar));
    map.insert("string".to_string(), Rc::new(InferedType::Scalar));
    map.insert("name".to_string(), Rc::new(InferedType::Scalar));

    Self { types: map }
  }

  pub fn register_compound(&mut self, name: String, extends: Option<String>) -> Result<(), String> {
    if self.types.contains_key(&name) {
      return Err(format!("compound type {} was registered twice", &name));
    }

    let compound = Rc::new(InferedType::Compound {
      type_inference_map: RefCell::new(HashMap::new()),
      extends
    });

    self.types.insert(name, compound.clone());

    Ok(())
  }

  pub fn register_function(
    &mut self, name: String, parameters: Vec<FunctionInferedParameterType>,
    return_type: Option<String>, span: Span
  ) -> Result<(), String> {
    if self.types.contains_key(&name) {
      return Err(format!("function {} was registered twice", &name));
    }

    let function = Rc::new(InferedType::Function(Rc::new(FunctionInferedType {
      parameters,
      return_type,
      span
    })));
    self.types.insert(name, function.clone());

    Ok(())
  }

  pub fn register_method(
    &mut self, parent_compound_name: String, name: String,
    parameters: Vec<FunctionInferedParameterType>, return_type: Option<String>, span: Span
  ) -> Result<(), String> {
    let mut result = Ok(());

    self
      .types
      .entry(parent_compound_name)
      .and_modify(|class_type| {
        match &**class_type {
          InferedType::Compound {
            type_inference_map,
            extends: _
          } => {
            let mut class = type_inference_map.borrow_mut();

            if class.contains_key(&name) {
              result = Err(format!("method {name} was registered twice"));
            }

            let method = Rc::new(InferedType::Function(Rc::new(FunctionInferedType {
              parameters,
              return_type,
              span
            })));
            class.insert(name, method.clone());
          }
          _ => {}
        };
      });

    result
  }
}

pub type TypeInferenceMap = HashMap<String, Rc<InferedType>>;

#[derive(Debug)]
pub enum InferedType {
  /// Primitive types, or types that hold a single value
  Scalar,

  /// Structs, classes, types that hold multiple values
  ///
  /// The TypeInferenceMap it holds is for its methods
  Compound {
    type_inference_map: RefCell<TypeInferenceMap>,

    /// In case the compound type extend another type, this value is set to
    /// `Some(base_type_identifier)`
    extends: Option<String>
  },

  Function(Rc<FunctionInferedType>),

  Lambda(Rc<FunctionInferedType>),

  /// For unknown types, coming from a different source,
  /// such as the game sources.
  Unknown
}

#[derive(Debug)]
pub struct FunctionInferedType {
  pub parameters: Vec<FunctionInferedParameterType>,
  pub return_type: Option<String>,
  pub span: Span
}

#[derive(Debug)]
pub struct FunctionInferedParameterType {
  pub parameter_type: ParameterType,

  /// Obtained using
  /// ```
  /// TypeDeclaration::to_string()
  /// ```
  pub infered_type: String,
  pub span: Span
}
