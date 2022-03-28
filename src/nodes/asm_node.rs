use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct AssemblyNode {
    content: String,
    pos_start: Position,
    pos_end: Position,
}

impl AssemblyNode {
    pub fn new(content: String, pos_start: Position, pos_end: Position) -> Self {
        Self {
            content,
            pos_start,
            pos_end,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl NodeToAny for AssemblyNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for AssemblyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<AssemblyNode>[\"{}\"]", &self.content)
    }
}

impl Node for AssemblyNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Assembly
    }
}