
use std::collections::HashMap;
use std::fmt::Write;


use crate::nodes::{Node, NodeType};
use crate::nodes::asm_node::AssemblyNode;
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::number_node::NumberNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::string_node::StringNode;
use crate::nodes::var_node::access::VarAccessNode;
use crate::nodes::var_node::assign::VarAssignNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::token::TokenType;


const SCRATCH_REGS: [&str; 7] = [
    "rbx", "r10", "r11", "r12", "r13", "r14", "r15"
];

const NUMBER_ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub struct Compiler {
    scratch_regs: u8,
    label_count: u128,
    current_function_epilogue: Option<u128>,

    current_loop_start: Option<u128>,
    current_loop_break: Option<u128>,

    strings: HashMap<String, String>,

    base_offset: u64,
    offset_table: HashMap<String, i64>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scratch_regs: 0,
            label_count: 0,
            current_loop_start: None,
            current_loop_break: None,
            current_function_epilogue: None,
            strings: HashMap::new(),
            base_offset: 0,
            offset_table: HashMap::new(),
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
        return format!("L{}", label);
    }

    fn function_label_name(&self, function: &str) -> String {
        return format!("F_{}", function);
    }
    //endregion

    fn create_string_label(&mut self, string: String) -> String {
        if self.strings.contains_key(&string) {
            return self.strings[&string].clone();
        }

        let uuid = format!("S{}", self.strings.len());

        self.strings.insert(string.clone(), uuid);
        self.strings[&string].clone()
    }

    fn register_var(&mut self, name: String, size: u64) {
        self.base_offset += size;
        self.offset_table.insert(name, self.base_offset as i64);
    }

    fn get_var(&self, name: &str) -> i64 {
        self.offset_table[name]
    }

    fn clear_vars(&mut self) {
        self.base_offset = 0;
        self.offset_table.clear();
    }

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
            writeln!(w, "\tmov     {}, QWORD {}", self.scratch_name(reg), node.as_any().downcast_ref::<NumberNode>().unwrap().get_number());
            return Some(reg);
        }

        if node.node_type() == NodeType::String {
            let str_label = self.create_string_label(node.as_any().downcast_ref::<StringNode>().unwrap().get_string());
            let reg = self.res_scratch();
            writeln!(w, "\tmov     {}, {}", self.scratch_name(reg), str_label);
            return Some(reg);
        }

        if node.node_type() == NodeType::BinOp {
            let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();

            let left_reg = self.code_gen(bin_op_node.left_node(), w).unwrap();
            let right_reg = self.code_gen(bin_op_node.right_node(), w).unwrap();

            let mut res_reg = left_reg;

            if bin_op_node.op_token().token_type() == TokenType::Plus {
                writeln!(w, "\tadd     {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Minus {
                writeln!(w, "\tsub     {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Mul {
                writeln!(w, "\tmov     rax, {}", self.scratch_name(left_reg));
                writeln!(w, "\timul    {}", self.scratch_name(right_reg));
                res_reg = self.res_scratch();
                writeln!(w, "\tmov     {}, rax", self.scratch_name(res_reg));
                self.free_scratch(left_reg);
            } else {
                panic!("Token '{:?}' not supported as a binary operation yet!", bin_op_node.op_token().token_type());
            }

            self.free_scratch(right_reg);

            return Some(res_reg);
        }

        if node.node_type() == NodeType::Call {
            let call_node = node.as_any().downcast_ref::<CallNode>().unwrap();
            let func_label = self.function_label_name(call_node.func_to_call());

            let mut number_reg_index: usize = 0;
            for arg in call_node.arg_nodes() {
                let reg = self.code_gen(arg, w).unwrap();

                if number_reg_index >= NUMBER_ARG_REGS.len() {
                    writeln!(w, "\tpush    {}", self.scratch_name(reg));
                } else {
                    writeln!(w, "\tmov     {}, {}", self.scratch_name(reg), NUMBER_ARG_REGS[number_reg_index]);
                    number_reg_index += 1;
                }

                self.free_scratch(reg);
            }

            writeln!(w, "\tcall    {}", func_label);

            let reg = self.res_scratch();
            writeln!(w, "\tmov     {}, rax", self.scratch_name(reg));

            return Some(reg);
        }

        if node.node_type() == NodeType::FunctionDef {
            let func_def_node = node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap();
            let func_epilogue_label = self.label_create();

            self.clear_vars();

            writeln!(w, "{}:", self.function_label_name(func_def_node.var_name()));
            writeln!(w, "\tpush    rbp");
            writeln!(w, "\tmov     rbp, rsp");

            let mut number_reg_index: usize = 0;
            for (key, arg_type) in func_def_node.args() {
                self.register_var(key.clone(), arg_type.get_size());

                if number_reg_index >= NUMBER_ARG_REGS.len() {
                    let reg = self.res_scratch();
                    writeln!(w, "\tpop     {}", self.scratch_name(reg));
                    writeln!(w, "\tmov     QWORD [rbp - ({})], {}", self.base_offset, self.scratch_name(reg));
                    self.free_scratch(reg);
                } else {
                    writeln!(w, "\tmov     QWORD [rbp - ({})], {}", self.base_offset, NUMBER_ARG_REGS[number_reg_index]);
                    number_reg_index += 1;
                }
            }

            self.current_function_epilogue = Some(func_epilogue_label);

            self.code_gen(func_def_node.body_node(), w);

            writeln!(w, "{}:", self.label_name(&func_epilogue_label));
            writeln!(w, "\tmov     rsp, rbp");
            writeln!(w, "\tpop     rbp");
            writeln!(w, "\tret\n");

            return None
        }

        if node.node_type() == NodeType::Return {
            let return_node = node.as_any().downcast_ref::<ReturnNode>().unwrap();

            let reg: u8;
            if return_node.node_to_return().is_some() {
                reg = self.code_gen(return_node.node_to_return().as_ref().unwrap(), w).unwrap();
            } else {
                reg = self.res_scratch();
                writeln!(w, "\tmov     {}, 0", self.scratch_name(reg));
            }

            writeln!(w, "\tmov     rax, {}", self.scratch_name(reg));
            self.free_scratch(reg);

            writeln!(w, "\tjmp     {}", self.label_name(self.current_function_epilogue.as_ref().unwrap()));

            return None;
        }

        if node.node_type() == NodeType::Break {
            writeln!(w, "\tjmp     {}", self.label_name(self.current_loop_break.as_ref().unwrap()));
            return None;
        }

        if node.node_type() == NodeType::Continue {
            writeln!(w, "\tjmp     {}", self.label_name(self.current_loop_start.as_ref().unwrap()));
            return None;
        }

        if node.node_type() == NodeType::VarDeclaration {
            let var_declaration_node = node.as_any().downcast_ref::<VarDeclarationNode>().unwrap();

            self.register_var(var_declaration_node.var_name().to_string(), var_declaration_node.var_type().get_size());

            let reg = self.code_gen(var_declaration_node.value_node(), w).unwrap();
            writeln!(w, "\tmov     QWORD [rbp - ({})], {}", self.base_offset, self.scratch_name(reg));
            self.free_scratch(reg);

            return None;
        }

        if node.node_type() == NodeType::VarAssign {
            let var_assign_node = node.as_any().downcast_ref::<VarAssignNode>().unwrap();
            let var_offset = self.get_var(var_assign_node.var_name());

            let reg = self.code_gen(var_assign_node.value_node(), w).unwrap();
            writeln!(w, "\tmov     QWORD [rbp - ({})], {}", var_offset, self.scratch_name(reg));
            self.free_scratch(reg);

            return None;
        }

        if node.node_type() == NodeType::VarAccess {
            let var_access_node = node.as_any().downcast_ref::<VarAccessNode>().unwrap();

            let var_offset = self.get_var(var_access_node.var_name());
            let reg = self.res_scratch();
            writeln!(w, "\tmov     {}, QWORD [rbp - ({})]", self.scratch_name(reg), var_offset);

            return Some(reg);
        }

        None
    }

    pub fn compile_to_str(&mut self, node: &Box<dyn Node>) -> String {
        let mut res = String::new();

        let mut code = String::new();

        writeln!(code, "section .text\n");
        writeln!(code, "global  _start\n");

        writeln!(code, "_start:");
        writeln!(code, "\tcall    {}", self.function_label_name("main"));
        writeln!(code, "\tmov     rax, 60");
        writeln!(code, "\tmov     rdi, 0");
        writeln!(code, "\tsyscall");
        writeln!(code, "\tret\n");

        self.code_gen(node, &mut code);

        writeln!(res, "section .data");

        for (str, uuid) in &self.strings {
            writeln!(res, "\t{}  db \"{}\"", uuid, str);
        }

        writeln!(res, "\n{}", code);

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