use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct U64ToF64Node {
    node: Box<dyn Node>,
}

impl U64ToF64Node {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self { node }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
}

impl NodeToAny for U64ToF64Node {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for U64ToF64Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<U64ToF64Node>[Node: {}]", self.node)
    }
}

impl Node for U64ToF64Node {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::U64ToF64
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
