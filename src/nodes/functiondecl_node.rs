use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct FunctionDeclarationNode {
    var_name: String,
    args: Vec<(String, Box<dyn ValueType>)>,
    return_type: Box<dyn ValueType>,
    pos_start: Position,
    pos_end: Position,
}

impl FunctionDeclarationNode {
    pub fn new(
        var_name: String,
        args: Vec<(String, Box<dyn ValueType>)>,
        return_type: Box<dyn ValueType>,
        pos_start: Position,
        pos_end: Position,
    ) -> Self {
        FunctionDeclarationNode {
            var_name,
            args,
            return_type,
            pos_start,
            pos_end,
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
}

impl Display for FunctionDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<FunctionDeclarationNode>[Name: {}, Args: {}]",
            self.var_name,
            self.args.len()
        )
    }
}

impl NodeToAny for FunctionDeclarationNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for FunctionDeclarationNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::FunctionDecl
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
