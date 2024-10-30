use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct ContinueNode {
    pos_start: Position,
    pos_end: Position,
}

impl ContinueNode {
    pub fn new(pos_start: Position, pos_end: Position) -> Self {
        ContinueNode { pos_start, pos_end }
    }
}

impl Display for ContinueNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "continue")
    }
}

impl NodeToAny for ContinueNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ContinueNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Continue
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
