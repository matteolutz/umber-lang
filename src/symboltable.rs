use std::borrow::Borrow;
use std::collections::HashMap;
use crate::values::vtype::ValueType;

pub struct Symbol {
    value_type: Box<dyn ValueType>,
    is_mutable: bool
}

impl Symbol {

    pub fn new(value_type: Box<dyn ValueType>, is_mutable: bool) -> Self {
        Symbol {
            value_type,
            is_mutable
        }
    }

    pub fn is_mutable(&self) -> bool { self.is_mutable }
    pub fn value_type(&self) -> &Box<dyn ValueType> { &self.value_type }

}

pub struct SymbolTable<'a> {
    symbols: HashMap<String, Symbol>,
    parent: Option<&'a mut SymbolTable<'a>>
}

impl<'a> SymbolTable<'a> {

    pub fn new(parent: Option<&'a mut SymbolTable<'a>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent
        }
    }

    pub fn symbols(&self) -> &HashMap::<String, Symbol> { &self.symbols }
    pub fn parent(&self) -> &Option<&mut SymbolTable<'a>> { &self.parent }

    pub fn has(&self, name: &str) -> bool {
        if self.symbols.contains_key(name) {
            return true;
        } else if self.parent.is_some() {
            return self.parent.as_ref().unwrap().has(name);
        }

        false
    }

    pub fn is_mutable(&self, name: &str) -> bool {
        if self.symbols.contains_key(name) {
            return self.symbols.get(name).as_ref().unwrap().is_mutable();
        } else if self.parent.is_some() {
            return self.parent.as_ref().unwrap().is_mutable(name);
        }

        false
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        if self.has(name) {
            return self.symbols.get(name);
        } else if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get(name);
        }

        None
    }

    pub fn declare(&mut self, name: &str, symbol: Symbol) -> bool {
        if self.has(name) {
            return false;
        }

        self.symbols.insert(String::from(name), symbol);
        true
    }
}