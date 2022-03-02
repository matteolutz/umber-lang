use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;

pub struct FunctionDefinitionNode {
    var_name: Option<String>,
    arg_names: Vec<String>,
    body_node: Box<dyn Node>,
    should_auto_return: bool,
    pos_start: Position,
}

impl FunctionDefinitionNode {
    pub fn new(var_name: Option<String>, arg_names: Vec<String>, body_node: Box<dyn Node>, should_auto_return: bool, pos_start: Position) -> Self {
        FunctionDefinitionNode {
            var_name,
            arg_names,
            body_node,
            should_auto_return,
            pos_start,
        }
    }
}

impl Display for FunctionDefinitionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FunctionDefinitionNode>[Name: {}, Args: [{}], Body: {}, AutoReturn?: {}]", self.var_name.as_ref().unwrap_or(&String::from("<anonymous>")), self.arg_names.join(","), self.body_node, self.should_auto_return)
    }
}

impl Node for FunctionDefinitionNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.body_node.pos_end()
    }
}