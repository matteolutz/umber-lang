use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct StructInitNode {
    struct_name: String,
    pos_start: Position,
    pos_end: Position,
}

impl StructInitNode {
    pub fn new(struct_name: String, pos_start: Position, pos_end: Position) -> Self {
        Self {
            struct_name,
            pos_start,
            pos_end,
        }
    }

    pub fn struct_name(&self) -> &str {
        &self.struct_name
    }
}

impl Display for StructInitNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {} {{}}", self.struct_name)
    }
}

impl NodeToAny for StructInitNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for StructInitNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::StructInit
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
