use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

use crate::nodes::address_of_node::AddressOfNode;
use crate::nodes::array_node::ArrayNode;
use crate::nodes::asm_node::AssemblyNode;
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::cast_node::CastNode;
use crate::nodes::char_node::CharNode;
use crate::nodes::const_def_node::ConstDefinitionNode;
use crate::nodes::extern_node::ExternNode;
use crate::nodes::for_node::ForNode;
use crate::nodes::functiondecl_node::FunctionDeclarationNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::if_node::IfNode;
use crate::nodes::import_node::ImportNode;
use crate::nodes::number_node::NumberNode;
use crate::nodes::offset_node::OffsetNode;
use crate::nodes::pointer_assign_node::PointerAssignNode;
use crate::nodes::read_bytes_node::ReadBytesNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::sizeof_node::SizeOfNode;
use crate::nodes::stack_allocation_node::StackAllocationNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::static_decl_node::StaticDeclarationNode;
use crate::nodes::static_def_node::StaticDefinitionNode;
use crate::nodes::string_node::StringNode;
use crate::nodes::syscall_node::SyscallNode;
use crate::nodes::unaryop_node::UnaryOpNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::nodes::var_node::typed_access::VarTypedAccessNode;
use crate::nodes::var_node::typed_assign::VarTypedAssignNode;
use crate::nodes::while_node::WhileNode;
use crate::nodes::{Node, NodeType};
use crate::syscall::{CallingConvention, SyscallTable, TargetObjectType};
use crate::token::TokenType;
use crate::values::value_size::ValueSize;

