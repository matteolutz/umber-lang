use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct IgnoredNode {
    pos_start: Position,
    pos_end: Position,
}

impl IgnoredNode {
    pub fn new(pos_start: Position, pos_end: Position) -> Self {
        Self { pos_start, pos_end }
    }
}

impl NodeToAny for IgnoredNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for IgnoredNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IgnoredNode")
    }
}

impl Node for IgnoredNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Ignored
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
