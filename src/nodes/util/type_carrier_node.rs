use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct TypeCarrierNode {
    carried_type: Box<dyn ValueType>,
    pos_start: Position,
    pos_end: Position
}

impl TypeCarrierNode {
    pub fn new(carried_type: Box<dyn ValueType>) -> Self {
        Self {
            carried_type,
            pos_start: Position::empty(),
            pos_end: Position::empty()
        }
    }

    pub fn carried_type(&self) -> &Box<dyn ValueType> {
        &self.carried_type
    }
}

impl NodeToAny for TypeCarrierNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for TypeCarrierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TypeCarrierNode>")
    }
}

impl Node for TypeCarrierNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::TypeCarrier
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}