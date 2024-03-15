use std::fmt::{Display, Formatter};

use crate::nodes::Node;

#[derive(Clone)]
pub struct ElseCase {
    statements: Box<dyn Node>,
}

impl ElseCase {
    pub fn new(statements: Box<dyn Node>) -> Self {
        ElseCase { statements }
    }

    pub fn statements(&self) -> &Box<dyn Node> {
        &self.statements
    }
}

impl Display for ElseCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ElseCase>[Stmts: {}]", self.statements)
    }
}
