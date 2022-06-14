use std::fmt::{Display, Formatter};

use crate::nodes::Node;

#[derive(Clone)]
pub struct IfCase {
    condition: Box<dyn Node>,
    statements: Box<dyn Node>
}

impl IfCase {
    pub fn new(condition: Box<dyn Node>, statements: Box<dyn Node>) -> Self {
        IfCase {
            condition,
            statements,
        }
    }

    pub fn condition(&self) -> &Box<dyn Node> {
        &self.condition
    }

    pub fn statements(&self) -> &Box<dyn Node> {
        &self.statements
    }
}

impl Display for IfCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<IfCase>[Cond.: {}, Stmts: {}]", self.condition, self.statements)
    }
}