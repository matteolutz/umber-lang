use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;

pub struct ContinueNode {
    pos_start: Position,
    pos_end: Position
}

impl ContinueNode {

    pub fn new(pos_start: Position, pos_end: Position) -> Self {
        ContinueNode {
            pos_start,
            pos_end
        }
    }

}

impl Display for ContinueNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ContinueNode>[]")
    }
}

impl Node for ContinueNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }
}