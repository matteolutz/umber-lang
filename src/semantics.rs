use std::any::{Any, TypeId};
use crate::context::Context;
use crate::error;
use crate::error::Error;
use crate::nodes::binop::BinOpNode;
use crate::nodes::{Node, NodeType};
use crate::nodes::list::ListNode;
use crate::results::validation::ValidationResult;
use crate::values::types::list::ListType;
use crate::values::types::number::NumberType;
use crate::values::vtype::ValueType;

pub fn validate(node: &Box<dyn Node>, context: &Context) -> ValidationResult {
    match node.node_type() {
        NodeType::BinOp => validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap(), context),
        NodeType::List => validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap(), context),
        _ => validate_empty()
    }
}

fn validate_empty() -> ValidationResult {
    let mut res = ValidationResult::new();

    res.success(Box::from(NumberType::new()));
    res
}

fn validate_bin_op_node(node: &BinOpNode, context: &Context) -> ValidationResult {
    todo!("implement bin op validation")
}

fn validate_list_node(node: &ListNode, context: &Context) -> ValidationResult {

    let mut res = ValidationResult::new();

    if node.element_nodes().is_empty() {
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

    res.success(Box::from(ListType::new(begin_type.unwrap())));
    res
}