use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct CharNode {
    value: char,
    pos_start: Position,
    pos_end: Position,
}

impl CharNode {
    pub fn new(value: char, pos_start: Position, pos_end: Position) -> Self {
        Self {
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn value(&self) -> &char {
        &self.value
    }
}

impl NodeToAny for CharNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for CharNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CharNode>[Char: {}]", self.value)
    }
}

impl Node for CharNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Char
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}