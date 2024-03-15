use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct VarDeclarationNode {
    var_name: String,
    var_type: Box<dyn ValueType>,
    value_node: Box<dyn Node>,
    is_mutable: bool,
    pos_start: Position,
}

impl VarDeclarationNode {
    pub fn new(
        var_name: String,
        var_type: Box<dyn ValueType>,
        value_node: Box<dyn Node>,
        is_mutable: bool,
        pos_start: Position,
    ) -> Self {
        VarDeclarationNode {
            var_name,
            var_type,
            value_node,
            is_mutable,
            pos_start,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn var_type(&self) -> &Box<dyn ValueType> {
        &self.var_type
    }
    pub fn value_node(&self) -> &Box<dyn Node> {
        &self.value_node
    }
    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }
    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }
}

impl Display for VarDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<VarDeclarationNode>[Name: {}, Type: {}, Value: {}, IsMutable?: {}]",
            self.var_name, self.var_type, self.value_node, self.is_mutable
        )
    }
}

impl NodeToAny for VarDeclarationNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for VarDeclarationNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::VarDeclaration
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
