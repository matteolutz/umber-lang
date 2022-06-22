use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::OldToken;

#[derive(Clone)]
pub struct UnaryOpNode {
    op_token: OldToken,
    node: Box<dyn Node>,
}

impl UnaryOpNode {
    pub fn new(op_token: OldToken, node: Box<dyn Node>) -> Self {
        UnaryOpNode {
            op_token,
            node,
        }
    }

    pub fn op_token(&self) -> &OldToken {
        &self.op_token
    }
    pub fn node(&self) -> &Box<dyn Node> {
        &self.node
    }

}

impl Display for UnaryOpNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<UnaryOpNode>[Op: {}, Node: {}]", self.op_token, self.node)
    }
}

impl NodeToAny for UnaryOpNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for UnaryOpNode {
    fn pos_start(&self) -> &Position {
        self.op_token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.node.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::UnaryOp
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}