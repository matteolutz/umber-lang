use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct OffsetNode {
    node: Box<dyn Node>,
    offset_node: Box<dyn Node>,
    pointee_type: Box<dyn ValueType>
}

impl OffsetNode {
    pub fn new(node: Box<dyn Node>, offset_node: Box<dyn Node>, pointee_type: Box<dyn ValueType>) -> Self {
        Self {
            node,
            offset_node,
            pointee_type
        }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
    pub fn offset_node(&self) -> &Box<dyn Node> {
        &self.offset_node
    }
    pub fn pointee_type(&self) -> &Box<dyn ValueType> {
        &self.pointee_type
    }

}

impl NodeToAny for OffsetNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for OffsetNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<OffsetNode>")
    }
}

impl Node for OffsetNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.offset_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Offset
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}