use std::borrow::Borrow;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::TypedIdentifier;

#[derive(Debug)]
pub struct Context {
  /// Mostly for debugging purposes
  pub name: String,

  pub context_type: ContextType,

  pub identifiers: HashMap<String, String>,

  pub children_contexts: Vec<Rc<RefCell<Context>>>,
  pub parent_context: Option<Rc<RefCell<Context>>>,

  /// Stores the generic variables in the context and what they should translate
  /// to.
  pub generic_context: Option<GenericContext>,

  pub is_library: bool,
  pub mangled_accessor: Option<String>,

  /// Stores the variable declarations in the context. To be able to emit them
  /// at the start of the functions/classes/structs
  pub variable_declarations: Vec<Rc<TypedIdentifier>>,

  /// Stores for the identifiers (used as the keys) the infered types (used as
  /// values).
  /// 
  /// This map stores the variables used in the local context and their infered
  /// types. For compound types, a lookup into the TypeInferenceStore may be
  /// needed. The key is the name of the variable, the value is the way to
  /// identify the infered type. If you'd pass the value to the TypeInferenceStore,
  /// it would return you the actual object representation of the type.
  pub local_variables_inference: HashMap<String, String>,
}

impl Context {
  pub fn new(
    name: &str, generic_types: Option<Vec<GenericType>>, context_type: ContextType,
  ) -> Self {
    Self {
      name: name.to_string(),
      context_type,
      identifiers: HashMap::new(),
      children_contexts: Vec::new(),
      parent_context: None,
      generic_context: generic_types.and_then(|t| Some(GenericContext::new(t))),
      is_library: false,
      mangled_accessor: None,
      variable_declarations: Vec::new(),
      local_variables_inference: HashMap::new()
    }
  }

  pub fn set_as_library(&mut self) {
    self.is_library = true;
    self.mangled_accessor = Some(format!(
      "wss{}",
      uuid::Uuid::new_v4().to_string().replace("-", "")
    ));
  }

  pub fn get_class_name(&self) -> Option<String> {
    if self.name.starts_with("class: ") {
      Some(self.name.replacen("class: ", "", 1))
    }
    else {
      None
    }
  }

  pub fn set_parent_context(this: &Rc<RefCell<Context>>, parent: &Rc<RefCell<Context>>) {
    if let Some(parent_context) = &Self::get_ref(this).parent_context {
      Self::remove_child(parent_context, &this);
    }

    (*parent).borrow_mut().children_contexts.push(this.clone());
    (*this).borrow_mut().parent_context = Some(parent.clone());

    if Self::get_ref(parent).is_library {
      (*this).borrow_mut().set_as_library();
    }
  }

  pub fn remove_child(this: &Rc<RefCell<Context>>, child: &Rc<RefCell<Context>>) {
    let index = Self::get_ref(this)
      .children_contexts
      .iter()
      .position(|c| std::ptr::eq(child.as_ptr(), c.as_ptr()));

    if let Some(index) = index {
      (*this).borrow_mut().children_contexts.swap_remove(index);
    }
  }

  /// Return the top most context, the higher context in the tree with no
  /// parents.
  pub fn get_top_most_context(this: &Rc<RefCell<Context>>) -> Rc<RefCell<Context>> {
    if let Some(context) = &Self::get_ref(&this).parent_context {
      return Self::get_top_most_context(&context);
    }

    this.clone()
  }

  pub fn find_global_function_declaration(
    this: &Rc<RefCell<Context>>, name: &str,
  ) -> Option<Rc<RefCell<Context>>> {
    let program = Self::get_top_most_context(this);
    let context_name = format!("function: {}", name);

    for file_context in &Self::get_ref(&program).children_contexts {
      let result = Self::get_ref(&file_context)
        .children_contexts
        .iter()
        .find(|context| Self::get_ref(context).name == context_name)
        .and_then(|c| Some(c.clone()));

      if result.is_some() {
        return result;
      }
    }

    None
  }

  pub fn find_global_class_declaration(
    this: &Rc<RefCell<Context>>, name: &str,
  ) -> Option<Rc<RefCell<Context>>> {
    let program = Self::get_top_most_context(this);
    let context_name = format!("class: {}", name);

    for file_context in &Self::get_ref(&program).children_contexts {
      let result = Self::get_ref(&file_context)
        .children_contexts
        .iter()
        .find(|context| Self::get_ref(&context).name == context_name)
        .and_then(|c| Some(c.clone()));

      if result.is_some() {
        return result;
      }
    }

    None
  }

