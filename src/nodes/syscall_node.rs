use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct SyscallNode {
    args: Vec<Box<dyn Node>>,
    pos_start: Position,
    pos_end: Position,
}

impl SyscallNode {
    pub fn new(args: Vec<Box<dyn Node>>, pos_start: Position, pos_end: Position) -> Self {
        Self {
            args,
            pos_start,
            pos_end
        }
    }

    pub fn args(&self) -> &Vec<Box<dyn Node>> {
        &self.args
    }
}

impl NodeToAny for SyscallNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for SyscallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SyscallNode>[Args: {}]", self.args.len())
    }
}

impl Node for SyscallNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Syscall
    }
}