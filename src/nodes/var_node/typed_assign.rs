use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct VarTypedAssignNode {
    var_name: String,
    value_node: Box<dyn Node>,
    value_type: Box<dyn ValueType>,
    pos_start: Position,
}

impl VarTypedAssignNode {
    pub fn new(
        var_name: String,
        value_node: Box<dyn Node>,
        value_type: Box<dyn ValueType>,
        pos_start: Position,
    ) -> Self {
        Self {
            var_name,
            value_node,
            value_type,
            pos_start,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn value_node(&self) -> &Box<dyn Node> {
        &self.value_node
    }
    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }
    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }
}

impl Display for VarTypedAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.var_name, self.value_node)
    }
}

impl NodeToAny for VarTypedAssignNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for VarTypedAssignNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::VarTypedAssign
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
