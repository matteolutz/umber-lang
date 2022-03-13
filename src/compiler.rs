use std::collections::HashMap;
use std::convert::TryInto;
use std::env::var;
use std::fmt::Write;
use std::hash::Hash;
use crate::nodes::{Node, NodeType};
use crate::nodes::binop::BinOpNode;
use crate::nodes::call::CallNode;
use crate::nodes::functiondef::FunctionDefinitionNode;
use crate::nodes::list::ListNode;
use crate::nodes::number::NumberNode;
use crate::nodes::statements::StatementsNode;
use crate::nodes::string::StringNode;
use crate::nodes::unaryop::UnaryOpNode;
use crate::nodes::var::access::VarAccessNode;
use crate::nodes::var::assign::VarAssignNode;
use crate::nodes::var::declare::VarDeclarationNode;
use crate::token::{Token, TokenType};
use crate::values::vtype::{ValueType, ValueTypes};

fn register_func(node: &FunctionDefinitionNode, stack_frame: &mut HashMap<String, usize>, funcs: &mut HashMap<String, (String, String)>) -> String {
    let mut func_stack_frame = stack_frame.clone();

    for i in 0..node.arg_names().len() {
        let idx = func_stack_frame.len() + (8 * (i + 1));
        stack_frame.insert(node.arg_names()[i].clone(), idx);
        return format!("{}\tpop     QWORD [rbp-({})]\n", code_gen_asm(node.value_node(), stack_frame, funcs), idx);
    }



    let mut max = 0;
    for (_, u) in func_stack_frame.iter() {
        if u > &max {
            max = *u;
        }
    }

    let offset = max + 16;
    for (_, value) in func_stack_frame.iter_mut() {
        *value -= offset;
    }

    let mut temp = String::new();
    writeln!(&mut temp, "\tpush    rbp
\tmov     rbp, rsp
        ");
    writeln!(&mut temp, "{}", code_gen_asm(node.body_node(), &mut func_stack_frame, funcs));
    writeln!(&mut temp, "\tleave
\tret
        ");

    let name = format!("{}-{}({})", funcs.len(), node.var_name().as_ref().unwrap(), node.arg_names().join(","));

    println!("func: '{}' registered!", &name);
    funcs.insert(node.var_name().as_ref().unwrap().clone(), (name.clone(), temp));

    return name;
}

fn code_gen_asm(node: &Box<dyn Node>, stack_frame: &mut HashMap<String, usize>, funcs: &mut HashMap<String, (String, String)>) -> String {
    if node.node_type() == NodeType::Statements {
        let mut temp = String::new();
        for el in node.as_any().downcast_ref::<StatementsNode>().unwrap().statement_nodes() {
            writeln!(&mut temp, "{}", code_gen_asm(el, stack_frame, funcs));
        }
        return temp;
    } else if node.node_type() == NodeType::Number {
        return format!("\tpush    QWORD {}\n", node.as_any().downcast_ref::<NumberNode>().unwrap().get_number().to_string());
    } else if node.node_type() == NodeType::String {
        return String::from("\tpush    QWORD 0x0");
    } else if node.node_type() == NodeType::BinOp {
        let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();
        return format!(
            "{}{}\tpop     rax\n\tpop     rbx\n\t{}rax, rbx\n\tpush    rax",
            code_gen_asm(bin_op_node.left_node(), stack_frame, funcs),
            code_gen_asm(bin_op_node.right_node(), stack_frame, funcs),
            match bin_op_node.op_token().token_type() {
                TokenType::Plus => "add     ",
                TokenType::Minus => "sub     ",
                TokenType::Mul => "imul    ",
                TokenType::Div => "idiv    ",
                TokenType::BitOr => "or      ",
                TokenType::BitAnd => "and     ",
                _ => panic!("No valid bin op")
            });
    } else if node.node_type() == NodeType::FunctionDef {
        let func_def_node = node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap();

        let name = register_func(func_def_node, stack_frame, funcs);
        return format!("push    QWORD {}", &name);
    } else if node.node_type() == NodeType::Call {
        let call_node = node.as_any().downcast_ref::<CallNode>().unwrap();
        let mut temp = String::new();

        for (i, arg) in call_node.arg_nodes().iter().enumerate().rev() {
            write!(&mut temp, "{}", code_gen_asm(arg, stack_frame, funcs));
        }

        if !funcs.contains_key(call_node.func_to_call()) {
            panic!("Function not defined: '{}'!", call_node.func_to_call());
        }

        writeln!(&mut temp, "\tcall    {}", funcs.get(call_node.func_to_call()).unwrap().0);

        return temp;
    } else if node.node_type() == NodeType::VarAccess {
        let var_access_node = node.as_any().downcast_ref::<VarAccessNode>().unwrap();

        if !stack_frame.contains_key(var_access_node.var_name().as_str()) {
            panic!("Var not found!");
        }

        // return format!("{} {}", symbols.get(var_access_node.var_name().as_str()).unwrap().symbol_type(), symbols.get(var_access_node.var_name().as_str()).unwrap().content());
        return format!("\tpush    QWORD [rbp-({})]\n", stack_frame.get(var_access_node.var_name().as_str()).unwrap().to_string());
    } else if node.node_type() == NodeType::VarDeclaration {
        let var_declaration_node = node.as_any().downcast_ref::<VarDeclarationNode>().unwrap();
        let idx = stack_frame.len() + 8;
        stack_frame.insert(var_declaration_node.var_name().clone(), idx);
        return format!("{}\tpop     QWORD [rbp-({})]\n", code_gen_asm(var_declaration_node.value_node(), stack_frame, funcs), idx);
    }

    format!("\t;; TODO: ASM code gen for node type '{:?}' not implemented yet.", node.node_type())
}

pub fn to_asm(node: &Box<dyn Node>) -> String {
    let mut asm = String::new();

    let mut string_pool: HashMap<&str, String> = HashMap::new();

    let mut stack_frame: HashMap<String, usize> = HashMap::new();
    let mut funcs: HashMap<String, (String, String)> = HashMap::new();

    let mut global = String::new();

    code_gen_asm(node, &mut stack_frame, &mut funcs);

    // writeln!(&mut global, "{}", funs.iter().map(|&key, &value| format!("{}:\n{}", key, value)).)
    for (name, (id, content)) in funcs.iter() {
        writeln!(&mut global, "{}:\n{}\n", id, content);
    }
    // writeln!(&mut global, "{}", code_gen_asm(node, &mut stack_frame, &mut funs));

    writeln!(&mut asm, "default rel\nglobal main\n");
    writeln!(&mut asm, "{}", global);
    asm
}