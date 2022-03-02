use std::fmt::{Display, Formatter};
use crate::position::Position;

pub struct Context<'a> {
    pub display_name: &'static str,
    pub parent: Option<&'a Context<'a>>,
    pub parent_entry_pos: Option<Position>
}

impl Display for Context<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}
