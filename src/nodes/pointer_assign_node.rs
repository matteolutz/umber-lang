use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct PointerAssignNode {
    assign_to: Box<dyn Node>,
    assign_node: Box<dyn Node>
}

impl PointerAssignNode {
    pub fn new(assign_to: Box<dyn Node>, assign_node: Box<dyn Node>) -> Self {
        Self {
            assign_to,
            assign_node
        }
    }

    pub fn assign_to(&self) -> &Box<dyn Node> {
        &self.assign_to
    }
    pub fn assign_node(&self) -> &Box<dyn Node> {
        &self.assign_node
    }
}

impl NodeToAny for PointerAssignNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for PointerAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PointerAssign>[To: {}, From: {}]", self.assign_to, self.assign_node)
    }
}

impl Node for PointerAssignNode {
    fn pos_start(&self) -> &Position {
        self.assign_to.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.assign_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::PointerAssign
    }
}