use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct CastNode {
    node: Box<dyn Node>,
    cast_type: Box<dyn ValueType>,
    pos_end: Position
}

impl CastNode {
    pub fn new(node: Box<dyn Node>, cast_type: Box<dyn ValueType>, pos_end: Position) -> Self {
        Self {
            node,
            cast_type,
            pos_end
        }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
    pub fn cast_type(&self) -> &Box<dyn ValueType> {
        &self.cast_type
    }
}

impl NodeToAny for CastNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for CastNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CastNode>[Node: {}, CastType: {}]", self.node, self.cast_type)
    }
}

impl Node for CastNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Cast
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}