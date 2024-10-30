use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct AccessorNode {
    node: Box<dyn Node>,
    accessor: String,
    pos_end: Position,
}

impl AccessorNode {
    pub fn new(node: Box<dyn Node>, accessor: String, pos_end: Position) -> Self {
        Self {
            node,
            accessor,
            pos_end,
        }
    }

    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }
    pub fn accessor(&self) -> &str {
        &self.accessor
    }
}

impl NodeToAny for AccessorNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for AccessorNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}).({})", self.node, self.accessor)
    }
}

impl Node for AccessorNode {
    fn pos_start(&self) -> &Position {
        self.node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Accessor
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
