use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct WhileNode {
    condition_node: Box<dyn Node>,
    body_node: Box<dyn Node>,
    should_return_null: bool
}

impl WhileNode {

    pub fn new(condition_node: Box<dyn Node>, body_node: Box<dyn Node>, should_return_null: bool) -> Self {
        WhileNode {
            condition_node,
            body_node,
            should_return_null,
        }
    }

}

impl Display for WhileNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<WhileNode>[Cond.: {}, Body: {}, ReturnNull?: {}]", self.condition_node, self.body_node, self.should_return_null)
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
}