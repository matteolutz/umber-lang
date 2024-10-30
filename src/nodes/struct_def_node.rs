use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct StructDefinitionNode {
    name: String,
    fields: Vec<(String, Box<dyn ValueType>)>,
    pos_start: Position,
    pos_end: Position,
}

impl StructDefinitionNode {
    pub fn new(
        name: String,
        fields: Vec<(String, Box<dyn ValueType>)>,
        pos_start: Position,
        pos_end: Position,
    ) -> Self {
        Self {
            name,
            fields,
            pos_start,
            pos_end,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn fields(&self) -> &Vec<(String, Box<dyn ValueType>)> {
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
        write!(
            f,
            "struct {} {{ {} }}",
            self.name,
            self.fields
                .iter()
                .map(|(field_name, field_type)| format!("{}: {}", field_name, field_type))
                .collect::<Vec<String>>()
                .join(", ")
        )
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

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
