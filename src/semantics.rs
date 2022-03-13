use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use crate::context::Context;
use crate::error;
use crate::error::Error;
use crate::nodes::binop::BinOpNode;
use crate::nodes::{Node, NodeType};
use crate::nodes::call::CallNode;
use crate::nodes::functiondef::FunctionDefinitionNode;
use crate::nodes::list::ListNode;
use crate::nodes::NodeType::VarAssign;
use crate::nodes::number::NumberNode;
use crate::nodes::string::StringNode;
use crate::nodes::unaryop::UnaryOpNode;
use crate::nodes::var::assign::VarAssignNode;
use crate::nodes::var::declare::VarDeclarationNode;
use crate::results::validation::ValidationResult;
use crate::symboltable::Symbol;
use crate::values::types::bool::BoolType;
use crate::values::types::function::FunctionType;
use crate::values::types::number::NumberType;
use crate::values::types::string::StringType;
use crate::values::vtype::{ValueType, ValueTypes};

pub fn validate(node: &Box<dyn Node>, context: &mut Context) -> ValidationResult {
    match node.node_type() {
        NodeType::Number => validate_number_node(),
        NodeType::String => validate_string_node(),
        NodeType::List => validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap(), context),
        NodeType::BinOp => validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap(), context),
        NodeType::UnaryOp => validate_unary_op_node(node.as_any().downcast_ref::<UnaryOpNode>().unwrap(), context),
        NodeType::VarDeclaration => validate_var_declaration_node(node.as_any().downcast_ref::<VarDeclarationNode>().unwrap(), context),
        NodeType::VarAssign => validate_var_assign_node(node.as_any().downcast_ref::<VarAssignNode>().unwrap(), context),
        NodeType::FunctionDef => validate_function_def_node(node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap(), context),
        NodeType::Call => validate_call_node(node.as_any().downcast_ref::<CallNode>().unwrap(), context),
        _ => validate_empty()
    }
}

fn validate_empty() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::from(BoolType::new()));
    res
}

fn validate_number_node() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::from(NumberType::new()));
    res
}

fn validate_string_node() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::from(StringType::new()));
    res
}

fn validate_list_node(node: &ListNode, context: &mut Context) -> ValidationResult {
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

    res.success(Box::from(ListType::new(begin_type.unwrap())));*/
    for el in node.element_nodes() {
        res.register_res(validate(el, context));
        if res.has_error() {
            return res;
        }
    }

    res.success(Box::from(BoolType::new()));
    res
}

fn validate_bin_op_node(node: &BinOpNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    let left = res.register_res(validate(node.left_node(), context));
    if res.has_error() {
        return res;
    }

    let right = res.register_res(validate(node.right_node(), context));
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

fn validate_unary_op_node(node: &UnaryOpNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    let right = res.register_res(validate(node.node(), context));
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

fn validate_var_declaration_node(node: &VarDeclarationNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    if context.symbol_table().has(node.var_name().as_str()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was already declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    let t = res.register_res(validate(node.value_node(), context));
    if res.has_error() {
        return res;
    }

    context.symbol_table().declare(node.var_name().as_str(), Symbol::new(t.unwrap(), node.is_mutable()));
    res.success(Box::from(BoolType::new()));
    res
}

fn validate_var_assign_node(node: &VarAssignNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    if !context.symbol_table().has(node.var_name()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' was not declared in this scope!", node.var_name()).as_str()));
        return res;
    }

    if !context.symbol_table().get(node.var_name()).as_ref().unwrap().is_mutable() {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable '{}' is not mutable!", node.var_name()).as_str()));
        return res;
    }

    let assign_type = res.register_res(validate(node.value_node(), context));
    if res.has_error() {
        return res;
    }

    if !context.symbol_table().get(node.var_name()).as_ref().unwrap().value_type().eq(&assign_type.as_ref().unwrap()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Variable type {} does not match assign type {}!", context.symbol_table().get(node.var_name()).as_ref().unwrap().value_type(), assign_type.as_ref().unwrap()).as_str()));
        return res;
    }

    res.success(Box::from(BoolType::new()));
    res
}

fn validate_function_def_node(node: &FunctionDefinitionNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    if node.var_name().is_some() && context.symbol_table().has(node.var_name().as_ref().unwrap()) {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function or variable with name '{}' was already declared in this scope!", node.var_name().as_ref().unwrap()).as_str()));
        return res;
    }

    let mut arg_types: Vec<Box<dyn ValueType>> = vec![];
    arg_types.reserve(node.arg_names().len());

    for (i, arg) in node.arg_names().iter().enumerate() {
        arg_types.push(Box::from(BoolType::new()));
    }

    context.symbol_table().declare(
        node.var_name().as_ref().unwrap().as_str(),
        Symbol::new(Box::from(FunctionType::new(arg_types, Box::from(BoolType::new()))), false),
    );

    res.success(Box::from(BoolType::new()));
    res
}

fn validate_call_node(node: &CallNode, context: &mut Context) -> ValidationResult {
    let mut res = ValidationResult::new();

    if !context.symbol_table().has(node.func_to_call()) || context.symbol_table().get(node.func_to_call()).unwrap().value_type().value_type() != ValueTypes::Function {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), "Expected function!"))
    }

    /*let call_type = res.register_res(validate(node.node_to_call(), context));
    if res.has_error() {
        return res;
    }

    if call_type.as_ref().unwrap().value_type() != ValueTypes::Function {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Can't call type: {}", call_type.as_ref().unwrap()).as_str()));
        return res;
    }*/

    /*let function_type = context.c_symbol_table().get(node.func_to_call()).as_ref().unwrap().value_type().as_any().downcast_ref::<FunctionType>().unwrap();
    if function_type.arg_types().len() != node.arg_nodes().len() {
        res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Function expected {} parameters. Passed {}!", function_type.arg_types().len(), node.arg_nodes().len()).as_str()));
        return res;
    }*/

    for (i, arg) in node.arg_nodes().iter().enumerate() {
        let t = res.register_res(validate(arg, context));
        if res.has_error() {
            return res;
        }

        /*if !t.as_ref().unwrap().eq(&function_type.arg_types()[i]) {
            res.failure(error::semantic_error(*node.pos_start(), *node.pos_end(), format!("Expected type {}, got {}!", function_type.arg_types()[i], t.as_ref().unwrap()).as_str()));
            return res;
        }*/
    }

    res.success(Box::from(BoolType::new()));
    res
}