use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;

pub struct NumberNode {
    token: Token,
}

impl NumberNode {

    pub fn new(token: Token) -> Self {
        NumberNode {
            token
        }
    }

}

impl Display for NumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NumberNode>[Token: {}]", self.token)
    }
}

impl NodeToAny for NumberNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for NumberNode {
    fn pos_start(&self) -> &Position {
        &self.token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.token.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Number
    }
}