  /// Returns an optional mangled name the identifier should use to use the
  /// the generic type instead of the regular one.
  pub fn register_generic_call(&mut self, types: &Vec<String>) -> Option<String> {
    if let Some(context) = &mut self.generic_context {
      if context.types.len() != types.len() {
        panic!("supplied types and expected types do not match in length");
      }

      let mut variant = HashMap::new();

      for i in 0..types.len() {
        let given_type = &types[i];
        let generic_type = &context.types[i];

        variant.insert(generic_type.to_string(), given_type.to_string());
      }

      context.add_generic_variant(variant);
    }

    if self.is_library {
      return self.mangled_accessor.clone();
    }

    None
  }

  /// If the passed identifier is a generic type with a resolved value, get
  /// the resolved type in return. If not or if there is no match then return
  /// the unchanged identifier that was passed as a parameter.
  pub fn transform_if_generic_type(
    &self, f: &mut Vec<u8>, identifier: &str,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let res = self
      .generic_context
      .as_ref()
      .and_then(|generic_context| generic_context.transform_if_generic_type(f, identifier));

    let should_call_parent = !res.and_then(|result| Some(result.is_ok())).unwrap_or(false);

    if !should_call_parent {
      return Ok(());
    }

    match &self.parent_context {
      Some(parent) => Self::get_ref(parent)
        .transform_if_generic_type(f, identifier)?,
      None => {
        write!(f, "{identifier}")?;
      }
    };

    Ok(())
  }

  pub fn get_ref(context: &Rc<RefCell<Context>>) -> Ref<Context> {
    let context: &RefCell<Context> = context.borrow();
    
    context.borrow()
  }

  pub fn print(&self, depth: usize) {
    println!("{}{}", "  ".repeat(depth), self.name);

    for child in &self.children_contexts {
      Self::get_ref(&child).print(depth + 1);
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ContextType {
  Global,
  ClassOrStruct,
  Function,
}

type GenericType = String;
type ResolvedGenericType = String;
type GenericVariantIdentifier = String;

#[derive(Debug)]
pub struct GenericContext {
  /// The list of generic types the node accepts
  pub types: Vec<GenericType>,

  /// Contains the list of variant the node accepts
  pub translation_variants:
    HashMap<GenericVariantIdentifier, HashMap<GenericType, ResolvedGenericType>>,

  pub currently_used_variant: Option<GenericVariantIdentifier>,
}

impl GenericContext {
  pub fn new(types: Vec<GenericType>) -> Self {
    Self {
      types,
      translation_variants: HashMap::new(),
      currently_used_variant: None,
    }
  }

  pub fn generic_variant_suffix_from_types(types: &Vec<GenericType>) -> String {
    types.iter().map(|s| format!("_{s}")).collect::<String>()
  }

  pub fn add_generic_variant(&mut self, types: HashMap<GenericType, ResolvedGenericType>) {
    // the identifier is just the concatenation of all types used in the variant
    let identifier = types.values().map(|s| s.to_string()).collect::<String>();

    // we already have the variant in the map
    if self.translation_variants.contains_key(&identifier) {
      return;
    }

    if !self.is_variant_valid(&types) {
      return;
    }

    self.translation_variants.insert(identifier, types);
  }

  /// Returns if the given variant contains all the types this generic context
  /// needs and vice-versa. In short, returns if `types` and `self.types` both
  /// contain the same exact keys and no more.
  fn is_variant_valid(&self, types: &HashMap<GenericType, ResolvedGenericType>) -> bool {
    self.types.iter().all(|t| types.contains_key(t))
      && types.iter().all(|(key, _)| self.types.contains(key))
  }

  pub fn contains_generic_identifier(&self, identifiers: &Vec<&str>) -> bool {
    let hash: HashSet<String> = HashSet::from_iter(self.types.iter().cloned());

    identifiers.iter().any(|t| hash.contains(*t))
  }
}

impl<'a> GenericContext {
  pub fn transform_if_generic_type(
    &'a self, f: &mut Vec<u8>, identifier: &str,
  ) -> Option<Result<(), std::io::Error>> {
    use std::io::Write as IoWrite;

    let some_translation = self
      .currently_used_variant
      .as_ref()
      .and_then(|variant| self.translation_variants.get(variant))
      .and_then(|translations| translations.get(identifier));

    match some_translation {
      Some(translation) => Some(write!(f, "{translation}")),
      None => None,
    }
  }
}
