use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::IndexMut;
use crate::error;
use crate::error::{Error, semantic_error};
use crate::nodes::binop::BinOpNode;
use crate::nodes::{Node, NodeType};
use crate::nodes::call::CallNode;
use crate::nodes::functiondef::FunctionDefinitionNode;
use crate::nodes::list::ListNode;
use crate::nodes::NodeType::VarAssign;
use crate::nodes::nreturn::ReturnNode;
use crate::nodes::number::NumberNode;
use crate::nodes::statements::StatementsNode;
use crate::nodes::string::StringNode;
use crate::nodes::unaryop::UnaryOpNode;
use crate::nodes::var::access::VarAccessNode;
use crate::nodes::var::assign::VarAssignNode;
use crate::nodes::var::declare::VarDeclarationNode;
use crate::results::validation::ValidationResult;
use crate::symboltable::{Symbol, SymbolTable};
use crate::values::types::bool::BoolType;
use crate::values::types::function::FunctionType;
use crate::values::types::number::NumberType;
use crate::values::types::string::StringType;
use crate::values::types::void::VoidType;
use crate::values::vtype::{ValueType, ValueTypes};

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

    fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        for s in self.type_stack.iter().rev() {
            if s.contains_key(name) {
                return s.get(name);
            }
        }

        None
    }

    fn declare(&mut self, name: &str, sym: Symbol) {
        if self.has_symbol(name) {
            return;
        }

        (self.type_stack.index_mut(self.type_stack.len() - 1)).insert(name.to_string(), sym);
    }

}

pub fn validate(node: &Box<dyn Node>, symbol_table: &mut SymbolTable) -> ValidationResult {
    match node.node_type() {
        NodeType::Statements => validate_statements_node(node.as_any().downcast_ref::<StatementsNode>().unwrap(), symbol_table),
        NodeType::Number => validate_number_node(),
        NodeType::String => validate_string_node(),
        NodeType::List => validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap(), symbol_table),
        NodeType::BinOp => validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap(), symbol_table),
        NodeType::UnaryOp => validate_unary_op_node(node.as_any().downcast_ref::<UnaryOpNode>().unwrap(), symbol_table),
        NodeType::VarDeclaration => validate_var_declaration_node(node.as_any().downcast_ref::<VarDeclarationNode>().unwrap(), symbol_table),
        NodeType::VarAssign => validate_var_assign_node(node.as_any().downcast_ref::<VarAssignNode>().unwrap(), symbol_table),
        NodeType::VarAccess => validate_var_access_node(node.as_any().downcast_ref::<VarAccessNode>().unwrap(), symbol_table),
        NodeType::FunctionDef => validate_function_def_node(node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap(), symbol_table),
        NodeType::Call => validate_call_node(node.as_any().downcast_ref::<CallNode>().unwrap(), symbol_table),
        NodeType::Return => validate_return_node(node.as_any().downcast_ref::<ReturnNode>().unwrap(), symbol_table),
        _ => validate_empty()
    }
}

fn validate_statements_node(node: &StatementsNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    for s in node.statement_nodes() {
        res.register_res(validate(s, symbol_table));
        if res.has_error() {
            return res;
        }
    }

    // res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "Can't use statements as a type!"));
    res
}

fn validate_empty() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::new(BoolType::new()));
    res
}

fn validate_number_node() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::new(NumberType::new()));
    res
}

fn validate_string_node() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::new(StringType::new()));
    res
}

fn validate_list_node(node: &ListNode, symbol_table: &mut SymbolTable) -> ValidationResult {
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
        res.register_res(validate(el, symbol_table));
        if res.has_error() {
            return res;
        }
    }

    res.success(Box::new(BoolType::new()));
    res
}