const QW_SCRATCH_REGS: [&str; 7] = ["rbx", "r10", "r11", "r12", "r13", "r14", "r15"];
const DW_SCRATCH_REGS: [&str; 7] = ["ebx", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"];
const W_SCRATCH_REGS: [&str; 7] = ["bx", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"];
const B_SCRATCH_REGS: [&str; 7] = ["bl", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"];

const QW_NUMBER_ARG_REGS_UNIX: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
const DW_NUMBER_ARG_REGS_UNIX: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
const W_NUMBER_ARG_REGS_UNIX: [&str; 6] = ["di", "si", "dx", "cx", "r8w", "r9w"];
const B_NUMBER_ARG_REGS_UNIX: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];

const QW_NUMBER_ARG_REGS_WIN: [&str; 4] = ["rcx", "rdx", "r8", "r9"];
const DW_NUMBER_ARG_REGS_WIN: [&str; 4] = ["ecx", "edx", "r8d", "r9d"];
const W_NUMBER_ARG_REGS_WIN: [&str; 4] = ["cx", "dx", "r8w", "r9w"];
const B_NUMBER_ARG_REGS_WIN: [&str; 4] = ["cl", "dl", "r8b", "r9b"];

const SYSCALL_REGS: [&str; 4] = ["rax", "rdi", "rsi", "rdx"];

const ENTRY_SYMBOL: &str = "_start";

pub struct Compiler {
    scratch_regs: u8,
    label_count: u128,
    current_function_epilogue: Option<u128>,

    current_loop_start: Option<u128>,
    current_loop_break: Option<u128>,

    strings: HashMap<String, String>,
    constants: HashMap<String, String>,

    base_offset: u64,
    offset_table: HashMap<String, (u64, ValueSize)>,

    externs: Vec<String>,

    globals: Vec<String>,

    statics: HashMap<String, ValueSize>,

    target_object_type: TargetObjectType,
}

impl Compiler {
    pub fn new(target_object_type: TargetObjectType) -> Self {
        Compiler {
            scratch_regs: 0,
            label_count: 0,
            current_loop_start: None,
            current_loop_break: None,
            current_function_epilogue: None,
            strings: HashMap::new(),
            constants: HashMap::new(),
            base_offset: 0,
            offset_table: HashMap::new(),
            externs: vec![],
            globals: vec![],
            statics: HashMap::new(),
            target_object_type,
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

    fn scratch_name_lower_sized(&self, i: u8, size: &ValueSize) -> &str {
        match size {
            ValueSize::Byte => B_SCRATCH_REGS[i as usize],
            ValueSize::Word => W_SCRATCH_REGS[i as usize],
            ValueSize::Dword => DW_SCRATCH_REGS[i as usize],
            ValueSize::Qword => QW_SCRATCH_REGS[i as usize],
        }
    }

    fn scratch_name(&self, i: u8) -> &str {
        self.scratch_name_lower_sized(i, &ValueSize::Qword)
    }

    fn number_arg_reg_size(&self, calling_convention: &CallingConvention) -> usize {
        match calling_convention {
            CallingConvention::Unix => 6,
            CallingConvention::Win => 4,
        }
    }

    fn number_arg_reg_name(
        &self,
        i: u8,
        size: &ValueSize,
        calling_convention: &CallingConvention,
    ) -> &str {
        match size {
            ValueSize::Byte => match calling_convention {
                CallingConvention::Unix => B_NUMBER_ARG_REGS_UNIX[i as usize],
                CallingConvention::Win => B_NUMBER_ARG_REGS_WIN[i as usize],
            },
            ValueSize::Word => match calling_convention {
                CallingConvention::Unix => W_NUMBER_ARG_REGS_UNIX[i as usize],
                CallingConvention::Win => W_NUMBER_ARG_REGS_WIN[i as usize],
            },
            ValueSize::Dword => match calling_convention {
                CallingConvention::Unix => DW_NUMBER_ARG_REGS_UNIX[i as usize],
                CallingConvention::Win => DW_NUMBER_ARG_REGS_WIN[i as usize],
            },
            ValueSize::Qword => match calling_convention {
                CallingConvention::Unix => QW_NUMBER_ARG_REGS_UNIX[i as usize],
                CallingConvention::Win => QW_NUMBER_ARG_REGS_WIN[i as usize],
            },
        }
    }

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
        format!(".L{}", label)
    }

    fn function_label_name(&self, function: &str) -> String {
        format!("{}", function)
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

    fn register_var(&mut self, name: String, size: ValueSize) {
        self.base_offset += size.get_size_in_bytes() as u64;
        self.offset_table.insert(name, (self.base_offset, size));
    }

    fn get_var(&self, name: &str) -> (u64, ValueSize) {
        self.offset_table[name]
    }

    fn reset_stack_offset(&mut self) {
        self.base_offset = 0;
        self.offset_table.clear();
    }

    #[allow(dead_code)]
    fn add_extern(&mut self, s: String) {
        self.externs.push(s);
    }

    fn add_static(&mut self, s: String, size: ValueSize) {
        self.statics.insert(s, size);
    }

    fn add_global(&mut self, g: String) {
        self.globals.push(g);
    }

    fn is_static(&self, s: &str) -> bool {
        self.statics.contains_key(s)
    }

    #[allow(dead_code)]
    fn get_static(&self, s: &str) -> ValueSize {
        self.statics[s]
    }

    fn get_static_name(&self, s: &str) -> String {
        format!("ST_{}", s)
    }

    fn create_constant_label(&mut self, name: String) -> String {
        let uuid = format!("C{}", self.constants.len());

        self.constants.insert(name.clone(), uuid);
        self.constants[&name].clone()
    }

    fn get_constant(&self, name: &str) -> String {
        self.constants[name].clone()
    }

    fn is_constant(&self, name: &str) -> bool {
        self.constants.contains_key(name)
    }

    pub fn scratch_regs(&self) -> &u8 {
        &self.scratch_regs
    }
    pub fn label_count(&self) -> &u128 {
        &self.label_count
    }
    pub fn current_function_epilogue(&self) -> &Option<u128> {
        &self.current_function_epilogue
    }
}

impl Compiler {
    fn code_gen(&mut self, node: &Box<dyn Node>, w: &mut String) -> Result<Option<u8>, fmt::Error> {
        if node.node_type() == NodeType::Statements {
            for n in node
                .as_any()
                .downcast_ref::<StatementsNode>()
                .unwrap()
                .statement_nodes()
            {
                let reg = self.code_gen(n, w)?;
                if reg.is_some() {
                    self.free_scratch(reg.unwrap());
                }
            }
            return Ok(None);
        }

        if node.node_type() == NodeType::Assembly {
            let assembly_node = node.as_any().downcast_ref::<AssemblyNode>().unwrap();

            let reg = self.res_scratch();
            writeln!(w, "\tpush    rax")?;
            writeln!(w, "\t{}", assembly_node.content())?;
            writeln!(w, "\tmov     {}, rax", self.scratch_name(reg))?;
            writeln!(w, "\tpop     rax")?;

            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::Syscall {
            // TODO: push syscall regs

            let syscall_node = node.as_any().downcast_ref::<SyscallNode>().unwrap();
            writeln!(w, "\n;; Syscall injected")?;
            for i in 0..4 {
                let reg = self.code_gen(&syscall_node.args()[i], w)?.unwrap();
                writeln!(
                    w,
                    "\tmov     {}, {}",
                    SYSCALL_REGS[i],
                    self.scratch_name(reg)
                )?;
                self.free_scratch(reg);
            }
            writeln!(w, "\tsyscall\n;; End injected syscall\n")?;

            let result_reg = self.res_scratch();
            writeln!(w, "\tmov     {}, rax", self.scratch_name(result_reg))?;

            return Ok(Some(result_reg));
        }

        if node.node_type() == NodeType::Number {
            let number_node = node.as_any().downcast_ref::<NumberNode>().unwrap();

            let reg = self.res_scratch();
            if number_node.size().get_size() != ValueSize::Qword {
                writeln!(
                    w,
                    "\txor     {}, {}",
                    self.scratch_name(reg),
                    self.scratch_name(reg)
                )?;
            }
            writeln!(
                w,
                "\tmov     {}, {} {}",
                self.scratch_name_lower_sized(reg, &number_node.size().get_size()),
                number_node.size().get_size(),
                number_node.get_number()
            )?;
            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::String {
            let str_label = self.create_string_label(
                node.as_any()
                    .downcast_ref::<StringNode>()
                    .unwrap()
                    .get_string(),
            );
            let reg = self.res_scratch();

            if self.target_object_type.should_use_rel() {
                writeln!(
                    w,
                    "\tlea     {}, [rel {}]",
                    self.scratch_name(reg),
                    str_label
                )?;
            } else {
                writeln!(
                    w,
                    "\tmov     {}, QWORD {}",
                    self.scratch_name(reg),
                    str_label
                )?;
            }
            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::Char {
            let reg = self.res_scratch();
            writeln!(
                w,
                "\txor     {}, {}",
                self.scratch_name(reg),
                self.scratch_name(reg)
            )?;
            writeln!(
                w,
                "\tmov     {}, BYTE {}",
                self.scratch_name_lower_sized(reg, &ValueSize::Byte),
                *node.as_any().downcast_ref::<CharNode>().unwrap().value() as u8
            )?;
            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::Array {
            let array_node = node.as_any().downcast_ref::<ArrayNode>().unwrap();

            let beginning_offset = self.base_offset;
            self.base_offset += *array_node.size() as u64
                * array_node.element_type().get_size().get_size_in_bytes() as u64;

            if array_node.element_nodes().is_empty() {
                println!("it is empty");
                for i in 0..*array_node.size() {
                    writeln!(
                        w,
                        "\tmov     {} [rbp-{}], {}",
                        array_node.element_type().get_size(),
                        beginning_offset
                            + ((i + 1) as u64
                                * array_node.element_type().get_size().get_size_in_bytes() as u64),
                        0
                    )?;
                }
            }

            for (i, elem) in array_node.element_nodes().iter().rev().enumerate() {
                let reg = self.code_gen(elem, w)?.unwrap();
                writeln!(
                    w,
                    "\tmov     [rbp-{}], {}",
                    beginning_offset
                        + ((i + 1) as u64
                            * array_node.element_type().get_size().get_size_in_bytes() as u64),
                    self.scratch_name(reg)
                )?;
                self.free_scratch(reg);
            }

            let reg = self.res_scratch();
            writeln!(
                w,
                "\tlea     {}, [rbp-{}]",
                self.scratch_name(reg),
                self.base_offset
            )?;
            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::BinOp {
            let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();

            // Special case for And and Or operators, which need to handle short-circuiting
            if bin_op_node.op_token().token_type() == TokenType::And {
                let label_true = self.label_create();
                let label_false = self.label_create();
                let label_end = self.label_create();

                let left_reg = self.code_gen(&bin_op_node.left_node(), w)?.unwrap();
                writeln!(w, "\tcmp     {}, 0", self.scratch_name(left_reg))?;
                writeln!(w, "\tje      {}", self.label_name(&label_false))?;

                let right_reg = self.code_gen(&bin_op_node.right_node(), w)?.unwrap();
                writeln!(w, "\tcmp     {}, 0", self.scratch_name(right_reg))?;
                self.free_scratch(right_reg);
                writeln!(w, "\tje      {}", self.label_name(&label_false))?;

                writeln!(w, "{}:", self.label_name(&label_true))?;
                writeln!(w, "\tmov     {}, 1", self.scratch_name(left_reg))?;
                writeln!(w, "\tjmp     {}", self.label_name(&label_end))?;

                writeln!(w, "{}:", self.label_name(&label_false))?;
                writeln!(w, "\tmov     {}, 0", self.scratch_name(left_reg))?;

                writeln!(w, "{}:", self.label_name(&label_end))?;

                return Ok(Some(left_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Or {
                let label_true = self.label_create();
                let label_false = self.label_create();
                let label_end = self.label_create();

                let left_reg = self.code_gen(&bin_op_node.left_node(), w)?.unwrap();
                writeln!(w, "\tcmp     {}, 0", self.scratch_name(left_reg))?;
                writeln!(w, "\tjne     {}", self.label_name(&label_true))?;

                let right_reg = self.code_gen(&bin_op_node.right_node(), w)?.unwrap();
                writeln!(w, "\tcmp     {}, 0", self.scratch_name(right_reg))?;
                self.free_scratch(right_reg);
                writeln!(w, "\tje      {}", self.label_name(&label_false))?;

                writeln!(w, "{}:", self.label_name(&label_true))?;
                writeln!(w, "\tmov     {}, 1", self.scratch_name(left_reg))?;
                writeln!(w, "\tjmp     {}", self.label_name(&label_end))?;

                writeln!(w, "{}:", self.label_name(&label_false))?;
                writeln!(w, "\tmov     {}, 0", self.scratch_name(left_reg))?;

                writeln!(w, "{}:", self.label_name(&label_end))?;

                return Ok(Some(left_reg));
            }

            let left_reg = self.code_gen(bin_op_node.left_node(), w)?.unwrap();
            let right_reg = self.code_gen(bin_op_node.right_node(), w)?.unwrap();

            let res_reg = left_reg;

            if bin_op_node.op_token().token_type() == TokenType::Plus {
                writeln!(
                    w,
                    "\tadd     {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else if bin_op_node.op_token().token_type() == TokenType::Minus {
                writeln!(
                    w,
                    "\tsub     {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else if bin_op_node.op_token().token_type() == TokenType::Mul {
                writeln!(w, "\tpush    rax")?;
                writeln!(w, "\tpush    rdx")?;

                writeln!(w, "\tmov     rax, {}", self.scratch_name(left_reg))?;
                writeln!(w, "\timul    {}", self.scratch_name(right_reg))?;

                writeln!(w, "\tmov     {}, rax", self.scratch_name(res_reg))?;

                writeln!(w, "\tpop     rdx")?;
                writeln!(w, "\tpop     rax")?;
            } else if bin_op_node.op_token().token_type() == TokenType::Div {
                writeln!(w, "\tpush    rax")?;
                writeln!(w, "\tpush    rdx")?;

                writeln!(w, "\txor     rdx, rdx")?;
                writeln!(w, "\tmov     rax, {}", self.scratch_name(left_reg))?;
                writeln!(w, "\tidiv    {}", self.scratch_name(right_reg))?;

                writeln!(w, "\tmov     {}, rax", self.scratch_name(left_reg))?;

                writeln!(w, "\tpop     rdx")?;
                writeln!(w, "\tpop     rax")?;
            } else if bin_op_node.op_token().token_type() == TokenType::Modulo {
                writeln!(w, "\tpush    rax")?;
                writeln!(w, "\tpush    rdx")?;

                writeln!(w, "\txor     rdx, rdx")?;
                writeln!(w, "\tmov     rax, {}", self.scratch_name(left_reg))?;
                writeln!(w, "\tidiv    {}", self.scratch_name(right_reg))?;

                writeln!(w, "\tmov     {}, rdx", self.scratch_name(left_reg))?;

                writeln!(w, "\tpop     rdx")?;
                writeln!(w, "\tpop     rax")?;
            } else if bin_op_node.op_token().token_type() == TokenType::BitAnd {
                writeln!(
                    w,
                    "\tand     {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else if bin_op_node.op_token().token_type() == TokenType::BitOr {
                writeln!(
                    w,
                    "\tor      {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else if bin_op_node.op_token().token_type() == TokenType::BitXor {
                writeln!(
                    w,
                    "\txor     {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else if bin_op_node.op_token().token_type() == TokenType::BitShl {
                writeln!(w, "\tpush    rcx")?;
                writeln!(w, "\tmov     rcx, {}", self.scratch_name(right_reg))?;
                writeln!(w, "\tsal     {}, cl", self.scratch_name(left_reg))?;
                writeln!(w, "\tpop     rcx")?;
            } else if bin_op_node.op_token().token_type() == TokenType::BitShr {
                writeln!(w, "\tpush    rcx")?;
                writeln!(w, "\tmov     rcx, {}", self.scratch_name(right_reg))?;
                writeln!(w, "\tsar     {}, cl", self.scratch_name(left_reg))?;
                writeln!(w, "\tpop     rcx")?;
            } else if bin_op_node.op_token().token_type() == TokenType::Ee
                || bin_op_node.op_token().token_type() == TokenType::Gt
                || bin_op_node.op_token().token_type() == TokenType::Lt
                || bin_op_node.op_token().token_type() == TokenType::Gte
                || bin_op_node.op_token().token_type() == TokenType::Lte
                || bin_op_node.op_token().token_type() == TokenType::Ne
            {
                writeln!(
                    w,
                    "\tcmp     {}, {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
                let label_true = self.label_create();
                let label_after = self.label_create();

                if bin_op_node.op_token().token_type() == TokenType::Ee {
                    writeln!(w, "\tje      {}", self.label_name(&label_true))?;
                } else if bin_op_node.op_token().token_type() == TokenType::Lt {
                    writeln!(w, "\tjl      {}", self.label_name(&label_true))?;
                } else if bin_op_node.op_token().token_type() == TokenType::Gt {
                    writeln!(w, "\tjg      {}", self.label_name(&label_true))?;
                } else if bin_op_node.op_token().token_type() == TokenType::Lte {
                    writeln!(w, "\tjle     {}", self.label_name(&label_true))?;
                } else if bin_op_node.op_token().token_type() == TokenType::Gte {
                    writeln!(w, "\tjge     {}", self.label_name(&label_true))?;
                } else if bin_op_node.op_token().token_type() == TokenType::Ne {
                    writeln!(w, "\tjne     {}", self.label_name(&label_true))?;
                }

                writeln!(w, "\tmov     {}, QWORD 0", self.scratch_name(res_reg))?;
                writeln!(w, "\tjmp     {}", self.label_name(&label_after))?;

                writeln!(w, "{}:", self.label_name(&label_true))?;
                writeln!(w, "\tmov     {}, QWORD 1", self.scratch_name(res_reg))?;

                writeln!(w, "{}:", self.label_name(&label_after))?;
            } else {
                panic!(
                    "Token '{:?}' not supported as a binary operation yet!",
                    bin_op_node.op_token().token_type()
                );
            }

            self.free_scratch(right_reg);

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::UnaryOp {
            let unary_op_node = node.as_any().downcast_ref::<UnaryOpNode>().unwrap();

            let left = self.code_gen(unary_op_node.node(), w)?.unwrap();
            let mut res_reg = left;

            if unary_op_node.op_token().token_type() == TokenType::Minus {
                todo!("Unary minus");
            } else if unary_op_node.op_token().token_type() == TokenType::Not {
                let label_true = self.label_create();
                let label_after = self.label_create();

                res_reg = self.res_scratch();
                writeln!(w, "\tcmp     {}, QWORD 0", self.scratch_name(left))?;
                self.free_scratch(left);
                writeln!(w, "\tje      {}", self.label_name(&label_true))?;
                writeln!(w, "\tmov     {}, QWORD 0", self.scratch_name(res_reg))?;
                writeln!(w, "\tjmp     {}", self.label_name(&label_after))?;
                writeln!(w, "{}:", self.label_name(&label_true))?;
                writeln!(w, "\tmov     {}, QWORD 1", self.scratch_name(res_reg))?;
                writeln!(w, "{}:", self.label_name(&label_after))?;
            } else if unary_op_node.op_token().token_type() == TokenType::BitNot {
                writeln!(w, "\tnot     {}", self.scratch_name(res_reg))?;
            } else {
                panic!(
                    "Token '{:?}' not supported as an unary operation yet!",
                    unary_op_node.op_token().token_type()
                );
            }

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::Call {
            let call_node = node.as_any().downcast_ref::<CallNode>().unwrap();
            let func_label = if self.externs.contains(&call_node.func_to_call().to_string()) {
                call_node.func_to_call().to_string()
            } else {
                self.function_label_name(call_node.func_to_call())
            };

            writeln!(w, "\tpush    r10")?;
            writeln!(w, "\tpush    r11")?;

            for arg in call_node.arg_nodes().iter().rev() {
                let reg = self.code_gen(arg, w)?.unwrap();
                writeln!(w, "\tpush    {}", self.scratch_name(reg))?;
                self.free_scratch(reg);
            }

            for (i, _arg) in call_node.arg_nodes().iter().enumerate() {
                if i >= self.number_arg_reg_size(&self.target_object_type.calling_convention()) {
                    break;
                };
                writeln!(
                    w,
                    "\tpop     {}",
                    self.number_arg_reg_name(
                        i as u8,
                        &ValueSize::Qword,
                        &self.target_object_type.calling_convention()
                    )
                )?;
            }

            writeln!(w, "\tcall    {}", func_label)?;

            if call_node.arg_nodes().len() > QW_NUMBER_ARG_REGS_UNIX.len() {
                writeln!(
                    w,
                    "\tadd     rsp, {}",
                    (call_node.arg_nodes().len() - QW_NUMBER_ARG_REGS_UNIX.len()) * 8
                )?;
            }

            writeln!(w, "\tpop     r11")?;
            writeln!(w, "\tpop     r10")?;

            let reg = self.res_scratch();
            writeln!(w, "\tmov     {}, rax", self.scratch_name(reg))?;

            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::FunctionDef {
            let func_def_node = node
                .as_any()
                .downcast_ref::<FunctionDefinitionNode>()
                .unwrap();
            let func_epilogue_label = self.label_create();

            self.reset_stack_offset();

            let func_label = self.function_label_name(func_def_node.var_name());

            self.add_global(func_label.clone());
            writeln!(w, "{}:", func_label)?;

            writeln!(w, "\tpush    rbp")?;
            writeln!(w, "\tmov     rbp, rsp")?;

            let mut function_body = String::new();

            for (i, (key, arg_type)) in func_def_node.args().iter().enumerate() {
                self.register_var(key.clone(), arg_type.get_size());

                let num_arg_regs =
                    self.number_arg_reg_size(&self.target_object_type.calling_convention());
                if i >= num_arg_regs {
                    let reg = self.res_scratch();

                    let non_reg_index = i - num_arg_regs + 1;

                    writeln!(
                        &mut function_body,
                        "\tmov     {}, QWORD [rbp + {}]",
                        self.scratch_name(reg),
                        (non_reg_index * 8) + 8
                    )?;
                    writeln!(
                        &mut function_body,
                        "\tmov     {} [rbp - ({})], {}",
                        arg_type.get_size(),
                        self.base_offset,
                        self.scratch_name_lower_sized(reg, &arg_type.get_size())
                    )?;
                    self.free_scratch(reg);
                } else {
                    writeln!(
                        &mut function_body,
                        "\tmov     {} [rbp - ({})], {}",
                        arg_type.get_size(),
                        self.base_offset,
                        self.number_arg_reg_name(
                            i as u8,
                            &arg_type.get_size(),
                            &self.target_object_type.calling_convention()
                        )
                    )?;
                }
            }

            self.current_function_epilogue = Some(func_epilogue_label);

            self.code_gen(func_def_node.body_node(), &mut function_body)?;

            if self.base_offset > 0 {
                writeln!(w, "\tsub     rsp, {}", self.base_offset)?;
            }

            writeln!(w, "\tpush    rbx")?;
            writeln!(w, "\tpush    r12")?;
            writeln!(w, "\tpush    r13")?;
            writeln!(w, "\tpush    r14")?;
            writeln!(w, "\tpush    r15")?;

            writeln!(w, "{}", function_body)?;

            writeln!(w, "{}:", self.label_name(&func_epilogue_label))?;

            writeln!(w, "\tpop     r15")?;
            writeln!(w, "\tpop     r14")?;
            writeln!(w, "\tpop     r13")?;
            writeln!(w, "\tpop     r12")?;
            writeln!(w, "\tpop     rbx")?;

            writeln!(w, "\tmov     rsp, rbp")?;
            writeln!(w, "\tpop     rbp")?;
            writeln!(w, "\tret\n")?;

            return Ok(None);
        }

        if node.node_type() == NodeType::Return {
            let return_node = node.as_any().downcast_ref::<ReturnNode>().unwrap();

            let reg: u8;
            if return_node.node_to_return().is_some() {
                reg = self
                    .code_gen(return_node.node_to_return().as_ref().unwrap(), w)?
                    .unwrap();
            } else {
                reg = self.res_scratch();
                writeln!(w, "\tmov     {}, 0", self.scratch_name(reg))?;
            }

            writeln!(w, "\tmov     rax, {}", self.scratch_name(reg))?;
            self.free_scratch(reg);

            writeln!(
                w,
                "\tjmp     {}",
                self.label_name(self.current_function_epilogue.as_ref().unwrap())
            )?;

            return Ok(None);
        }

        if node.node_type() == NodeType::Break {
            writeln!(
                w,
                "\tjmp     {}",
                self.label_name(self.current_loop_break.as_ref().unwrap())
            )?;
            return Ok(None);
        }

        if node.node_type() == NodeType::Continue {
            writeln!(
                w,
                "\tjmp     {}",
                self.label_name(self.current_loop_start.as_ref().unwrap())
            )?;
            return Ok(None);
        }

        if node.node_type() == NodeType::VarDeclaration {
            let var_declaration_node = node.as_any().downcast_ref::<VarDeclarationNode>().unwrap();

            let result_reg = self
                .code_gen(var_declaration_node.value_node(), w)?
                .unwrap();

            self.register_var(
                var_declaration_node.var_name().to_string(),
                var_declaration_node.var_type().get_size(),
            );

            writeln!(
                w,
                "\tmov     {} [rbp - ({})], {}",
                var_declaration_node.var_type().get_size(),
                self.base_offset,
                self.scratch_name_lower_sized(
                    result_reg,
                    &var_declaration_node.var_type().get_size()
                )
            )?;

            return Ok(Some(result_reg));
        }

        if node.node_type() == NodeType::VarTypedAssign {
            let var_assign_node = node.as_any().downcast_ref::<VarTypedAssignNode>().unwrap();

            let reg = self.code_gen(var_assign_node.value_node(), w)?.unwrap();

            if self.is_static(var_assign_node.var_name()) {
                writeln!(
                    w,
                    "\tmov     [{}{}], {}",
                    if self.target_object_type.should_use_rel() {
                        "rel "
                    } else {
                        ""
                    },
                    self.get_static_name(var_assign_node.var_name()),
                    self.scratch_name(reg)
                )?;
                return Ok(Some(reg));
            }

            let (var_offset, _) = self.get_var(var_assign_node.var_name());

            writeln!(
                w,
                "\tmov     {} [rbp - ({})], {}",
                var_assign_node.value_type().get_size(),
                var_offset,
                self.scratch_name_lower_sized(reg, &var_assign_node.value_type().get_size())
            )?;

            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::VarTypedAccess {
            let var_access_node = node.as_any().downcast_ref::<VarTypedAccessNode>().unwrap();

            if self.is_constant(var_access_node.var_name()) {
                todo!("constants are not implemented yet");
                #[allow(unreachable_code)]
                {
                    let reg = self.res_scratch();
                    writeln!(
                        w,
                        "\tmov     {}, {}",
                        self.scratch_name(reg),
                        self.get_constant(var_access_node.var_name())
                    )?;
                    return Ok(Some(reg));
                }
            }

            if self.is_static(var_access_node.var_name()) {
                let reg = self.res_scratch();
                writeln!(
                    w,
                    "\tmov     {}, [{}{}]",
                    self.scratch_name(reg),
                    if self.target_object_type.should_use_rel() {
                        "rel "
                    } else {
                        ""
                    },
                    self.get_static_name(var_access_node.var_name())
                )?;
                return Ok(Some(reg));
            }

            let (var_offset, _) = self.get_var(var_access_node.var_name());
            let reg = self.res_scratch();

            let var_size = var_access_node.value_type().get_size();

            if var_size != ValueSize::Qword {
                writeln!(
                    w,
                    "\txor     {}, {}",
                    self.scratch_name(reg),
                    self.scratch_name(reg)
                )?;
            }

            writeln!(
                w,
                "\tmov     {}, {} [rbp - ({})]",
                self.scratch_name_lower_sized(reg, &var_size),
                var_size,
                var_offset
            )?;

            return Ok(Some(reg));
        }

        if node.node_type() == NodeType::While {
            let while_node = node.as_any().downcast_ref::<WhileNode>().unwrap();

            let label_start = self.label_create();
            let label_end = self.label_create();

            let prev_loop_start = self.current_loop_start;
            let prev_loop_break = self.current_loop_break;

            self.current_loop_start = Some(label_start);
            self.current_loop_break = Some(label_end);

            writeln!(w, "{}:", self.label_name(&label_start))?;

            let condition_reg = self.code_gen(while_node.condition_node(), w)?.unwrap();
            writeln!(w, "\tcmp     {}, 0", self.scratch_name(condition_reg))?;
            self.free_scratch(condition_reg);
            writeln!(w, "\tje      {}", self.label_name(&label_end))?;

            self.code_gen(while_node.body_node(), w)?;
            writeln!(w, "\tjmp     {}", self.label_name(&label_start))?;

            writeln!(w, "{}:", self.label_name(&label_end))?;

            self.current_loop_start = prev_loop_start;
            self.current_loop_break = prev_loop_break;

            return Ok(None);
        }

        if node.node_type() == NodeType::For {
            let for_node = node.as_any().downcast_ref::<ForNode>().unwrap();

            let prev_loop_start = self.current_loop_start;
            let prev_loop_break = self.current_loop_break;

            let label_start = self.label_create();
            let label_next = self.label_create();
            let label_end = self.label_create();

            self.current_loop_start = Some(label_next);
            self.current_loop_break = Some(label_end);

            let init_reg = self.code_gen(for_node.init_stmt(), w)?;
            if let Some(init_reg) = init_reg {
                self.free_scratch(init_reg);
            }

            writeln!(w, "{}:", self.label_name(&label_start))?;
            let condition_reg = self.code_gen(for_node.condition(), w)?.unwrap();
            writeln!(w, "\tcmp     {}, 0", self.scratch_name(condition_reg))?;
            self.free_scratch(condition_reg);
            writeln!(w, "\tje      {}", self.label_name(&label_end))?;

            let body_reg = self.code_gen(for_node.body(), w)?;
            if let Some(body_reg) = body_reg {
                self.free_scratch(body_reg);
            }

            writeln!(w, "{}:", self.label_name(&label_next))?;
            let next_reg = self.code_gen(for_node.next_expr(), w)?;
            if let Some(next_reg) = next_reg {
                self.free_scratch(next_reg);
            }

            writeln!(w, "\tjmp     {}", self.label_name(&label_start))?;
            writeln!(w, "{}:", self.label_name(&label_end))?;

            self.current_loop_start = prev_loop_start;
            self.current_loop_break = prev_loop_break;

            return Ok(None);
        }

        if node.node_type() == NodeType::If {
            let if_node = node.as_any().downcast_ref::<IfNode>().unwrap();

            let label_else = self.label_create();
            let label_end = self.label_create();

            let mut case_labels: Vec<u128> = vec![];
            case_labels.reserve(if_node.cases().len());

            for _ in if_node.cases() {
                case_labels.push(self.label_create());
            }

            for i in 0..if_node.cases().len() {
                let case = &if_node.cases()[i];

                writeln!(w, "{}:", self.label_name(&case_labels[i]))?;
                let condition_reg = self.code_gen(case.condition(), w)?.unwrap();
                writeln!(w, "\tcmp     {}, 0", self.scratch_name(condition_reg))?;
                self.free_scratch(condition_reg);

                writeln!(
                    w,
                    "\tje      {}",
                    if i == if_node.cases().len() - 1 {
                        self.label_name(&label_else)
                    } else {
                        self.label_name(&case_labels[i + 1])
                    }
                )?;
                self.code_gen(case.statements(), w)?;
                writeln!(w, "\tjmp     {}", self.label_name(&label_end))?;
            }

            writeln!(w, "{}:", self.label_name(&label_else))?;
            if if_node.else_case().is_some() {
                self.code_gen(if_node.else_case().as_ref().unwrap().statements(), w)?;
            }

            writeln!(w, "{}:", self.label_name(&label_end))?;

            return Ok(None);
        }

        if node.node_type() == NodeType::Cast {
            let cast_node = node.as_any().downcast_ref::<CastNode>().unwrap();
            let from_reg = self.code_gen(cast_node.node(), w)?.unwrap();
            let res_reg = self.res_scratch();

            if cast_node.cast_type().get_size() != ValueSize::Qword {
                writeln!(
                    w,
                    "\txor     {}, {}",
                    self.scratch_name(res_reg),
                    self.scratch_name(res_reg)
                )?;
            }

            writeln!(
                w,
                "\tmov     {}, {}",
                self.scratch_name_lower_sized(res_reg, &cast_node.cast_type().get_size()),
                self.scratch_name_lower_sized(from_reg, &cast_node.cast_type().get_size())
            )?;
            self.free_scratch(from_reg);

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::ConstDef {
            let const_def_node = node.as_any().downcast_ref::<ConstDefinitionNode>().unwrap();
            self.create_constant_label(const_def_node.name().to_string());
            return Ok(None);
        }

        if node.node_type() == NodeType::SizeOf {
            let size_of_node = node.as_any().downcast_ref::<SizeOfNode>().unwrap();

            let res_reg = self.res_scratch();
            writeln!(
                w,
                "\tmov     {}, {}",
                self.scratch_name(res_reg),
                size_of_node.value_type().get_size().get_size_in_bytes()
            )?;

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::StaticDef {
            let static_def_node = node
                .as_any()
                .downcast_ref::<StaticDefinitionNode>()
                .unwrap();

            self.add_static(
                static_def_node.name().to_string(),
                static_def_node.value_type().get_size(),
            );

            return Ok(None);
        }

        if node.node_type() == NodeType::ReadBytes {
            let read_bytes_node = node.as_any().downcast_ref::<ReadBytesNode>().unwrap();

            let from_reg = self.code_gen(read_bytes_node.node(), w)?.unwrap();

            let res_reg = self.res_scratch();

            if *read_bytes_node.bytes() != ValueSize::Qword {
                writeln!(
                    w,
                    "\txor     {}, {}",
                    self.scratch_name(res_reg),
                    self.scratch_name(res_reg)
                )?;
            }
            writeln!(
                w,
                "\tmov     {}, {} [{}]",
                self.scratch_name_lower_sized(res_reg, read_bytes_node.bytes()),
                read_bytes_node.bytes(),
                self.scratch_name(from_reg)
            )?;

            self.free_scratch(from_reg);

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::PointerAssign {
            let pointer_assign_node = node.as_any().downcast_ref::<PointerAssignNode>().unwrap();
            let left_reg = self.code_gen(pointer_assign_node.ptr(), w)?.unwrap();
            let right_reg = self.code_gen(pointer_assign_node.value(), w)?.unwrap();

            if pointer_assign_node.pointee_type().get_size() == ValueSize::Qword {
                writeln!(
                    w,
                    "\tmov     [{}], {}",
                    self.scratch_name(left_reg),
                    self.scratch_name(right_reg)
                )?;
            } else {
                writeln!(
                    w,
                    "\tmov     {} [{}], {}",
                    pointer_assign_node.pointee_type().get_size(),
                    self.scratch_name(left_reg),
                    self.scratch_name_lower_sized(
                        right_reg,
                        &pointer_assign_node.pointee_type().get_size()
                    )
                )?;
            }

            self.free_scratch(left_reg);
            return Ok(Some(right_reg));
        }

        if node.node_type() == NodeType::Offset {
            let offset_node = node.as_any().downcast_ref::<OffsetNode>().unwrap();
            let left_reg = self.code_gen(offset_node.node(), w)?.unwrap();
            let right_reg = self.code_gen(offset_node.offset_node(), w)?.unwrap();
            // let res_reg = self.res_scratch();

            writeln!(w, "\tpush    rax")?;
            writeln!(w, "\tpush    rdx")?;

            writeln!(w, "\tmov     rax, {}", self.scratch_name(right_reg))?;
            writeln!(
                w,
                "\tmov     {}, {}",
                self.scratch_name(right_reg),
                offset_node.pointee_type().get_size().get_size_in_bytes()
            )?;
            writeln!(w, "\timul    {}", self.scratch_name(right_reg))?;

            writeln!(w, "\tmov     {}, rax", self.scratch_name(right_reg))?;

            writeln!(w, "\tpop     rdx")?;
            writeln!(w, "\tpop     rax")?;

            writeln!(
                w,
                "\tadd     {}, {}",
                self.scratch_name(left_reg),
                self.scratch_name(right_reg)
            )?;

            /*if offset_node.pointee_type().get_size() != ValueSize::QWORD {
                writeln!(w, "\txor     {}, {}", self.scratch_name(res_reg), self.scratch_name(res_reg));
            }
            writeln!(w, "\tmov     {}, {} [{}]", self.scratch_name_lower_sized(res_reg, &offset_node.pointee_type().get_size()), offset_node.pointee_type().get_size(), self.scratch_name(left_reg));
             */

            // self.free_scratch(left_reg);
            self.free_scratch(right_reg);

            return Ok(Some(left_reg));
        }

        if node.node_type() == NodeType::Import {
            let import_node = node.as_any().downcast_ref::<ImportNode>().unwrap();
            self.code_gen(import_node.node(), w)?;
            return Ok(None);
        }

        if node.node_type() == NodeType::Extern {
            let extern_node = node.as_any().downcast_ref::<ExternNode>().unwrap();

            match extern_node.top_level_statement().node_type() {
                NodeType::FunctionDecl => {
                    let func_decl_node = extern_node
                        .top_level_statement()
                        .as_any()
                        .downcast_ref::<FunctionDeclarationNode>()
                        .unwrap();
                    self.add_extern(func_decl_node.var_name().to_string());
                }
                NodeType::StaticDecl => {
                    let static_decl_node = extern_node
                        .top_level_statement()
                        .as_any()
                        .downcast_ref::<StaticDeclarationNode>()
                        .unwrap();
                    self.add_extern(static_decl_node.name().to_string());
                }
                _ => unreachable!(),
            }

            return Ok(None);
        }

        if node.node_type() == NodeType::AddressOf {
            let address_of_node = node.as_any().downcast_ref::<AddressOfNode>().unwrap();

            let res_reg = self.res_scratch();

            if self.is_static(address_of_node.var_name()) {
                writeln!(
                    w,
                    "\tmov     {}, {}",
                    self.scratch_name(res_reg),
                    self.get_static_name(address_of_node.var_name())
                )?;
                return Ok(Some(res_reg));
            }

            let (var_offset, _) = self.get_var(address_of_node.var_name());

            writeln!(
                w,
                "\tlea     {}, [rbp - {}]",
                self.scratch_name(res_reg),
                var_offset
            )?;

            return Ok(Some(res_reg));
        }

        if node.node_type() == NodeType::StackAllocationNode {
            let stack_allocation_node =
                node.as_any().downcast_ref::<StackAllocationNode>().unwrap();

            let beginning_offset = self.base_offset;
            self.base_offset += stack_allocation_node.size_in_bytes();

            for i in 0..*stack_allocation_node.size_in_bytes() {
                writeln!(
                    w,
                    "\tmov     BYTE [rbp-{}], {}",
                    beginning_offset + (i + 1),
                    0
                )?;
            }

            let reg = self.res_scratch();
            writeln!(
                w,
                "\tlea     {}, [rbp-{}]",
                self.scratch_name(reg),
                self.base_offset
            )?;
            return Ok(Some(reg));
        }

        Ok(None)
    }

    pub fn compile_to_str(
        &mut self,
        node: &Box<dyn Node>,
        no_entry: bool,
        arch: TargetObjectType,
    ) -> Result<String, fmt::Error> {
        let mut res = String::new();

        let mut code = String::new();
        self.code_gen(node, &mut code)?;

        // region .data
        writeln!(res, "section .data")?;

        writeln!(res, "\t;; Static strings")?;
        for (str, uuid) in &self.strings {
            writeln!(res, "\t{}: db  `{}`, 0", uuid, str)?;
        }

        writeln!(res, "section .bss")?;
        writeln!(res, "\t;; Other statics")?;
        for (name, size) in &self.statics {
            writeln!(
                res,
                "\t{}  RESB {}",
                self.get_static_name(name),
                size.get_size_in_bytes()
            )?;
        }
        // endregion

        writeln!(res, "\nsection .text\n")?;

        if !no_entry {
            self.add_global(ENTRY_SYMBOL.to_string());

            writeln!(code, "{}:", ENTRY_SYMBOL)?;
            writeln!(code, "\tpop     rdi")?;
            writeln!(code, "\tpop     rsi")?;

            writeln!(code, "\tcall    {}", self.function_label_name("main"))?;

            writeln!(code, "\tmov     rdi, rax")?;
            writeln!(code, "\tmov     rax, {}", SyscallTable::Exit.code(arch))?;
            writeln!(code, "\tsyscall")?;
            writeln!(code, "\tret")?;
        }

        if !self.globals.is_empty() {
            writeln!(res, "global {}\n", self.globals.join(","))?;
        }

        if !self.externs.is_empty() {
            writeln!(res, "\textern {}", self.externs.join(","))?;
        }

        write!(res, "{}\n", code)?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn compiler_register_distribution() {
        let mut c = Compiler::new(TargetObjectType::X86_64);

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
