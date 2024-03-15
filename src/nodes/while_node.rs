use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct WhileNode {
    condition_node: Box<dyn Node>,
    body_node: Box<dyn Node>,
}

impl WhileNode {
    pub fn new(condition_node: Box<dyn Node>, body_node: Box<dyn Node>) -> Self {
        WhileNode {
            condition_node,
            body_node,
        }
    }

    pub fn condition_node(&self) -> &Box<dyn Node> {
        &self.condition_node
    }

    pub fn body_node(&self) -> &Box<dyn Node> {
        &self.body_node
    }
}

impl Display for WhileNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<WhileNode>[Cond.: {}, Body: {}]",
            self.condition_node, self.body_node
        )
    }
}

impl NodeToAny for WhileNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for WhileNode {
    fn pos_start(&self) -> &Position {
        self.condition_node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.body_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::While
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