fn validate_bin_op_node(node: &BinOpNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    let left = res.register_res(validate(node.left_node(), symbol_table));
    if res.has_error() {
        return res;
    }

    let right = res.register_res(validate(node.right_node(), symbol_table));
    if res.has_error() {
        return res;
    }

    let result_type = left.as_ref().unwrap().is_valid_bin_op(node.op_token(), right.as_ref().unwrap());
    if result_type.is_none() {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Binary operation '{}' not allowed between types {} and {}!", node.op_token(), left.as_ref().unwrap(), right.as_ref().unwrap()).as_str()));
        return res;
    }

    res.success(result_type.unwrap());
    res
}

fn validate_unary_op_node(node: &UnaryOpNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    let right = res.register_res(validate(node.node(), symbol_table));
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

fn validate_var_declaration_node(node: &VarDeclarationNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if symbol_table.has(node.var_name().as_str()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was already declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    let t = res.register_res(validate(node.value_node(), symbol_table));
    if res.has_error() {
        return res;
    }

    let symbol_type = t.unwrap();

    if !symbol_type.eq(node.var_type()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Type '{}' can't be assigned to type '{}'!", &symbol_type, node.var_type()).as_str()));
        return res;
    }

    symbol_table.declare(node.var_name().as_str(), Symbol::new(symbol_type.clone(), node.is_mutable()));
    res.success(symbol_type);
    res
}

fn validate_var_assign_node(node: &VarAssignNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if !symbol_table.has(node.var_name()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was not declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    if !symbol_table.get(node.var_name()).as_ref().unwrap().is_mutable() {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' is not mutable!", node.var_name()).as_str()));
        return res;
    }

    let assign_type = res.register_res(validate(node.value_node(), symbol_table));
    if res.has_error() {
        return res;
    }

    let symbol_type = assign_type.unwrap();

    if symbol_table.get(node.var_name()).as_ref().unwrap().value_type().eq(&symbol_type) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable type {} does not match assign type {}!", symbol_table.get(node.var_name()).as_ref().unwrap().value_type(), &symbol_type).as_str()));
        return res;
    }

    res.success(symbol_type);
    res
}

fn validate_var_access_node(node: &VarAccessNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if !symbol_table.has(node.var_name().as_str()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' wasn't declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    res.success(symbol_table.get(node.var_name().as_str()).as_ref().unwrap().value_type().clone());
    res
}

fn validate_function_def_node(node: &FunctionDefinitionNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if symbol_table.has(node.var_name()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function or variable with name '{}' was already declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    let mut arg_types: Vec<Box<dyn ValueType>> = vec![];
    arg_types.reserve(node.args().len());

    for (name, value_type) in node.args() {
        arg_types.push(value_type.clone());
        symbol_table.declare(name.as_str(), Symbol::new(value_type.clone(), false));
    }

    symbol_table.declare(
        node.var_name().as_str(),
        Symbol::new(Box::new(FunctionType::new(arg_types, node.return_type().clone())), false),
    );

    res.register_res(validate(node.body_node(), symbol_table));
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

fn validate_call_node(node: &CallNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if !symbol_table.has(node.func_to_call()) || symbol_table.get(node.func_to_call()).unwrap().value_type().value_type() != ValueTypes::Function {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "Expected function!"))
    }

    let symbol_type = symbol_table.get(node.func_to_call().as_str()).as_ref().unwrap().value_type().clone();
    let function_type = symbol_type.as_any().downcast_ref::<FunctionType>().unwrap();

    if function_type.arg_types().len() != node.arg_nodes().len() {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function expected {} arguments. Passed {}!", function_type.arg_types().len(), node.arg_nodes().len()).as_str()));
        return res;
    }

    for (i, arg) in node.arg_nodes().iter().enumerate() {
        let t = res.register_res(validate(arg, symbol_table));
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

fn validate_return_node(node: &ReturnNode, symbol_table: &mut SymbolTable) -> ValidationResult {
    let mut res = ValidationResult::new();

    if node.node_to_return().is_none() {
        res.success_return(Box::new(VoidType::new()));
        return res;
    }

    let return_type = res.register_res(validate(node.node_to_return().as_ref().unwrap(), symbol_table));
    if res.has_error() {
        return res;
    }

    res.success_return(return_type.unwrap());
    res
}