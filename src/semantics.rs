use std::collections::HashMap;
use std::ops::IndexMut;

use crate::error;
use crate::nodes::{Node, NodeType};
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::list_node::ListNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::unaryop_node::UnaryOpNode;
use crate::nodes::var_node::access::VarAccessNode;
use crate::nodes::var_node::assign::VarAssignNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::results::validation::ValidationResult;
use crate::symbol_table::Symbol;
use crate::values::value_type::{ValueType, ValueTypes};
use crate::values::value_type::bool_type::BoolType;
use crate::values::value_type::function_type::FunctionType;
use crate::values::value_type::number_type::NumberType;
use crate::values::value_type::string_type::StringType;
use crate::values::value_type::void_type::VoidType;

pub struct Validator {
    type_stack: Vec<HashMap<String, Symbol>>,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            type_stack: vec![
                HashMap::new()
            ]
        }
    }

    fn has_symbol(&self, name: &str) -> bool {
        for s in self.type_stack.iter().rev() {
            if s.contains_key(name) {
                return true;
            }
        }

        false
    }

    fn is_symbol_mut(&self, name: &str) -> bool {
        let s = self.get_symbol(name);

        if s.is_some() {
            return s.unwrap().is_mutable();
        }

        false
    }

    fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        for s in self.type_stack.iter().rev() {
            if s.contains_key(name) {
                return s.get(name);
            }
        }

        None
    }

    fn declare_symbol(&mut self, name: &str, sym: Symbol) {
        if self.type_stack.is_empty() {
            return;
        }

        if self.has_symbol(name) {
            return;
        }

        (self.type_stack.index_mut(self.type_stack.len() - 1)).insert(name.to_string(), sym);
    }

    fn push_child_table(&mut self) {
        self.type_stack.push(HashMap::new());
    }

    fn pop_child_table(&mut self) {
        if self.type_stack.len() == 1 {
            panic!("Can't pop root table!");
        }

        self.type_stack.pop();
    }

}

impl Validator {

