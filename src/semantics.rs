use std::collections::HashMap;
use std::io::Read;
use std::ops::IndexMut;

use crate::error;
use crate::nodes::{Node, NodeType};
use crate::nodes::accessor_node::AccessorNode;
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::break_node::BreakNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::cast_node::CastNode;
use crate::nodes::char_node::CharNode;
use crate::nodes::const_def_node::ConstDefinitionNode;
use crate::nodes::continue_node::ContinueNode;
use crate::nodes::dereference_node::DereferenceNode;
use crate::nodes::for_node::ForNode;
use crate::nodes::functiondecl_node::FunctionDeclarationNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::if_node::case::IfCase;
use crate::nodes::if_node::elsecase::ElseCase;
use crate::nodes::if_node::IfNode;
use crate::nodes::ignored_node::IgnoredNode;
use crate::nodes::import_node::ImportNode;
use crate::nodes::list_node::ListNode;
use crate::nodes::macro_def_node::MacroDefNode;
use crate::nodes::NodeType::{BinOp, Syscall, VarTypedAssign};
use crate::nodes::number_node::NumberNode;
use crate::nodes::offset_node::OffsetNode;
use crate::nodes::pointer_assign_node::PointerAssignNode;
use crate::nodes::read_bytes_node::ReadBytesNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::sizeof_node::SizeOfNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::static_def_node::StaticDefinitionNode;
use crate::nodes::string_node::StringNode;
use crate::nodes::struct_def_node::StructDefinitionNode;
use crate::nodes::syscall_node::SyscallNode;
use crate::nodes::unaryop_node::UnaryOpNode;
use crate::nodes::var_node::access::VarAccessNode;
use crate::nodes::var_node::assign::VarAssignNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::nodes::var_node::typed_access::VarTypedAccessNode;
use crate::nodes::var_node::typed_assign::VarTypedAssignNode;
use crate::nodes::while_node::WhileNode;
use crate::position::Position;
use crate::results::validation::ValidationResult;
use crate::symbol_table::Symbol;
use crate::token::{OldToken, TokenType};
use crate::token::TokenType::U64;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypes};
use crate::values::value_type::bool_type::BoolType;
use crate::values::value_type::char_type::CharType;
use crate::values::value_type::extern_type::ExternType;
use crate::values::value_type::function_type::FunctionType;
use crate::values::value_type::ignored_type::IgnoredType;
use crate::values::value_type::u64_type::U64Type;
use crate::values::value_type::pointer_type::PointerType;
use crate::values::value_type::string_type::StringType;
use crate::values::value_type::struct_type::StructType;
use crate::values::value_type::ValueTypes::Void;
use crate::values::value_type::void_type::VoidType;

#[derive(PartialEq, Debug)]
enum ScopeType {
    Global,
    Function,
    Loop,
    Block,
}

pub struct Validator {
    type_stack: Vec<HashMap<String, (Symbol, Position)>>,
    scope_stack: Vec<ScopeType>,

    current_function_return_type: Option<Box<dyn ValueType>>,

    structs: HashMap<String, Vec<(String, Box<dyn ValueType>)>>
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            type_stack: vec![
                HashMap::new(),
            ],
            scope_stack: vec![
                ScopeType::Global,
            ],
            current_function_return_type: None,
            structs: HashMap::new(),
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
            return s.unwrap().0.is_mutable();
        }

        false
    }

    fn get_symbol(&self, name: &str) -> Option<&(Symbol, Position)> {
        for s in self.type_stack.iter().rev() {
            if s.contains_key(name) {
                return s.get(name);
            }
        }

        None
    }

    fn declare_symbol(&mut self, name: String, sym: Symbol, pos: Position) {
        if self.type_stack.is_empty() {
            return;
        }

        if self.has_symbol(&name) {
            return;
        }

        (self.type_stack.index_mut(self.type_stack.len() - 1)).insert(name, (sym, pos));
    }

    fn push_child_scope(&mut self, scope_type: ScopeType) {
        self.type_stack.push(HashMap::new());
        self.scope_stack.push(scope_type);
    }

    fn pop_child_scope(&mut self) {
        if self.type_stack.len() == 1 || self.scope_stack.len() == 1 {
            panic!("Can't pop root tables!");
        }

        self.type_stack.pop();
        self.scope_stack.pop();
    }

    fn is_in_scope_stack(&self, scope_type: ScopeType) -> bool {
        for s in self.scope_stack.iter().rev() {
            if *s == scope_type {
                return true;
            }
        }
        false
    }

    fn find_first_function(&self) -> Option<Box<dyn ValueType>> {
        if self.is_in_scope_stack(ScopeType::Function) {
            return self.current_function_return_type.clone();
        }

        None
    }
}

