use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;
use crate::token::Token;

pub struct BinOpNode {
    left_node: Box<dyn Node>,
    op_token: Token,
    right_node: Box<dyn Node>,
}

impl BinOpNode {
    pub fn new(left_node: Box<dyn Node>, op_token: Token, right_node: Box<dyn Node>) -> Self {
        BinOpNode {
            right_node,
            op_token,
            left_node
        }
    }
}

impl Display for BinOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BinOpNode>[Left: {}, Op: {}, Right: {}]", self.left_node, self.op_token, self.right_node)
    }
}

impl Node for BinOpNode {
    fn pos_start(&self) -> &Position {
        self.left_node.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.right_node.pos_end()
    }
}