use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct SizeOfNode {
    value_type: Box<dyn ValueType>,
    pos_start: Position,
    pos_end: Position,
}

impl SizeOfNode {
    pub fn new(value_type: Box<dyn ValueType>, pos_start: Position, pos_end: Position) -> Self {
        Self {
            value_type,
            pos_start,
            pos_end,
        }
    }

    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }
}

impl NodeToAny for SizeOfNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for SizeOfNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SizeOfNode>[Type: {}]", self.value_type)
    }
}

impl Node for SizeOfNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::SizeOf
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
