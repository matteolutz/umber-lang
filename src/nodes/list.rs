use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct ListNode {
    element_nodes: Vec<Box<dyn Node>>,
    pos_start: Position,
    pos_end: Position
}

impl ListNode {

    pub fn new(element_nodes: Vec<Box<dyn Node>>, pos_start: Position, pos_end: Position) -> Self {
        ListNode {
            element_nodes,
            pos_start,
            pos_end
        }
    }

}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ListNode>[{}]", self.element_nodes.iter().map(|el| format!("{}", el)).collect::<Vec<String>>().join(","))
    }
}

impl NodeToAny for ListNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ListNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::List
    }
}