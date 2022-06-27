use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::FunctionDeclaration;

#[derive(Debug)]
pub struct Context {
  /// Mostly for debugging purposes
  pub name: String,

  pub identifiers: HashMap<String, String>,

  pub children_contexts: Vec<Rc<RefCell<Context>>>,
  pub parent_context: Option<Rc<RefCell<Context>>>,

  /// Stores the generic variables in the context and what they should translate
  /// to.
  pub generic_context: Option<GenericContext>,
}

impl Context {
  pub fn new(name: &str, generic_types: Option<Vec<GenericType>>) -> Self {
    Self {
      name: name.to_string(),
      identifiers: HashMap::new(),
      children_contexts: Vec::new(),
      parent_context: None,
      generic_context: generic_types.and_then(|t| Some(GenericContext::new(t))),
    }
  }

  pub fn set_parent_context(this: &Rc<RefCell<Context>>, parent: &Rc<RefCell<Context>>) {
    if let Some(parent_context) = &(*this).borrow().parent_context {
      Self::remove_child(parent_context, &this);
    }

    (*parent).borrow_mut().children_contexts.push(this.clone());
    (*this).borrow_mut().parent_context = Some(parent.clone());
  }

  pub fn remove_child(this: &Rc<RefCell<Context>>, child: &Rc<RefCell<Context>>) {
    let index = (*this)
      .borrow()
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
    if let Some(context) = &this.borrow().parent_context {
      return Self::get_top_most_context(&context);
    }

    this.clone()
  }

  pub fn find_global_function_declaration(this: &Rc<RefCell<Context>>, name: &str) -> Option<Rc<RefCell<Context>>> {
    let program = Self::get_top_most_context(this);
    let context_name = format!("function: {}", name);

    let result = program.borrow().children_contexts.iter()
      .find(|context| context.borrow().name == context_name)
      .and_then(|c| Some(c.clone()));

    result
  }

  pub fn register_generic_call(&mut self, types: &Vec<String>) {
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
  }

  /// If the passed identifier is a generic type with a resolved value, get
  /// the resolved type in return. If not or if there is no match then return
  /// the unchanged identifier that was passed as a parameter.
  pub fn transform_if_generic_type(
    &self,
    f: &mut std::fmt::Formatter<'_>,
    identifier: &str,
  ) -> std::fmt::Result {
    let res = self
      .generic_context
      .as_ref()
      .and_then(|generic_context| generic_context.transform_if_generic_type(f, identifier));

    let should_call_parent = !res.and_then(|result| Some(result.is_ok())).unwrap_or(false);

    if !should_call_parent {
      return Ok(());
    }

    match &self.parent_context {
      Some(parent) => (*parent).borrow().transform_if_generic_type(f, identifier),
      None => write!(f, "{identifier}"),
    }
  }

  pub fn print(&self, depth: usize) {
    println!("{}{}", "  ".repeat(depth), self.name);

    for child in &self.children_contexts {
      (*child).borrow().print(depth + 1);
    }
  }
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
      currently_used_variant: None
    }
  }

  pub fn add_generic_variant(&mut self, types: HashMap<GenericType, ResolvedGenericType>) {
    // the identifier is just the concatenation of all types used in the variant
    let identifier = types.values().map(|s| s.to_string()).collect::<String>();

    println!("adding generic call variant {identifier}");

    // we already have the variant in the map
    if self.translation_variants.contains_key(&identifier) {
      return;
    }

    if self.is_variant_valid(&types) {
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
}

impl<'a> GenericContext {
  pub fn transform_if_generic_type(
    &'a self,
    f: &mut std::fmt::Formatter<'_>,
    identifier: &str,
  ) -> Option<std::fmt::Result> {
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
