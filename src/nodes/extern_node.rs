use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct ExternNode {
    top_level_statement: Box<dyn Node>,
    pos_start: Position,
    pos_end: Position,
}

impl ExternNode {
    pub fn new(top_level_statement: Box<dyn Node>, pos_start: Position, pos_end: Position) -> Self {
        Self {
            top_level_statement,
            pos_start,
            pos_end,
        }
    }

    pub fn top_level_statement(&self) -> &Box<dyn Node> {
        &self.top_level_statement
    }
}

impl NodeToAny for ExternNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ExternNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "extern ({})", self.top_level_statement)
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
