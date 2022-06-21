
pub struct FunctionVisitor {

}

impl super::Visitor for FunctionVisitor {
    fn visit_function_declaration(&mut self, node: &mut crate::ast::FunctionDeclaration) {
        println!("visitor: {:?}", node.name);
    }

    fn visitor_type(&self) -> super::VisitorType {
        super::VisitorType::FunctionDeclarationVisitor
    }
}