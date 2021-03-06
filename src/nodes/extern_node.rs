use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct ExternNode {
    name: String,
    pos_start: Position,
    pos_end: Position
}

impl ExternNode {
    pub fn new(name: String, pos_start: Position, pos_end: Position) -> Self {
        Self { name, pos_start, pos_end }
    }

    pub fn name(&self) -> &str { &self.name }

}

impl NodeToAny for ExternNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ExternNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ExternNode>[]")
    }
}

impl Node for ExternNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Extern
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}