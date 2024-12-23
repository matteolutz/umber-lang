use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_size::ValueSize;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct ReadBytesNode {
    node: Box<dyn Node>,
    bytes: ValueSize,
    pos_end: Position,
}

impl ReadBytesNode {
    pub fn new(node: Box<dyn Node>, bytes: ValueSize, pos_end: Position) -> Self {
        Self {
            node,
            bytes,
            pos_end,
        }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
    pub fn bytes(&self) -> &ValueSize {
        &self.bytes
    }
}

impl NodeToAny for ReadBytesNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ReadBytesNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadBytesNode")
    }
}

impl Node for ReadBytesNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::ReadBytes
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
