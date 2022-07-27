/// Data passed to child node of the AST while emitting.
/// Can be used to give some additional information to the child nodes so they
/// can emit different structures.
pub struct EmitAdditionalData {
  parent_node: Option<NodeType>,
}

/// It is not meant to be an exhaustive list, if you need more node types for
/// your needs then you can add them. All the node types you see currently are
/// the ones we need and that's it.
pub enum NodeType {
  Struct,
}
