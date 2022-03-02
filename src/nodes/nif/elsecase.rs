use std::fmt::{Display, Formatter};
use crate::nodes::Node;

pub struct ElseCase {
    statements: Box<dyn Node>,
    should_auto_return: bool
}

impl ElseCase {

    pub fn new(statements: Box<dyn Node>, should_auto_return: bool) -> Self {
        ElseCase {
            statements,
            should_auto_return
        }
    }

    pub fn statements(&self) -> &Box<dyn Node> {
        &self.statements
    }

    pub fn should_auto_return(&self) -> bool {
        self.should_auto_return
    }

}

impl Display for ElseCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ElseCase>[Stmts: {}, AutoReturn?: {}]", self.statements, self.should_auto_return)
    }
}