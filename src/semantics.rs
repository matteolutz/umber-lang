use std::any::{Any, TypeId};
use crate::context::Context;
use crate::error;
use crate::error::Error;
use crate::nodes::binop::BinOpNode;
use crate::nodes::{Node, NodeType};
use crate::nodes::list::ListNode;
use crate::values::vtype::ValueType;

pub fn validate(node: &Box<dyn Node>, context: &Context) -> (Option<Box<dyn ValueType>>, Option<Error>) {
    match node.node_type() {
        NodeType::BinOp => validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap(), context),
        NodeType::List => validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap(), context),
        _ => panic!("no validation method defined!")
    }
}

fn validate_bin_op_node(node: &BinOpNode, context: &Context) -> (Option<Box<dyn ValueType>>, Option<Error>) {
    todo!("implement bin op validation")
}
fn validate_list_node(node: &ListNode, context: &Context) -> (Option<Box<dyn ValueType>>, Option<Error>) {
    todo!("implement list node validation")
}