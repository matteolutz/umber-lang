use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

pub struct ConstDefinitionNode {
    name: String,
    value: Box<dyn Node>,
    value_type: Box<dyn ValueType>,
    pos_start: Position
}

impl ConstDefinitionNode {
    pub fn new(name: String, value: Box<dyn Node>, value_type: Box<dyn ValueType>, pos_start: Position) -> Self {
        Self {
            name,
            value,
            value_type,
            pos_start
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> &Box<dyn Node> {
        &self.value
    }
    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }

}

impl NodeToAny for ConstDefinitionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ConstDefinitionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ConstDefinitionNode>[Name: \"{}\", Value: {}]", self.name, self.value)
    }
}

impl Node for ConstDefinitionNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::ConstDef
    }
}