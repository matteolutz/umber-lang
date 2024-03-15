use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct FunctionDefinitionNode {
    var_name: String,
    args: Vec<(String, Box<dyn ValueType>)>,
    return_type: Box<dyn ValueType>,
    body_node: Box<dyn Node>,
    generics: Vec<String>,
    pos_start: Position,
}

impl FunctionDefinitionNode {
    pub fn new(
        var_name: String,
        args: Vec<(String, Box<dyn ValueType>)>,
        return_type: Box<dyn ValueType>,
        body_node: Box<dyn Node>,
        generics: Vec<String>,
        pos_start: Position,
    ) -> Self {
        FunctionDefinitionNode {
            var_name,
            args,
            return_type,
            body_node,
            generics,
            pos_start,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn args(&self) -> &Vec<(String, Box<dyn ValueType>)> {
        &self.args
    }
    pub fn return_type(&self) -> &Box<dyn ValueType> {
        &self.return_type
    }
    pub fn body_node(&self) -> &Box<dyn Node> {
        &self.body_node
    }
    pub fn generics(&self) -> &Vec<String> {
        &self.generics
    }
}

impl Display for FunctionDefinitionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<FunctionDefinitionNode>[Name: {}, Args: {}, Body: {}, Generics: {:?}]",
            self.var_name,
            self.args.len(),
            self.body_node,
            self.generics
        )
    }
}

impl NodeToAny for FunctionDefinitionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for FunctionDefinitionNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.body_node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::FunctionDef
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
