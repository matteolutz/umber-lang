use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct NumberNode {
    token: Token,
    size: Box<dyn ValueType>,
}

impl NumberNode {
    pub fn new(token: Token, size: Box<dyn ValueType>) -> Self {
        NumberNode { token, size }
    }

    pub fn get_number(&self) -> u64 {
        self.token
            .token_value()
            .as_ref()
            .unwrap()
            .parse::<u64>()
            .unwrap()
    }

    pub fn size(&self) -> &Box<dyn ValueType> {
        &self.size
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

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
