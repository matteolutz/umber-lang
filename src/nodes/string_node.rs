use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;

#[derive(Clone)]
pub struct StringNode {
    token: Token,
}

impl StringNode {

    pub fn new(token: Token) -> Self {
        StringNode {
            token
        }
    }

    pub fn get_string(&self) -> String {
        self.token.token_value().as_ref().unwrap().clone()
    }

}

impl Display for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StringNode>[Token: {}]", self.token)
    }
}

impl NodeToAny for StringNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for StringNode {
    fn pos_start(&self) -> &Position {
        &self.token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.token.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::String
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}