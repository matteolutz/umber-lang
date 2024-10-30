use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct F64ToU64Node {
    node: Box<dyn Node>,
}

impl F64ToU64Node {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self { node }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
}

impl NodeToAny for F64ToU64Node {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for F64ToU64Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) as u64", self.node)
    }
}

impl Node for F64ToU64Node {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::F64ToU64
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
