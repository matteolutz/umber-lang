use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct ListNode {
    size: usize,
    element_nodes: Vec<Box<dyn Node>>,
    element_type: Box<dyn ValueType>,
    pos_start: Position,
    pos_end: Position,
}

impl ListNode {
    pub fn new(size: usize, element_nodes: Vec<Box<dyn Node>>, element_type: Box<dyn ValueType>, pos_start: Position, pos_end: Position) -> Self {
        ListNode {
            size,
            element_nodes,
            element_type,
            pos_start,
            pos_end,
        }
    }

    pub fn size(&self) -> &usize {
        &self.size
    }
    pub fn element_nodes(&self) -> &Vec<Box<dyn Node>> { &self.element_nodes }
    pub fn element_type(&self) -> &Box<dyn ValueType> { &self.element_type }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ListNode>[{}, Type: {}]", self.element_nodes.iter().map(|el| format!("{}", el)).collect::<Vec<String>>().join(","), self.element_type)
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

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}