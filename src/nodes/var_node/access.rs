use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct VarAccessNode {
    var_name: String,
    pos_start: Position,
    pos_end: Position,
}

impl VarAccessNode {
    pub fn new(var_name: String, pos_start: Position, pos_end: Position) -> Self {
        VarAccessNode {
            var_name,
            pos_start,
            pos_end,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }
    pub fn pos_end(&self) -> &Position {
        &self.pos_end
    }
}

impl Display for VarAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<VarAccessNode>[{}]", self.var_name)
    }
}

impl NodeToAny for VarAccessNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for VarAccessNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::VarAccess
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
