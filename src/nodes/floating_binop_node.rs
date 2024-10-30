use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;

#[derive(Clone)]
pub struct FloatingBinOpNode {
    left_node: Box<dyn Node>,
    op_token: Token,
    right_node: Box<dyn Node>,
}

impl FloatingBinOpNode {
    pub fn new(left_node: Box<dyn Node>, op_token: Token, right_node: Box<dyn Node>) -> Self {
        Self {
            right_node,
            op_token,
            left_node,
        }
    }

    pub fn left_node(&self) -> &Box<dyn Node> {
        &self.left_node
    }
    pub fn op_token(&self) -> &Token {
        &self.op_token
    }
    pub fn right_node(&self) -> &Box<dyn Node> {
        &self.right_node
    }
}

impl Display for FloatingBinOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.left_node, self.op_token, self.right_node
        )
    }
}

impl NodeToAny for FloatingBinOpNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for FloatingBinOpNode {
    fn pos_start(&self) -> &Position {
        self.left_node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.right_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::FloatingBinOp
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