impl Validator {

    pub fn validate(&mut self, node: &Box<dyn Node>) -> ValidationResult {
        match node.node_type() {
            NodeType::Statements => self.validate_statements_node(node.as_any().downcast_ref::<StatementsNode>().unwrap()),
            NodeType::Number => self.validate_number_node(node.as_any().downcast_ref::<NumberNode>().unwrap()),
            NodeType::String => self.validate_string_node(node.as_any().downcast_ref::<StringNode>().unwrap()),
            NodeType::Char => self.validate_char_node(node.as_any().downcast_ref::<CharNode>().unwrap()),
            NodeType::List => self.validate_list_node(node.as_any().downcast_ref::<ListNode>().unwrap()),
            NodeType::BinOp => self.validate_bin_op_node(node.as_any().downcast_ref::<BinOpNode>().unwrap()),
            NodeType::UnaryOp => self.validate_unary_op_node(node.as_any().downcast_ref::<UnaryOpNode>().unwrap()),
            NodeType::VarDeclaration => self.validate_var_declaration_node(node.as_any().downcast_ref::<VarDeclarationNode>().unwrap()),
            NodeType::VarAssign => self.validate_var_assign_node(node.as_any().downcast_ref::<VarAssignNode>().unwrap()),
            NodeType::VarAccess => self.validate_var_access_node(node.as_any().downcast_ref::<VarAccessNode>().unwrap()),
            NodeType::FunctionDef => self.validate_function_def_node(node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap()),
            NodeType::FunctionDecl => self.validate_function_decl_node(node.as_any().downcast_ref::<FunctionDeclarationNode>().unwrap()),
            NodeType::Call => self.validate_call_node(node.as_any().downcast_ref::<CallNode>().unwrap()),
            NodeType::Return => self.validate_return_node(node.as_any().downcast_ref::<ReturnNode>().unwrap()),
            NodeType::Break => self.validate_break_node(node.as_any().downcast_ref::<BreakNode>().unwrap()),
            NodeType::Continue => self.validate_continue_node(node.as_any().downcast_ref::<ContinueNode>().unwrap()),
            NodeType::Syscall => self.validate_syscall_node(node.as_any().downcast_ref::<SyscallNode>().unwrap()),
            NodeType::While => self.validate_while_node(node.as_any().downcast_ref::<WhileNode>().unwrap()),
            NodeType::For => self.validate_for_node(node.as_any().downcast_ref::<ForNode>().unwrap()),
            NodeType::If => self.validate_if_node(node.as_any().downcast_ref::<IfNode>().unwrap()),
            NodeType::Cast => self.validate_cast_node(node.as_any().downcast_ref::<CastNode>().unwrap()),
            NodeType::ConstDef => self.validate_const_def_node(node.as_any().downcast_ref::<ConstDefinitionNode>().unwrap()),
            NodeType::SizeOf => self.validate_sizeof_node(node.as_any().downcast_ref::<SizeOfNode>().unwrap()),
            NodeType::StaticDef => self.validate_static_def_node(node.as_any().downcast_ref::<StaticDefinitionNode>().unwrap()),
            NodeType::StructDef => self.validate_struct_def_node(node.as_any().downcast_ref::<StructDefinitionNode>().unwrap()),
            NodeType::ReadBytes => self.validate_read_bytes_node(node.as_any().downcast_ref::<ReadBytesNode>().unwrap()),
            NodeType::Dereference => self.validate_dereference_node(node.as_any().downcast_ref::<DereferenceNode>().unwrap()),
            NodeType::Import => self.validate_import_node(node.as_any().downcast_ref::<ImportNode>().unwrap()),
            NodeType::MacroDef => self.validate_macro_def_node(node.as_any().downcast_ref::<MacroDefNode>().unwrap()),
            NodeType::Ignored => self.validate_ignored_node(node.as_any().downcast_ref::<IgnoredNode>().unwrap()),
            NodeType::Accessor => self.validate_accessor_node(node.as_any().downcast_ref::<AccessorNode>().unwrap()),
            _ => panic!("Unsupported node type: {:?}", node.node_type()),
        }
    }

