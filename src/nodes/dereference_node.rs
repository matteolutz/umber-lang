use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct DereferenceNode {
    node: Box<dyn Node>,
}

impl DereferenceNode {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self { node }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }

}

impl NodeToAny for DereferenceNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for DereferenceNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<DereferenceNode>[Node: {}]", self.node)
    }
}

impl Node for DereferenceNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Dereference
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}