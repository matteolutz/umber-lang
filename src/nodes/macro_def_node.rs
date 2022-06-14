use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct MacroDefNode {
    pos_start: Position,
    pos_end: Position
}

impl MacroDefNode {
    pub fn new(pos_start: Position, pos_end: Position) -> Self { Self {pos_start, pos_end} }
}

impl NodeToAny for MacroDefNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for MacroDefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<MacroDefNode>[]")
    }
}

impl Node for MacroDefNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::MacroDef
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}