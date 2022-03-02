use std::fmt::{Display, Formatter};
use crate::nodes::Node;
use crate::position::Position;
use crate::token::Token;

pub struct VarDeclarationNode {
    var_name: String,
    value_node: Box<dyn Node>,
    is_mutable: bool,
    pos_start: Position,
}

impl VarDeclarationNode {
    pub fn new(var_name: String, value_node: Box<dyn Node>, is_mutable: bool, pos_start: Position) -> Self {
        VarDeclarationNode {
            var_name,
            value_node,
            is_mutable,
            pos_start,
        }
    }
}

impl Display for VarDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<VarDeclarationNode>[Name: {}, Value: {}, IsMutable?: {}]", self.var_name, self.value_node, self.is_mutable)
    }
}

impl Node for VarDeclarationNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value_node.pos_end()
    }
}