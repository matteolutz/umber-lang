use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct AddressOfNode {
    var_name: String,
    pos_start: Position,
    pos_end: Position
}

impl AddressOfNode {
    pub fn new(var_name: String, pos_start: Position, pos_end: Position) -> Self {
        Self {
            var_name,
            pos_start,
            pos_end
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
}

impl NodeToAny for AddressOfNode {
    fn as_any(&self) -> &dyn Any {
       self
    }
}

impl Display for AddressOfNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddressOfNode")
    }
}

impl Node for AddressOfNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::AddressOf
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}