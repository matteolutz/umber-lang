use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;
use crate::token::Token;

pub struct StringNode {
    token: Token,
}

impl StringNode {

    pub fn new(token: Token) -> Self {
        StringNode {
            token
        }
    }

}

impl Display for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StringNode>[Token: {}]", self.token)
    }
}

impl Node for StringNode {
    fn pos_start(&self) -> &Position {
        &self.token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.token.pos_end()
    }
}