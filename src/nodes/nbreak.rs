use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;
use crate::token::Token;

pub struct BreakNode {
    pos_start: Position,
    pos_end: Position
}

impl BreakNode {

    pub fn new(pos_start: Position, pos_end: Position) -> Self {
        BreakNode {
            pos_start,
            pos_end
        }
    }

}

impl Display for BreakNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BreakNode>[]")
    }
}

impl Node for BreakNode {

    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

}