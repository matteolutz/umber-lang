use std::fmt::{Display, Formatter};
use crate::position::Position;
use crate::symboltable::{Symbol, SymbolTable};

pub struct Context<'a> {
    display_name: String,
    parent: Option<&'a Context<'a>>,
    parent_entry_pos: Option<Position>,
    symbol_table: SymbolTable<'a>
}

impl<'a> Context<'a> {

    pub fn new(display_name: &str, parent: Option<&'a Context<'a>>, parent_entry_pos: Option<Position>, symbol_table: SymbolTable<'a>) -> Self {
        Context {
            display_name: String::from(display_name),
            parent,
            parent_entry_pos,
            symbol_table
        }
    }

    pub fn new_with_symbol_table(display_name: &str, parent: Option<&'a Context<'a>>, parent_entry_pos: Option<Position>) -> Self {
        Context {
            display_name: String::from(display_name),
            parent,
            parent_entry_pos,
            symbol_table: SymbolTable::new(if parent.is_some() { Some(parent.as_ref().unwrap().symbol_table()) } else { None })
        }
    }

    pub fn display_name(&self) -> &String { &self.display_name }
    pub fn parent(&self) -> &Option<&'a Context<'a>> { &self.parent }
    pub fn parent_entry_pos(&self) -> &Option<Position> { &self.parent_entry_pos }
    pub fn symbol_table(&self) -> &SymbolTable { &self.symbol_table }

}

impl Display for Context<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}