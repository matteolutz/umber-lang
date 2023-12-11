use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct StackAllocationNode {
    size_in_bytes: u64,
    pos_start: Position,
    pos_end: Position
}

impl StackAllocationNode {
    pub fn new(size_in_bytes: u64, pos_start: Position, pos_end: Position) -> Self {
        Self {
            size_in_bytes,
            pos_start,
            pos_end
        }
    }

    pub fn size_in_bytes(&self) -> &u64 {
        &self.size_in_bytes
    }
}


impl Display for StackAllocationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StackAllocationNode")
    }
}

impl NodeToAny for StackAllocationNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for StackAllocationNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::StackAllocationNode
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
