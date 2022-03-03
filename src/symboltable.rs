pub struct Symbol {
    is_mutable: bool
}

impl Symbol {

    pub fn new(is_mutable: bool) -> Self {
        Symbol {
            is_mutable
        }
    }

    pub fn is_mutable(&self) -> bool { self.is_mutable }

}

pub struct SymbolTable<'a> {
    symbols: Vec<Symbol>,
    parent: Option<&'a SymbolTable<'a>>
}

impl<'a> SymbolTable<'a> {

    pub fn new(parent: Option<&'a SymbolTable<'a>>) -> Self {
        SymbolTable {
            symbols: vec![],
            parent
        }
    }

}