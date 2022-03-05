use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;

pub struct VarAssignNode {
    var_name: String,
    value_node: Box<dyn Node>,
    pos_start: Position
}

impl VarAssignNode {

    pub fn new(var_name: String, value_node: Box<dyn Node>, pos_start: Position) -> Self {
        VarAssignNode {
            var_name,
            value_node,
            pos_start
        }
    }

    pub fn var_name(&self) -> &String {
        &self.var_name
    }
    pub fn value_node(&self) -> &Box<dyn Node> {
        &self.value_node
    }
    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }

}

impl Display for VarAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<VarAssignNode>[Name: {}, Value: {}]", self.var_name, self.value_node)
    }
}

impl NodeToAny for VarAssignNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for VarAssignNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::VarAssign
    }
}