use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

pub struct StructDefinitionNode {
    name: String,
    fields: Vec<(String, Box<dyn ValueType>)>,
    pos_start: Position,
    pos_end: Position
}

impl StructDefinitionNode {
    pub fn new(name: String, fields: Vec<(String, Box<dyn ValueType>)>, pos_start: Position, pos_end: Position) -> Self {
        Self {
            name,
            fields,
            pos_start,
            pos_end
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_fields(&self) -> &Vec<(String, Box<dyn ValueType>)> {
        &self.fields
    }

}

impl NodeToAny for StructDefinitionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for StructDefinitionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StructDefinitionNode>[Name: {}, Fields: {}]", self.name, self.fields.len())
    }
}

impl Node for StructDefinitionNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::StructDef
    }
}