use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct ImportNode {
    node: Box<dyn Node>,
}

impl ImportNode {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self { node }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
}

impl NodeToAny for ImportNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ImportNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "import ({})", self.node)
    }
}

impl Node for ImportNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Import
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