    pub fn validate(&mut self, node: &Box<dyn Node>) -> ValidationResult {
        match node.node_type() {
            NodeType::Statements => self.validate_statements_node(node.as_any().downcast_ref::<StatementsNode>().unwrap()),
            NodeType::Number => self.validate_number_node(),
            NodeType::String => self.validate_string_node(),
            NodeType::List => self.validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap()),
            NodeType::BinOp => self.validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap()),
            NodeType::UnaryOp => self.validate_unary_op_node(node.as_any().downcast_ref::<UnaryOpNode>().unwrap()),
            NodeType::VarDeclaration => self.validate_var_declaration_node(node.as_any().downcast_ref::<VarDeclarationNode>().unwrap()),
            NodeType::VarAssign => self.validate_var_assign_node(node.as_any().downcast_ref::<VarAssignNode>().unwrap()),
            NodeType::VarAccess => self.validate_var_access_node(node.as_any().downcast_ref::<VarAccessNode>().unwrap()),
            NodeType::FunctionDef => self.validate_function_def_node(node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap()),
            NodeType::Call => self.validate_call_node(node.as_any().downcast_ref::<CallNode>().unwrap()),
            NodeType::Return => self.validate_return_node(node.as_any().downcast_ref::<ReturnNode>().unwrap()),
            _ => self.validate_empty()
        }
    }

    fn validate_statements_node(&mut self, node: &StatementsNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        self.push_child_table();

        for s in node.statement_nodes() {
            res.register_res(self.validate(s));
            if res.has_error() {
                return res;
            }
        }

        self.pop_child_table();

        // res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "Can't use statements as a type!"));
        res
    }

    fn validate_empty(&mut self) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(BoolType::new()));
        res
    }

    fn validate_number_node(&mut self) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(NumberType::new()));
        res
    }

    fn validate_string_node(&mut self) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(StringType::new()));
        res
    }

    fn validate_list_node(&mut self, node: &ListNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        /*if node.element_nodes().is_empty() {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "No given value type!"));
            return res;
        }

        let mut begin_type: Option<Box<dyn ValueType>> = None;
        for el in node.element_nodes() {

            let t = res.register_res(validate(el, context));
            if res.has_error() {
                return res;
            }

            if begin_type.is_some() && !t.as_ref().unwrap().eq(begin_type.as_ref().unwrap()) {
                res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Type {}, is incompatible with list type {}!", t.as_ref().unwrap(), begin_type.as_ref().unwrap()).as_str()));
                return res;
            } else if begin_type.is_none() {
                begin_type = t;
            }
        }

        res.success(Box::new(ListType::new(begin_type.unwrap())));*/
        for el in node.element_nodes() {
            res.register_res(self.validate(el));
            if res.has_error() {
                return res;
            }
        }

        res.success(Box::new(BoolType::new()));
        res
    }

    fn validate_bin_op_node(&mut self, node: &BinOpNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let left = res.register_res(self.validate(node.left_node()));
        if res.has_error() {
            return res;
        }

        let right = res.register_res(self.validate(node.right_node()));
        if res.has_error() {
            return res;
        }

        let result_type = left.as_ref().unwrap().is_valid_bin_op(node.op_token(), right.as_ref().unwrap());
        if result_type.is_none() {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Binary operation '{}' not allowed between value_type {} and {}!", node.op_token(), left.as_ref().unwrap(), right.as_ref().unwrap()).as_str()));
            return res;
        }

        res.success(result_type.unwrap());
        res
    }

    fn validate_unary_op_node(&mut self, node: &UnaryOpNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let right = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        let result_type = right.as_ref().unwrap().is_valid_unary_op(node.op_token());
        if result_type.is_none() {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Unary operation '{}' not allowed on type {}!", node.op_token(), right.as_ref().unwrap()).as_str()));
            return res;
        }

        res.success(result_type.unwrap());
        res
    }

    fn validate_var_declaration_node(&mut self, node: &VarDeclarationNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.has_symbol(node.var_name().as_str()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was already declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        let t = res.register_res(self.validate(node.value_node()));
        if res.has_error() {
            return res;
        }

        let symbol_type = t.unwrap();

        if !symbol_type.eq(node.var_type()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Type '{}' can't be assigned to type '{}'!", &symbol_type, node.var_type()).as_str()));
            return res;
        }

        self.declare_symbol(node.var_name().as_str(), Symbol::new(symbol_type.clone(), node.is_mutable()));
        res.success(symbol_type);
        res
    }

    fn validate_var_assign_node(&mut self, node: &VarAssignNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was not declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        if !self.is_symbol_mut(node.var_name()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' is not mutable!", node.var_name()).as_str()));
            return res;
        }

        let assign_type = res.register_res(self.validate(node.value_node()));
        if res.has_error() {
            return res;
        }

        let symbol_type = assign_type.unwrap();

        if self.get_symbol(node.var_name()).unwrap().value_type().eq(&symbol_type) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable type {} does not match assign type {}!", self.get_symbol(node.var_name()).unwrap().value_type(), &symbol_type).as_str()));
            return res;
        }

        res.success(symbol_type);
        res
    }

    fn validate_var_access_node(&mut self, node: &VarAccessNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.var_name().as_str()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' wasn't declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        res.success(self.get_symbol(node.var_name().as_str()).unwrap().value_type().clone());
        res
    }

    fn validate_function_def_node(&mut self, node: &FunctionDefinitionNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function or variable with name '{}' was already declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        let mut arg_types: Vec<Box<dyn ValueType>> = vec![];
        arg_types.reserve(node.args().len());

        for (_name, value_type) in node.args() {
            arg_types.push(value_type.clone());
        }

        self.declare_symbol(
            node.var_name().as_str(),
            Symbol::new(Box::new(FunctionType::new(arg_types.clone(), node.return_type().clone())), false),
        );

        self.push_child_table();

        for a in arg_types {
            self.declare_symbol(node.var_name().as_str(), Symbol::new(a.clone(), false));
        }

        res.register_res(self.validate(node.body_node()));

        self.pop_child_table();

        if res.has_error() {
            return res;
        }

        if !res.has_return_type() {
            res.failure(error::semantic_error(*node.pos_end(), *node.pos_end(), "No return statement given!"));
            return res;
        }

        if !res.return_type().as_ref().unwrap().eq(node.return_type()) {
            res.failure(error::semantic_error(*node.pos_end(), *node.pos_end(), format!("Function return type is '{}', returned was '{}'!", node.return_type(), res.return_type().as_ref().unwrap()).as_str()));
            return res;
        }

        res
    }

    fn validate_call_node(&mut self, node: &CallNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.func_to_call()) || self.get_symbol(node.func_to_call()).unwrap().value_type().value_type() != ValueTypes::Function {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "Expected function!"))
        }

        let symbol_type = self.get_symbol(node.func_to_call().as_str()).unwrap().value_type().clone();
        let function_type = symbol_type.as_any().downcast_ref::<FunctionType>().unwrap();

        if function_type.arg_types().len() != node.arg_nodes().len() {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function expected {} arguments. Passed {}!", function_type.arg_types().len(), node.arg_nodes().len()).as_str()));
            return res;
        }

        for (i, arg) in node.arg_nodes().iter().enumerate() {
            let t = res.register_res(self.validate(arg));
            if res.has_error() {
                return res;
            }

            if !t.as_ref().unwrap().eq(&function_type.arg_types()[i]) {
                res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Expected type '{}' as argument at index {}, got '{}'!", function_type.arg_types()[i], i, t.as_ref().unwrap()).as_str()));
                return res;
            }
        }

        res.success(function_type.return_type().clone());
        res
    }

    fn validate_return_node(&mut self, node: &ReturnNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if node.node_to_return().is_none() {
            res.success_return(Box::new(VoidType::new()));
            return res;
        }

        let return_type = res.register_res(self.validate(node.node_to_return().as_ref().unwrap()));
        if res.has_error() {
            return res;
        }

        res.success_return(return_type.unwrap());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn semantics_symbol_stack() {
        let mut v = Validator::new();

        v.declare_symbol("a", Symbol::new(Box::new(VoidType::new()), false));

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.get_symbol("a").unwrap().value_type().value_type(), ValueTypes::Void);
        assert_eq!(v.is_symbol_mut("a"), false);

        v.push_child_table();

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.has_symbol("b"), false);
        v.declare_symbol("b", Symbol::new(Box::new(VoidType::new()), false));
        assert_eq!(v.has_symbol("b"), true);

        v.pop_child_table();

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.has_symbol("b"), false);
    }
}