    fn validate_statements_node(&mut self, node: &StatementsNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        self.push_child_scope(ScopeType::Block);

        let mut stmts: Vec<Box<dyn Node>> = vec![];
        for s in node.statement_nodes() {
            let (_, stmt) = res.register_res(self.validate(s));
            if res.has_error() {
                return res;
            }
            stmts.push(stmt.unwrap());
        }

        self.pop_child_scope();

        res.success(Box::new(IgnoredType::new()), Box::new(StatementsNode::new(stmts, node.pos_start().clone(), node.pos_end().clone())));
        res
    }

    fn validate_number_node(&self, node: &NumberNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(U64Type::new()), node.box_clone());
        res
    }

    fn validate_string_node(&self, node: &StringNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(StringType::new()), node.box_clone());
        res
    }

    fn validate_char_node(&self, node: &CharNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(CharType::new()), node.box_clone());
        res
    }

    fn validate_list_node(&mut self, node: &ListNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        for el in node.element_nodes() {
            let (t, elem_node) = res.register_res(self.validate(el));

            if res.has_error() {
                return res;
            }

            if !t.as_ref().unwrap().eq(node.element_type()) {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Type {}, is incompatible with list type {}!", t.as_ref().unwrap(), node.element_type()).as_str()));
                return res;
            }
        }

        res.success(Box::new(PointerType::new(node.element_type().clone(), true)), node.box_clone());
        res
    }

    fn validate_bin_op_node(&mut self, node: &BinOpNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (left, left_node) = res.register_res(self.validate(node.left_node()));
        if res.has_error() {
            return res;
        }

        let (right, right_node) = res.register_res(self.validate(node.right_node()));
        if res.has_error() {
            return res;
        }

        let result_type = left.as_ref().unwrap().is_valid_bin_op(node.op_token(), right.as_ref().unwrap());
        if result_type.is_none() {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Binary operation '{}' not allowed between value_type {} and {}!", node.op_token(), left.as_ref().unwrap(), right.as_ref().unwrap()).as_str()));
            return res;
        }

        if node.op_token().token_type() == TokenType::PointerAssign {
            res.success(result_type.unwrap(), Box::new(PointerAssignNode::new(left_node.unwrap(), left.unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().box_clone(), right_node.unwrap())));
            return res;
        }
        if node.op_token().token_type() == TokenType::Offset {
            res.success(result_type.unwrap(), Box::new(OffsetNode::new(left_node.unwrap(), right_node.unwrap(), left.unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().box_clone())));
            return res;
        }

        res.success(result_type.unwrap(), Box::new(BinOpNode::new(left_node.unwrap(), node.op_token().clone(), right_node.unwrap())));
        res
    }

    fn validate_unary_op_node(&mut self, node: &UnaryOpNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (right, right_node) = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        let result_type = right.as_ref().unwrap().is_valid_unary_op(node.op_token());
        if result_type.is_none() {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Unary operation '{}' not allowed on type {}!", node.op_token(), right.as_ref().unwrap()).as_str()));
            return res;
        }

        res.success(result_type.unwrap(), Box::new(UnaryOpNode::new(node.op_token().clone(), right_node.unwrap())));
        res
    }

    fn validate_var_declaration_node(&mut self, node: &VarDeclarationNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.has_symbol(node.var_name()) {
            let (_, decl_pos) = self.get_symbol(node.var_name()).unwrap();
            res.failure(error::semantic_error_with_parent(
                node.pos_start().clone(), node.pos_end().clone(), format!("Variable '{}' was already declared in this scope!", node.var_name()).as_str(),
                error::semantic_error(decl_pos.clone(), decl_pos.clone(), format!("Previous declaration of '{}'", node.var_name()).as_str())
            ));
            return res;
        }

        let (t, value_node) = res.register_res(self.validate(node.value_node()));
        if res.has_error() {
            return res;
        }

        let symbol_type = t.unwrap();

        if !symbol_type.eq(node.var_type()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Type '{}' can't be assigned to type '{}'!", &symbol_type, node.var_type()).as_str()));
            return res;
        }

        self.declare_symbol(node.var_name().to_string(), Symbol::new(symbol_type.clone(), node.is_mutable()), node.pos_start().clone());
        res.success(symbol_type, Box::new(VarDeclarationNode::new(node.var_name().to_string(), node.var_type().box_clone(), value_node.unwrap(), node.is_mutable(), node.pos_start().clone())));
        res
    }

    fn validate_var_assign_node(&mut self, node: &VarAssignNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Variable '{}' was not declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        let (assign_type, assign_node) = res.register_res(self.validate(node.value_node()));
        if res.has_error() {
            return res;
        }

        if !self.get_symbol(node.var_name()).unwrap().0.value_type().eq(assign_type.as_ref().unwrap()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Variable type {} does not match assign type {}!", self.get_symbol(node.var_name()).unwrap().0.value_type(), assign_type.as_ref().unwrap()).as_str()));
            return res;
        }

        if !self.is_symbol_mut(node.var_name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Variable '{}' is not mutable!", node.var_name()).as_str()));
            return res;
        }

        res.success(assign_type.as_ref().unwrap().clone(), Box::new(VarTypedAssignNode::new(node.var_name().to_string(), assign_node.unwrap(), assign_type.unwrap(), node.pos_start().clone())));
        // res.success(symbol_type, Box::new(VarAssignNode::new(node.var_name().to_string(), assign_node.unwrap(), node.pos_start().clone())));
        res
    }

    fn validate_var_access_node(&mut self, node: &VarAccessNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Variable '{}' wasn't declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        let base_type = self.get_symbol(node.var_name()).unwrap().0.value_type().clone();

        res.success(base_type.clone(), Box::new(VarTypedAccessNode::new(node.var_name().to_string(), base_type, node.pos_start().clone(), node.pos_end().clone())));
        // res.success(base_type, node.box_clone());
        res
    }

    fn validate_function_def_node(&mut self, node: &FunctionDefinitionNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Function or variable with name '{}' was already declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        if node.var_name() == "main" {
            if node.return_type().value_type() != ValueTypes::U64 {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Main function must return type 'number'!").as_str()));
                return res;
            }

            if node.args().len() != 2 {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Main function must have two parameters!").as_str()));
                return res;
            }

            if node.args()[0].1.value_type() != ValueTypes::U64 {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("First parameter of main function must be of type 'number'!").as_str()));
                return res;
            }

            if node.args()[1].1.value_type() != ValueTypes::Pointer || node.args()[1].1.as_any().downcast_ref::<PointerType>().unwrap().pointee_type().value_type() != ValueTypes::Char {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Second parameter of main function must be a pointer to a character!").as_str()));
                return res;
            }

        }

        let mut arg_types: Vec<Box<dyn ValueType>> = vec![];
        arg_types.reserve(node.args().len());

        for (_, value_type) in node.args() {
            arg_types.push(value_type.clone());
        }

        self.declare_symbol(
            node.var_name().to_string(),
            Symbol::new(Box::new(FunctionType::new(arg_types.clone(), node.return_type().clone())), false),
            node.pos_start().clone(),
        );

        self.push_child_scope(ScopeType::Function);
        let old_return_type = self.current_function_return_type.clone();
        self.current_function_return_type = Some(node.return_type().clone());

        for (name, value_type) in node.args() {
            self.declare_symbol(name.clone(), Symbol::new(value_type.clone(), true), node.pos_start().clone());
        }

        let (_, body_node) = res.register_res(self.validate(node.body_node()));

        self.pop_child_scope();
        self.current_function_return_type = old_return_type;

        if res.has_error() {
            res.failure(error::semantic_error_with_parent(node.pos_start().clone(), node.pos_end().clone(), "Function definition failed!", res.error().as_ref().unwrap().clone()));
            return res;
        }

        if !res.has_return_type() {
            res.failure(error::semantic_error(node.pos_end().clone(), node.pos_end().clone(), "No return statement given!"));
            return res;
        }

        if !res.return_type().as_ref().unwrap().eq(node.return_type()) {
            res.failure(error::semantic_error(node.pos_end().clone(), node.pos_end().clone(), format!("Function return type is '{}', returned was '{}'!", node.return_type(), res.return_type().as_ref().unwrap()).as_str()));
            return res;
        }

        res.success(Box::new(IgnoredType::new()), Box::new(FunctionDefinitionNode::new(node.var_name().to_string(), node.args().clone(), node.return_type().box_clone(), body_node.unwrap(), node.pos_start().clone())));
        res
    }

    fn validate_function_decl_node(&mut self, node: &FunctionDeclarationNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.has_symbol(node.var_name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Function or variable with name '{}' was already declared in this scope!", node.var_name()).as_str()));
            return res;
        }

        let mut arg_types: Vec<Box<dyn ValueType>> = vec![];
        arg_types.reserve(node.args().len());

        for (_, value_type) in node.args() {
            arg_types.push(value_type.clone());
        }

        self.declare_symbol(
            node.var_name().to_string(),
            Symbol::new(Box::new(FunctionType::new(arg_types.clone(), node.return_type().clone())), false),
            node.pos_start().clone()
        );

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_call_node(&mut self, node: &CallNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.has_symbol(node.func_to_call()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("'{}' was not declared in this scope!", node.func_to_call()).as_str()));
            return res;
        }

        if self.get_symbol(node.func_to_call()).unwrap().0.value_type().value_type() != ValueTypes::Function {
            if self.get_symbol(node.func_to_call()).unwrap().0.value_type().value_type() == ValueTypes::Extern {
                res.success(Box::new(U64Type::new()), node.box_clone());
                return res;
            }

            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("'{}' is not a function!", node.func_to_call()).as_str()));
            return res;
        }

        let symbol_type = self.get_symbol(node.func_to_call()).unwrap().0.value_type().clone();
        let function_type = symbol_type.as_any().downcast_ref::<FunctionType>().unwrap();

        if function_type.arg_types().len() != node.arg_nodes().len() {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Function expected {} arguments. {} were passed!", function_type.arg_types().len(), node.arg_nodes().len()).as_str()));
            return res;
        }

        let mut arg_nodes: Vec<Box<dyn Node>> = vec![];

        for (i, arg) in node.arg_nodes().iter().enumerate() {
            let (t, arg_node) = res.register_res(self.validate(arg));
            if res.has_error() {
                return res;
            }

            if !t.as_ref().unwrap().eq(&function_type.arg_types()[i]) {
                res.failure(error::semantic_error(arg.pos_start().clone(), arg.pos_end().clone(), format!("Expected type '{}' as argument at index {}, got '{}'!", function_type.arg_types()[i], i, t.as_ref().unwrap()).as_str()));
                return res;
            }

            arg_nodes.push(arg_node.unwrap());
        }

        res.success(function_type.return_type().clone(), Box::new(CallNode::new(node.func_to_call().to_string(), arg_nodes, node.pos_start().clone())));
        res
    }

    fn validate_return_node(&mut self, node: &ReturnNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let function_return_type = self.find_first_function();
        if function_return_type.is_none() {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Return statement outside of function!"));
            return res;
        }

        if node.node_to_return().is_none() {
            if function_return_type.as_ref().unwrap().value_type() != ValueTypes::Void {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Function return type is not void!"));
                return res;
            }

            res.success_return(Box::new(VoidType::new()));
            res.success(Box::new(IgnoredType::new()), Box::new(ReturnNode::new(None, node.pos_start().clone(), node.pos_end().clone())));
            return res;
        }

        let (return_type, return_node) = res.register_res(self.validate(node.node_to_return().as_ref().unwrap()));
        if res.has_error() {
            return res;
        }

        if !return_type.as_ref().unwrap().eq(function_return_type.as_ref().unwrap()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Expected return type '{}', got '{}'!", function_return_type.as_ref().unwrap(), return_type.as_ref().unwrap()).as_str()));
            return res;
        }

        res.success_return(return_type.unwrap());
        res.success(Box::new(IgnoredType::new()), Box::new(ReturnNode::new(Some(return_node.unwrap()), node.pos_start().clone(), node.pos_end().clone())));
        res
    }

    fn validate_break_node(&mut self, node: &BreakNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.is_in_scope_stack(ScopeType::Loop) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Break statement outside of loop!"));
            return res;
        }

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_continue_node(&mut self, node: &ContinueNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if !self.is_in_scope_stack(ScopeType::Loop) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Continue statement outside of loop!"));
            return res;
        }

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_syscall_node(&mut self, node: &SyscallNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let mut arg_nodes: Vec<Box<dyn Node>> = vec![];
        for arg in node.args().iter() {
            let (_, arg_node) = res.register_res(self.validate(arg));
            if res.has_error() {
                return res;
            }

            arg_nodes.push(arg_node.unwrap());
        }

        res.success(Box::new(U64Type::new()), Box::new(SyscallNode::new(arg_nodes, node.pos_start().clone(), node.pos_end().clone())));
        res
    }

    fn validate_while_node(&mut self, node: &WhileNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (condition_type, condition_node) = res.register_res(self.validate(node.condition_node()));
        if res.has_error() {
            return res;
        }

        if condition_type.as_ref().unwrap().value_type() != ValueTypes::Bool {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Condition must be of type bool!"));
            return res;
        }

        self.push_child_scope(ScopeType::Loop);
        let (_, body_node) = res.register_res(self.validate(node.body_node()));
        self.pop_child_scope();

        if res.has_error() {
            return res;
        }

        res.success(Box::new(IgnoredType::new()), Box::new(WhileNode::new(condition_node.unwrap(), body_node.unwrap())));
        res
    }

    fn validate_for_node(&mut self, node: &ForNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        self.push_child_scope(ScopeType::Block);

        let (_, init_stmt) = res.register_res(self.validate(node.init_stmt()));
        if res.has_error() {
            self.pop_child_scope();
            return res;
        }

        let (condition_type, condition_node) = res.register_res(self.validate(node.condition()));
        if res.has_error() {
            self.pop_child_scope();
            return res;
        }

        if condition_type.unwrap().value_type() != ValueTypes::Bool {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Condition must be of type bool!"));
            self.pop_child_scope();
            return res;
        }

        let (_, next_expr) = res.register_res(self.validate(node.next_expr()));
        if res.has_error() {
            self.pop_child_scope();
            return res;
        }

        self.push_child_scope(ScopeType::Loop);

        let (_, body) = res.register_res(self.validate(node.body()));

        self.pop_child_scope();
        self.pop_child_scope();

        if res.has_error() {
            return res;
        }

        res.success(Box::new(IgnoredType::new()), Box::new(ForNode::new(init_stmt.unwrap(), condition_node.unwrap(), next_expr.unwrap(), body.unwrap())));
        res
    }

    fn validate_if_node(&mut self, node: &IfNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let mut cases: Vec<IfCase> = vec![];
        for case in node.cases() {
            let (condition_type, condition_node) = res.register_res(self.validate(case.condition()));
            if res.has_error() {
                return res;
            }

            if condition_type.unwrap().value_type() != ValueTypes::Bool {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Condition must be of type bool!"));
                return res;
            }

            self.push_child_scope(ScopeType::Block);
            let (_, statements) = res.register_res(self.validate(case.statements()));
            self.pop_child_scope();

            if res.has_error() {
                return res;
            }

            cases.push(IfCase::new(condition_node.unwrap(), statements.unwrap()));
        }

        let mut else_case: Option<ElseCase> = None;
        if node.else_case().is_some() {
            self.push_child_scope(ScopeType::Block);
            let (_, statements) = res.register_res(self.validate(node.else_case().as_ref().unwrap().statements()));
            self.pop_child_scope();

            if res.has_error() {
                return res;
            }

            else_case = Some(ElseCase::new(statements.unwrap()));
        }

        // TODO: check if all code paths return a value

        res.success(Box::new(IgnoredType::new()), Box::new(IfNode::new(cases, else_case)));
        res
    }

    fn validate_cast_node(&mut self, node: &CastNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (node_type, cast_node) = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        if !node_type.as_ref().unwrap().is_valid_cast(node.cast_type()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Invalid cast! Cannot cast from type '{}' to type '{}'!", node_type.as_ref().unwrap(), node.cast_type()).as_str()));
            return res;
        }

        res.success(node.cast_type().clone(), Box::new(CastNode::new(cast_node.unwrap(), node.cast_type().clone(), node.pos_end().clone())));
        res
    }

    fn validate_const_def_node(&mut self, node: &ConstDefinitionNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (assign_type, assign_node) = res.register_res(self.validate(node.value()));
        if res.has_error() {
            return res;
        }

        if !assign_type.as_ref().unwrap().eq(node.value_type()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Type '{}' can't be assigned to type '{}'!", assign_type.as_ref().unwrap(), node.value_type()).as_str()));
            return res;
        }


        self.declare_symbol(node.name().to_string(), Symbol::new(assign_type.unwrap(), false), node.pos_start().clone());

        res.success(Box::new(IgnoredType::new()), Box::new(ConstDefinitionNode::new(node.name().to_string(), assign_node.unwrap(), node.value_type().clone(), node.pos_start().clone())));
        res
    }

    fn validate_sizeof_node(&self, node: &SizeOfNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let mut size = node.value_type().get_size().get_size_in_bytes();
        if node.value_type().value_type() == ValueTypes::Struct {
            let struct_type = node.value_type().as_any().downcast_ref::<StructType>().unwrap();
            if !self.structs.contains_key(struct_type.name()) {
                res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Struct '{}' is not defined!", struct_type.name()).as_str()));
                return res;
            }

            size = 0;
            for (_, field_type) in self.structs[struct_type.name()].iter() {
                size += field_type.get_size().get_size_in_bytes();
            }
        }

        res.success(Box::new(U64Type::new()), Box::new(NumberNode::new(OldToken::new_with_value(TokenType::U64, size.to_string(), node.pos_start().clone(), node.pos_end().clone()), Box::new(U64Type::new()))));
        res
    }

    fn validate_static_def_node(&mut self, node: &StaticDefinitionNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (assign_type, assign_node) = res.register_res(self.validate(node.value()));
        if res.has_error() {
            return res;
        }

        if !assign_type.as_ref().unwrap().eq(node.value_type()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Type '{}' can't be assigned to type '{}'!", assign_type.as_ref().unwrap(), node.value_type()).as_str()));
            return res;
        }

        self.declare_symbol(node.name().to_string(), Symbol::new(assign_type.as_ref().unwrap().clone(), *node.is_mutable()), node.pos_start().clone());

        res.success(assign_type.unwrap(), Box::new(StaticDefinitionNode::new(node.name().to_string(), node.value_type().clone(), assign_node.unwrap(), *node.is_mutable(), node.pos_start().clone())));
        res
    }

    fn validate_struct_def_node(&mut self, node: &StructDefinitionNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        if self.structs.contains_key(node.name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Structure with the name '{}' was already defined!", node.name()).as_str()));
            return res;
        }

        self.structs.insert(node.name().to_string(), node.fields().clone());

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_read_bytes_node(&mut self, node: &ReadBytesNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (node_type, value_node) = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        if node_type.as_ref().unwrap().value_type() != ValueTypes::Pointer {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Can't read bytes from non-pointer type!"));
            return res;
        }

        res.success(
            node_type.unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().clone(),
            Box::new(ReadBytesNode::new(value_node.unwrap(), *node.bytes(), node.pos_end().clone())));
        res
    }

    fn validate_dereference_node(&mut self, node: &DereferenceNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (node_type, value_node) = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        if node_type.as_ref().unwrap().value_type() != ValueTypes::Pointer {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Can't dereference non-pointer type!"));
            return res;
        }

        let pointee_type_size = node_type.as_ref().unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().get_size();

        res.success(node_type.as_ref().unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().clone(), Box::new(ReadBytesNode::new(value_node.unwrap(), pointee_type_size, node.pos_end().clone())));
        res
    }

    fn validate_import_node(&mut self, node: &ImportNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let stmt_node = node.node().as_any().downcast_ref::<StatementsNode>().unwrap();

        let mut stmts: Vec<Box<dyn Node>> = vec![];
        for stmt in stmt_node.statement_nodes() {
            let (_, s) = res.register_res(self.validate(stmt));
            if res.has_error() {
                return res;
            }
            stmts.push(s.unwrap());
        }

        res.success(Box::new(IgnoredType::new()), Box::new(ImportNode::new(Box::new(StatementsNode::new(stmts, node.node().pos_start().clone(), node.node().pos_end().clone())))));
        res
    }

    fn validate_macro_def_node(&mut self, node: &MacroDefNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_ignored_node(&mut self, node: &IgnoredNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        res.success(Box::new(IgnoredType::new()), node.box_clone());
        res
    }

    fn validate_accessor_node(&mut self, node: &AccessorNode) -> ValidationResult {
        let mut res = ValidationResult::new();

        let (node_type, value_node) = res.register_res(self.validate(node.node()));
        if res.has_error() {
            return res;
        }

        if node_type.as_ref().unwrap().value_type() != ValueTypes::Pointer || node_type.as_ref().unwrap().as_any().downcast_ref::<PointerType>().unwrap().pointee_type().value_type() != ValueTypes::Struct {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), "Can't access fields of non-pointer-to-struct type!"));
            return res;
        }

        let pointer_type = node_type.as_ref().unwrap().as_any().downcast_ref::<PointerType>().unwrap();
        let struct_type = pointer_type.pointee_type().as_any().downcast_ref::<StructType>().unwrap();

        if !self.structs.contains_key(struct_type.name()) {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Structure with the name '{}' was not defined!", struct_type.name()).as_str()));
            return res;
        }

        let s_struct = &self.structs[struct_type.name()];

        let mut offset: usize = 0;
        let mut f_field_type: Option<&Box<dyn ValueType>> = None;
        let mut found: bool = false;
        for (field_name, field_type) in s_struct {
            if field_name == node.accessor() {
                found = true;
                f_field_type = Some(field_type);
                break;
            }

            offset += field_type.get_size().get_size_in_bytes() as usize;
        }

        if !found {
            res.failure(error::semantic_error(node.pos_start().clone(), node.pos_end().clone(), format!("Field '{}' was not found in structure '{}'!", node.accessor(), struct_type.name()).as_str()));
            return res;
        }

        res.success(Box::new(PointerType::new(f_field_type.unwrap().clone(), *pointer_type.is_mutable())), Box::new(BinOpNode::new(
            value_node.unwrap(),
            OldToken::new_without_value(TokenType::Plus, Position::empty(), Position::empty()),
            Box::new(NumberNode::new(OldToken::new_with_value(TokenType::U64, offset.to_string(), Position::empty(), Position::empty()), Box::new(U64Type::new()))))));
        res
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn semantics_symbol_stack() {
        let mut v = Validator::new();

        v.declare_symbol("a".to_string(), Symbol::new(Box::new(VoidType::new()), false), Position::empty());

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.get_symbol("a").unwrap().0.value_type().value_type(), ValueTypes::Void);
        assert_eq!(v.is_symbol_mut("a"), false);

        v.push_child_scope(ScopeType::Block);

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.has_symbol("b"), false);
        v.declare_symbol("b".to_string(), Symbol::new(Box::new(VoidType::new()), false), Position::empty());
        assert_eq!(v.has_symbol("b"), true);

        v.pop_child_scope();

        assert_eq!(v.has_symbol("a"), true);
        assert_eq!(v.has_symbol("b"), false);
    }
}