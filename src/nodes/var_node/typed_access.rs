use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct VarTypedAccessNode {
    var_name: String,
    value_type: Box<dyn ValueType>,
    pos_start: Position,
    pos_end: Position,
}

impl VarTypedAccessNode {
    pub fn new(
        var_name: String,
        value_type: Box<dyn ValueType>,
        pos_start: Position,
        pos_end: Position,
    ) -> Self {
        Self {
            var_name,
            value_type,
            pos_start,
            pos_end,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }
    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }
    pub fn pos_end(&self) -> &Position {
        &self.pos_end
    }
}

impl Display for VarTypedAccessNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<VarSizedAccess>[{}]", self.var_name)
    }
}

impl NodeToAny for VarTypedAccessNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for VarTypedAccessNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::VarTypedAccess
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
