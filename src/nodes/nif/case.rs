use std::fmt::{Display, Formatter};
use crate::nodes::Node;

pub struct IfCase {
    condition: Box<dyn Node>,
    statements: Box<dyn Node>,
    should_auto_return: bool,
}

impl IfCase {
    pub fn new(condition: Box<dyn Node>, statements: Box<dyn Node>, should_auto_return: bool) -> Self {
        IfCase {
            condition,
            statements,
            should_auto_return,
        }
    }

    pub fn condition(&self) -> &Box<dyn Node> {
        &self.condition
    }

    pub fn statements(&self) -> &Box<dyn Node> {
        &self.statements
    }

    pub fn should_auto_return(&self) -> bool {
        self.should_auto_return
    }
}

impl Display for IfCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<IfCase>[Cond.: {}, Stmts: {}, AutoReturn?: {}]", self.condition, self.statements, self.should_auto_return)
    }
}