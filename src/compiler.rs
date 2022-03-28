use std::alloc::handle_alloc_error;
use std::collections::HashMap;
use std::fmt::Write;
use std::hash::Hash;

use crate::nodes::{Node, NodeType};
use crate::nodes::asm_node::AssemblyNode;
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::number_node::NumberNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::statements_node::StatementsNode;
use crate::token::TokenType;
use crate::values::value_type::function_type::FunctionType;

const SCRATCH_REGS: [&str; 7] = [
    "%rbx", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15"
];

const NUMBER_ARG_REGS: [&str; 6] = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

pub struct Compiler {
    scratch_regs: u8,
    label_count: u128,
    current_function_epilogue: Option<u128>,

    current_loop_start: Option<u128>,
    current_loop_end: Option<u128>,

    function_names: HashMap<String, u128>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scratch_regs: 0,
            label_count: 0,
            current_loop_start: None,
            current_loop_end: None,
            current_function_epilogue: None,
            function_names: HashMap::new(),
        }
    }

    // region Register distribution
    fn res_scratch(&mut self) -> u8 {
        if self.scratch_regs ^ 0b1111111 == 0 {
            panic!("No free general purpose registers were found!");
        }

        let mut i = 0;
        let mut current = self.scratch_regs;
        while current & 1 == 1 {
            i += 1;
            current >>= 1;
        }

        self.scratch_regs |= 1 << i;

        i
    }

    fn scratch_name(&self, i: u8) -> &str { SCRATCH_REGS[i as usize] }

    fn free_scratch(&mut self, reg: u8) {
        self.scratch_regs = !(!self.scratch_regs | (1 << reg));
    }
    // endregion

    // region Label creation
    fn label_create(&mut self) -> u128 {
        self.label_count += 1;
        self.label_count
    }

    fn label_name(&self, label: &u128) -> String {
        return format!(".L{}", label);
    }
    //endregion

    pub fn scratch_regs(&self) -> &u8 { &self.scratch_regs }
    pub fn label_count(&self) -> &u128 { &self.label_count }
    pub fn current_function_epilogue(&self) -> &Option<u128> { &self.current_function_epilogue }
}

impl Compiler {
    fn code_gen(&mut self, node: &Box<dyn Node>, w: &mut String) -> Option<u8> {
        if node.node_type() == NodeType::Statements {
            for n in node.as_any().downcast_ref::<StatementsNode>().unwrap().statement_nodes() {
                let reg = self.code_gen(n, w);
                if reg.is_some() {
                    self.free_scratch(reg.unwrap());
                }
            }
            return None;
        }

        if node.node_type() == NodeType::Assembly {
            let assembly_node = node.as_any().downcast_ref::<AssemblyNode>().unwrap();
            writeln!(w, "\n;; Assembly injected im asm__(...)\n\t{}\n;; End\n", assembly_node.content());
        }

        if node.node_type() == NodeType::Number {
            let reg = self.res_scratch();
            writeln!(w, "\tMOVQ    {}, {}", node.as_any().downcast_ref::<NumberNode>().unwrap().get_number(), self.scratch_name(reg));
            return Some(reg);
        }

        if node.node_type() == NodeType::BinOp {
            let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();

            let left_reg = self.code_gen(bin_op_node.left_node(), w).unwrap();
            let right_reg = self.code_gen(bin_op_node.right_node(), w).unwrap();

            let mut res_reg = left_reg;

            if bin_op_node.op_token().token_type() == TokenType::Plus {
                writeln!(w, "\tADDQ    {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Minus {
                writeln!(w, "\tSUBQ    {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Mul {
                writeln!(w, "\tMOVQ    {}, %rax", self.scratch_name(left_reg));
                writeln!(w, "\tIMUL    {}", self.scratch_name(right_reg));
                res_reg = self.res_scratch();
                writeln!(w, "\tMOVQ    %rax, {}", self.scratch_name(res_reg));
                self.free_scratch(left_reg);
            } else {
                panic!("Token '{:?}' not supported as a binary operation yet!", bin_op_node.op_token().token_type());
            }

            self.free_scratch(right_reg);

            return Some(res_reg);
        }

        if node.node_type() == NodeType::Call {
            let call_node = node.as_any().downcast_ref::<CallNode>().unwrap();
            let func_label = *self.function_names.get(call_node.func_to_call()).unwrap();

            let mut number_reg_index: usize = 0;

            for arg in call_node.arg_nodes() {
                let reg = self.code_gen(arg, w).unwrap();

                if number_reg_index >= NUMBER_ARG_REGS.len() {
                    writeln!(w, "\tPUSHQ   {}", self.scratch_name(reg));
                } else {
                    writeln!(w, "\tMOVQ    {}, {}", self.scratch_name(reg), NUMBER_ARG_REGS[number_reg_index]);
                    number_reg_index += 1;
                }

                self.free_scratch(reg);
            }

            writeln!(w, "\tCALL    {}", self.label_name(&func_label));

            let reg = self.res_scratch();
            writeln!(w, "\tMOVQ    %rax, {}", self.scratch_name(reg));

            return Some(reg);
        }

        if node.node_type() == NodeType::FunctionDef {
            let func_def_node = node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap();
            let func_label = self.label_create();
            let func_epilogue_label = self.label_create();

            writeln!(w, "{}", self.label_name(&func_label));
            writeln!(w, "\tPUSHQ   %rbp");
            writeln!(w, "\tMOVQ    %rsp, %rbp");

            self.current_function_epilogue = Some(func_epilogue_label);
            self.function_names.insert(func_def_node.var_name().to_string(), func_label);

            self.code_gen(func_def_node.body_node(), w);

            writeln!(w, "{}", self.label_name(&func_epilogue_label));
            writeln!(w, "\tMOVQ    %rbp, %rsp");
            writeln!(w, "\tPOPQ    %rbp");
            writeln!(w, "\tRET");

            return None
        }

        if node.node_type() == NodeType::Return {
            let return_node = node.as_any().downcast_ref::<ReturnNode>().unwrap();

            let reg: u8;
            if return_node.node_to_return().is_some() {
                reg = self.code_gen(return_node.node_to_return().as_ref().unwrap(), w).unwrap();
            } else {
                reg = self.res_scratch();
                writeln!(w, "\tMOVQ    $0, {}", self.scratch_name(reg));
            }

            writeln!(w, "\tMOVQ    {}, %rax", self.scratch_name(reg));
            self.free_scratch(reg);

            writeln!(w, "\tJMP     {}", self.label_name(self.current_function_epilogue.as_ref().unwrap()));

            return None;
        }


        None
    }


    pub fn compile_to_str(&mut self, node: &Box<dyn Node>) -> String {
        let mut res = String::new();
        self.code_gen(node, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn compiler_register_distribution() {
        let mut c = Compiler::new();

        c.res_scratch();
        assert_eq!(*c.scratch_regs(), 0b0000001);

        c.res_scratch();
        assert_eq!(*c.scratch_regs(), 0b0000011);

        c.res_scratch();
        assert_eq!(*c.scratch_regs(), 0b0000111);

        c.free_scratch(1);
        assert_eq!(*c.scratch_regs(), 0b0000101);

        c.free_scratch(2);
        assert_eq!(*c.scratch_regs(), 0b0000001);

        c.free_scratch(0);
        assert_eq!(*c.scratch_regs(), 0b0000000);

        c.free_scratch(5);
        assert_eq!(*c.scratch_regs(), 0b0000000);

    }
}