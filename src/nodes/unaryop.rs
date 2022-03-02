use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;
use crate::token::Token;

pub struct UnaryOpNode {
    op_token: Token,
    node: Box<dyn Node>,
}

impl UnaryOpNode {
    pub fn new(op_token: Token, node: Box<dyn Node>) -> Self {
        UnaryOpNode {
            op_token,
            node,
        }
    }
}

impl Display for UnaryOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<UnaryOpNode>[Op: {}, Node: {}]", self.op_token, self.node)
    }
}

impl Node for UnaryOpNode {
    fn pos_start(&self) -> &Position {
        self.op_token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.node.pos_end()
    }
}