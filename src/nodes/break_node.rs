use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct BreakNode {
    pos_start: Position,
    pos_end: Position
}

impl BreakNode {

    pub fn new(pos_start: Position, pos_end: Position) -> Self {
        BreakNode {
            pos_start,
            pos_end
        }
    }

}

impl Display for BreakNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BreakNode>[]")
    }
}

impl NodeToAny for BreakNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for BreakNode {

    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Break
    }
}