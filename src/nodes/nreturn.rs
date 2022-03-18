use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct ReturnNode {
    node_to_return: Option<Box<dyn Node>>,
    pos_start: Position,
    pos_end: Position
}

impl ReturnNode {

    pub fn new(node_to_return: Option<Box<dyn Node>>, pos_start: Position, pos_end: Position) -> Self {
        ReturnNode {
            node_to_return,
            pos_start,
            pos_end
        }
    }

    pub fn node_to_return(&self) -> &Option<Box<dyn Node>> { &self.node_to_return }

}

impl Display for ReturnNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.node_to_return.is_some() {
            write!(f, "<ReturnNode>[Node: {}]", self.node_to_return.as_ref().unwrap())
        } else {
            write!(f, "<ReturnNode>[]")
        }
    }
}

impl NodeToAny for ReturnNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ReturnNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Return
    }
}