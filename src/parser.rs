use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use same_file::is_same_file;

use crate::error;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::nodes::{Node, NodeType};
use crate::nodes::accessor_node::AccessorNode;
use crate::nodes::asm_node::AssemblyNode;
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
use crate::nodes::number_node::NumberNode;
use crate::nodes::read_bytes_node::ReadBytesNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::sizeof_node::SizeOfNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::static_def_node::StaticDefinitionNode;
use crate::nodes::string_node::StringNode;
use crate::nodes::struct_def_node::StructDefinitionNode;
use crate::nodes::syscall_node::SyscallNode;
use crate::nodes::unaryop_node::UnaryOpNode;
use crate::nodes::util::type_carrier_node::TypeCarrierNode;
use crate::nodes::var_node::access::VarAccessNode;
use crate::nodes::var_node::assign::VarAssignNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::nodes::while_node::WhileNode;
use crate::results::parse::ParseResult;
use crate::token::{Token, TOKEN_FLAGS_IS_ASSIGN, TokenType};
use crate::values::value_size::ValueSize;
use crate::values::value_type::bool_type::BoolType;
use crate::values::value_type::char_type::CharType;
use crate::values::value_type::u64_type::U64Type;
use crate::values::value_type::pointer_type::PointerType;
use crate::values::value_type::string_type::StringType;
use crate::values::value_type::struct_type::StructType;
use crate::values::value_type::u16_type::U16Type;
use crate::values::value_type::u32_type::U32Type;
use crate::values::value_type::u8_type::U8Type;
use crate::values::value_type::ValueType;
use crate::values::value_type::void_type::VoidType;

macro_rules! advance {
    ($self:ident, $res:ident) => {
        $self.advance();
        $res.register_advancement();
    };
}

macro_rules! expect_token {
    ($self:ident, $res:ident, $token_type:expr, $repr:literal) => {
        if $self.current_token().token_type() != $token_type {
            $res.failure(error::invalid_syntax_error($self.current_token().pos_start().clone(), $self.current_token().pos_end().clone(), format!("Expected token '{}' of type '{:?}'!", $repr, $token_type).as_str()));
            return $res;
        }
    };
}

macro_rules! expect_keyword {
    ($self:ident, $res:ident, $keyword:literal) => {
        if !$self.current_token().matches_keyword($keyword) {
            $res.failure(error::invalid_syntax_error($self.current_token().pos_start().clone(), $self.current_token().pos_end().clone(), format!("Expected keyword '{}'!", $keyword).as_str()));
            return $res;
        }
    }
}

macro_rules! expect_token_value {
    ($self:ident, $res:ident) => {
        if $self.current_token().token_value().is_none() {
            $res.failure(error::invalid_syntax_error($self.current_token().pos_start().clone(), $self.current_token().pos_end().clone(), "Expected token to have value!"));
            return $res;
        }
    };
}


#[derive(Copy, Clone)]
enum BinOpFunction {
    Arith = 0,
    Comp = 1,
    Term = 2,
    Factor = 3,
    Call = 4,
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    token_index: usize,
    macros: &'a mut HashMap<String, Box<dyn Node>>,
    already_included: &'a mut Vec<PathBuf>,
    include_paths: &'a Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, include_paths: &'a Vec<String>, macros: &'a mut HashMap<String, Box<dyn Node>>, already_included: &'a mut Vec<PathBuf>) -> Self {
        if tokens.is_empty() {
            panic!("No tokens were provided!");
        }

        Parser {
            tokens,
            token_index: 0,
            macros,
            already_included,
            include_paths
        }
    }

    pub fn macros(&self) -> &&mut HashMap<String, Box<dyn Node>> {
        &self.macros
    }

    pub fn already_included(&self) -> &&mut Vec<PathBuf> {
        &self.already_included
    }

    fn current_token(&self) -> &Token { return &self.tokens[self.token_index]; }

    fn advance(&mut self) -> () {
        self.token_index += 1;
    }

    fn reverse(&mut self, amount: usize) -> () {
        self.token_index -= amount;
    }

    pub fn parse(&mut self) -> Result<Box<dyn Node>, Error> {
        let mut res = ParseResult::new();

        let stmts = res.register_res(self.statements(true));
        if res.has_error() {
            return Err(res.error().as_ref().unwrap().clone());
        }

        if self.current_token().token_type() != TokenType::Eof {
            return Err(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected end of file!"));
        }

        return Ok(stmts.unwrap());
    }

    // region Helper functions

    fn parse_intrinsic_type(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        expect_token!(self, res, TokenType::Keyword, "intrinsic type");

        let s = self.current_token().token_value().as_ref().unwrap().as_str();

        let base_type: Box<dyn ValueType> = match s {
            "u64" => Box::new(U64Type::new()),
            "u32" => Box::new(U32Type::new()),
            "u16" => Box::new(U16Type::new()),
            "u8" => Box::new(U8Type::new()),
            "string" => Box::new(StringType::new()),
            "bool" => Box::new(BoolType::new()),
            "char" => Box::new(CharType::new()),
            "void" => Box::new(VoidType::new()),
            "struct" => {
                advance!(self, res);

                expect_token!(self, res, TokenType::Identifier, "struct name");

                let struct_name = self.current_token().token_value().as_ref().unwrap().clone();

                Box::new(StructType::new(struct_name))
            }
            _ => {
                res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected intrinsic type!"));
                return res;
            }
        };

        advance!(self, res);

        let mut final_type = base_type;
        while self.current_token().token_type() == TokenType::Mul {
            advance!(self, res);

            if self.current_token().matches_keyword("mut") {
                advance!(self, res);
                final_type = Box::new(PointerType::new(final_type, true));
            } else {
                final_type = Box::new(PointerType::new(final_type, false));
            }
        }

        res.success(Box::new(TypeCarrierNode::new(final_type)));
        return res;
    }

    // endregion

    // region Parsing functions

    fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        expect_keyword!(self, res, "if");

        let mut cases: Vec<IfCase> = vec![];
        let mut else_case: Option<ElseCase> = None;

        while self.current_token().matches_keyword("if") {
            advance!(self, res);

            let condition = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            let statements = res.register_res(self.statements(false));
            if res.has_error() {
                return res;
            }

            cases.push(IfCase::new(condition.unwrap(), statements.unwrap()));

            if self.current_token().matches_keyword("else") {
                advance!(self, res);

                if self.current_token().matches_keyword("if") {
                    continue;
                }

                let statements = res.register_res(self.statements(false));
                if res.has_error() {
                    return res;
                }

                else_case = Some(ElseCase::new(statements.unwrap()));
            }
        }

        res.success(Box::new(IfNode::new(cases, else_case)));
        res
    }

    fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        expect_keyword!(self, res, "for");

        advance!(self, res);

        let init_expr = res.register_res(self.statement(false));
        if res.has_error() {
            return res;
        }

        expect_token!(self, res, TokenType::Newline, ";");

        advance!(self, res);

        let condition = res.register_res(self.expression());
        if res.has_error() {
            return res;
        }

        expect_token!(self, res, TokenType::Newline, ";");

        advance!(self, res);

        let next_expr = res.register_res(self.expression());
        if res.has_error() {
            return res;
        }

        let statements = res.register_res(self.statements(false));
        if res.has_error() {
            return res;
        }

        res.success(Box::new(ForNode::new(init_expr.unwrap(), condition.unwrap(), next_expr.unwrap(), statements.unwrap())));
        res
    }

    fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        expect_keyword!(self, res, "while");

        advance!(self, res);

        let condition = res.register_res(self.expression());
        if res.has_error() {
            return res;
        }

        let stmts = res.register_res(self.statements(false));
        if res.has_error() {
            return res;
        }

        res.success(Box::new(WhileNode::new(condition.unwrap(), stmts.unwrap())));
        res
    }

    fn list_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token().pos_start().clone();

        expect_token!(self, res, TokenType::Lsquare, "[");

        advance!(self, res);

        expect_token!(self, res, TokenType::Lt, "<");

        advance!(self, res);

        let type_carrier = res.register_res(self.parse_intrinsic_type());
        if res.has_error() {
            return res;
        }
        let element_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

        expect_token!(self, res, TokenType::Gt, ">");

        advance!(self, res);

        if self.current_token().token_type() == TokenType::Newline {
            advance!(self, res);

            expect_token!(self, res, TokenType::U64, "size");

            let length = self.current_token().token_value().as_ref().unwrap().parse::<usize>().unwrap();

            advance!(self, res);

            expect_token!(self, res, TokenType::Rsquare, "]");

            advance!(self, res);

            res.success(Box::new(ListNode::new(length, vec![], false, element_type, pos_start, self.current_token().pos_end().clone())));
            return res;
        }

        if self.current_token().token_type() == TokenType::Rsquare {
            res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected int or array element!"));
            return res;
        }

        let mut elements: Vec<Box<dyn Node>> = vec![];

        loop {
            let expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            elements.push(expr.unwrap());

            if self.current_token().token_type() != TokenType::Comma {
                break;
            } else {
                advance!(self, res);
            }
        }

        expect_token!(self, res, TokenType::Rsquare, "]");

        advance!(self, res);

        res.success(Box::new(ListNode::new(elements.len(), elements, true, element_type, pos_start, self.current_token().pos_end().clone())));
        res
    }

    fn function_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token().pos_start().clone();

        expect_keyword!(self, res, "fun");

        advance!(self, res);

        expect_token!(self, res, TokenType::Identifier, "function name");

        let func_name = self.current_token().token_value().as_ref().unwrap().clone();

        advance!(self, res);

        expect_token!(self, res, TokenType::Lparen, "(");

        advance!(self, res);

        let mut args: Vec<(String, Box<dyn ValueType>)> = vec![];

        if self.current_token().token_type() == TokenType::Identifier {
            expect_token_value!(self, res);

            let arg_name = self.current_token().token_value().as_ref().unwrap().clone();

            for (key, _) in &args {
                if &arg_name == key {
                    res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), format!("Argument name '{}' was already declared!", &arg_name).as_str()));
                    return res;
                }
            }

            advance!(self, res);

            expect_token!(self, res, TokenType::Colon, ":");

            advance!(self, res);

            expect_token!(self, res, TokenType::Keyword, "type");

            let type_carrier = res.register_res(self.parse_intrinsic_type());
            if res.has_error() {
                return res;
            }
            let arg_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

            args.push((arg_name, arg_type));

            while self.current_token().token_type() == TokenType::Comma {
                advance!(self, res);

                expect_token_value!(self, res);

                let arg_name = self.current_token().token_value().as_ref().unwrap().clone();

                for (key, _) in &args {
                    if &arg_name == key {
                        res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), format!("Argument name '{}' was already declared!", &arg_name).as_str()));
                        return res;
                    }
                }

                advance!(self, res);

                expect_token!(self, res, TokenType::Colon, ":");

                advance!(self, res);

                expect_token!(self, res, TokenType::Keyword, "type");

                let type_carrier = res.register_res(self.parse_intrinsic_type());
                if res.has_error() {
                    return res;
                }
                let arg_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

                args.push((arg_name, arg_type));
            }
        }

        expect_token!(self, res, TokenType::Rparen, ")");

        advance!(self, res);

        expect_token!(self, res, TokenType::Colon, ":");

        advance!(self, res);

        expect_token!(self, res, TokenType::Keyword, "type");

        let type_carrier = res.register_res(self.parse_intrinsic_type());
        if res.has_error() {
            return res;
        }
        let return_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

        if self.current_token().token_type() == TokenType::Newline {
            res.success(Box::new(FunctionDeclarationNode::new(func_name, args, return_type, pos_start, self.current_token().pos_end().clone())));
            return res;
        }

        let func_body = res.register_res(self.statements(false));
        if res.has_error() {
            return res;
        }

        res.success(Box::new(FunctionDefinitionNode::new(func_name, args, return_type, func_body.unwrap(), pos_start)));
        res
    }

    fn eval_bin_op_function(&mut self, func: BinOpFunction) -> ParseResult {
        match func {
            BinOpFunction::Arith => self.arith_expression(),
            BinOpFunction::Comp => self.comp_expression(),
            BinOpFunction::Term => self.term(),
            BinOpFunction::Factor => self.factor(),
            BinOpFunction::Call => self.call()
        }
    }
    fn bin_operation(&mut self, fn_a: BinOpFunction, ops: Vec<TokenType>, fn_b: BinOpFunction) -> ParseResult {
        let mut res = ParseResult::new();

        let mut left = res.register_res(self.eval_bin_op_function(fn_a));
        if res.has_error() {
            return res;
        }

        while ops.contains(&self.current_token().token_type()) {
            let op_token = self.current_token().clone();

            advance!(self, res);

            let right = res.register_res(self.eval_bin_op_function(fn_b));
            if res.has_error() {
                return res;
            }

            left = Some(Box::new(BinOpNode::new(left.unwrap(), op_token, right.unwrap())));
        }

        res.success(left.unwrap());
        res
    }

    fn statements(&mut self, is_top_level: bool) -> ParseResult {
        let mut res = ParseResult::new();

        expect_token!(self, res, TokenType::Lcurly, "{");
        advance!(self, res);

        let mut statements: Vec<Box<dyn Node>> = vec![];
        let pos_start = self.current_token().pos_start().clone();

        while self.current_token().token_type() == TokenType::Newline {
            advance!(self, res);
        }

        let statement = res.register_res(self.statement(is_top_level));
        if res.has_error() {
            return res;
        }
        statements.push(statement.unwrap());

        expect_token!(self, res, TokenType::Newline, ";");
        advance!(self, res);

        while self.current_token().token_type() != TokenType::Eof && self.current_token().token_type() != TokenType::Rcurly {
            let statement = res.register_res(self.statement(is_top_level));
            if res.has_error() {
                return res;
            }
            statements.push(statement.unwrap());

            expect_token!(self, res, TokenType::Newline, ";");
            advance!(self, res);
        }

        advance!(self, res);

        res.success(Box::new(StatementsNode::new(statements, pos_start, self.current_token().pos_end().clone())));
        res
    }

    fn statement(&mut self, is_top_level: bool) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token().pos_start().clone();

        if is_top_level {
            if self.current_token().matches_keyword("fun") {
                let func_def = res.register_res(self.function_def());

                if res.has_error() {
                    return res;
                }

                res.success(func_def.unwrap());
                return res;
            }

            if self.current_token().matches_keyword("const") {
                let pos_start = self.current_token().pos_start().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Identifier, "name of constant");

                let name = self.current_token().token_value().as_ref().unwrap().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Colon, ":");

                advance!(self, res);

                let type_carrier = res.register_res(self.parse_intrinsic_type());
                if res.has_error() {
                    return res;
                }
                let const_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

                expect_token!(self, res, TokenType::Eq, "=");

                advance!(self, res);

                let value_node = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::new(ConstDefinitionNode::new(name, value_node.unwrap(), const_type, pos_start)));
                return res;
            }

            if self.current_token().matches_keyword("struct") {
                advance!(self, res);

                expect_token!(self, res, TokenType::Identifier, "name of structure");

                let name = self.current_token().token_value().as_ref().unwrap().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Lcurly, "{");

                advance!(self, res);

                let mut fields: Vec<(String, Box<dyn ValueType>)> = Vec::new();
                loop {
                    expect_token!(self, res, TokenType::Identifier, "name of field");

                    let field_name = self.current_token().token_value().as_ref().unwrap().clone();

                    advance!(self, res);

                    expect_token!(self, res, TokenType::Colon, ":");

                    advance!(self, res);

                    let type_carrier = res.register_res(self.parse_intrinsic_type());
                    if res.has_error() {
                        return res;
                    }

                    let field_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

                    fields.push((field_name, field_type));

                    if self.current_token().token_type() != TokenType::Comma {
                        break;
                    }

                    advance!(self, res);
                }

                expect_token!(self, res, TokenType::Rcurly, "}");

                advance!(self, res);

                res.success(Box::new(StructDefinitionNode::new(name, fields, pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            if self.current_token().matches_keyword("import") {
                let pos_start = self.current_token().pos_start().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::String, "name of module");

                let module_name = self.current_token().token_value().as_ref().unwrap().clone();
                let mut module_path = Path::join(Path::new(self.current_token().pos_start().file_name()).parent().unwrap(), Path::new(&module_name));

                advance!(self, res);

                let mut include_paths = self.include_paths.clone();
                include_paths.reverse();

                while (!module_path.exists() || !module_path.is_file()) && !include_paths.is_empty() {
                    let ip = include_paths.pop().unwrap();
                    module_path = Path::new(ip.as_str()).join(&module_name);
                }

                if !module_path.exists() || !module_path.is_file() {
                    res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), format!("Module '{}' not found!", &module_name).as_str()));
                    return res;
                }

                for ai in self.already_included.iter() {
                    if is_same_file(ai, &module_path).unwrap_or(true) {
                        res.success(Box::new(IgnoredNode::new(self.current_token().pos_start().clone(), self.current_token().pos_end().clone())));
                        return res;
                    }
                }

                let file_text_res = fs::read_to_string(module_path.clone());

                if let Err(file_err) = file_text_res {
                    res.failure(error::semantic_error_with_parent(pos_start.clone(), self.current_token().pos_end().clone(), format!("Failed to import module '{}'!", &module_name).as_str(),
                        error::io_error(pos_start, self.current_token().pos_end().clone(), format!("File '{}' couldn't be opened!\n\t{}", module_path.to_str().unwrap(), file_err.to_string()).as_str())
                    ));
                    return res;
                }

                let file_text = file_text_res.unwrap();

                let mut l = Lexer::new(module_path.clone(), file_text);
                let lexer_res = l.make_tokens();

                if let Err(lexing_error) = lexer_res {
                    res.failure(error::semantic_error_with_parent(pos_start, self.current_token().pos_end().clone(), format!("Failed to import module '{}'!", &module_name).as_str(), lexing_error));
                    return res;
                }
                let tokens = lexer_res.unwrap();

                let mut p = Parser::new(tokens, self.include_paths, self.macros, self.already_included);
                let parse_res = p.parse();

                if let Err(parse_error) = parse_res {
                    res.failure(error::semantic_error_with_parent(pos_start, self.current_token().pos_end().clone(), format!("Failed to import module '{}'!", &module_name).as_str(), parse_error));
                    return res;
                }
                let root_node = parse_res.unwrap();

                self.already_included.push(module_path.clone());

                res.success(Box::new(ImportNode::new(root_node)));
                return res;
            }

            if self.current_token().matches_keyword("macro") {
                let pos_start = self.current_token().pos_start().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Identifier, "name of macro");

                let name = self.current_token().token_value().as_ref().unwrap().clone();

                // for now allow macro redifinition

                advance!(self, res);

                let macro_body = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                self.macros.insert(name, macro_body.as_ref().unwrap().clone());

                res.success(Box::new(MacroDefNode::new(pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            if self.current_token().matches_keyword("static") {
                advance!(self, res);

                let mut is_mutable: bool = false;
                if self.current_token().matches_keyword("mut") {
                    advance!(self, res);
                    is_mutable = true;
                }

                expect_token!(self, res, TokenType::Identifier, "name of static");

                let static_name = self.current_token().token_value().as_ref().unwrap().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Colon, ":");

                advance!(self, res);

                let type_carrier = res.register_res(self.parse_intrinsic_type());
                if res.has_error() {
                    return res;
                }
                let in_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

                expect_token!(self, res, TokenType::Eq, "=");

                advance!(self, res);

                let expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::new(StaticDefinitionNode::new(static_name, in_type, expr.unwrap(), is_mutable, pos_start)));
                return res;
            }

        } else {
            if self.current_token().token_type() == TokenType::Lcurly {
                let block = res.register_res(self.statements(false));
                if res.has_error() {
                    return res;
                }

                res.success(block.unwrap());
                return res;
            }

            if self.current_token().matches_keyword("return") {
                advance!(self, res);

                let (expr, _) = res.try_register_res(self.expression());
                if expr.is_none() {
                    self.reverse(res.to_reverse_count());
                }

                res.success(Box::new(ReturnNode::new(expr, pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            if self.current_token().matches_keyword("continue") {
                advance!(self, res);

                res.success(Box::new(ContinueNode::new(pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            if self.current_token().matches_keyword("break") {
                advance!(self, res);

                res.success(Box::new(BreakNode::new(pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            if self.current_token().matches_keyword("asm") {
                advance!(self, res);

                expect_token!(self, res, TokenType::Lsquare, "[");

                advance!(self, res);

                expect_token!(self, res, TokenType::String, "assembly code");

                let asm_str = self.current_token().token_value().as_ref().unwrap().clone();

                advance!(self, res);

                expect_token!(self, res, TokenType::Rsquare, "]");

                advance!(self, res);

                res.success(Box::new(AssemblyNode::new(asm_str, pos_start, self.current_token().pos_end().clone())));
                return res;
            }

            let expr = res.register_res(self.expression());
            if res.has_error() {
                res.failure(error::invalid_syntax_error_with_parent(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected non top level statement or expression!", res.error().as_ref().unwrap().clone()));
                return res;
            }

            res.success(expr.unwrap());
            return res;
        }

        res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected statement or expression!"));
        res
    }

    fn expression(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token().pos_start().clone();

        if self.current_token().matches_keyword("let") {
            advance!(self, res);

            let mut is_mutable = false;
            if self.current_token().matches_keyword("mut") {
                is_mutable = true;
                advance!(self, res);
            }

            expect_token!(self, res, TokenType::Identifier, "name of variable");
            expect_token_value!(self, res);

            let var_name = self.current_token().token_value().as_ref().unwrap().clone();

            advance!(self, res);

            expect_token!(self, res, TokenType::Colon, ":");

            advance!(self, res);

            expect_token!(self, res, TokenType::Keyword, "type");

            let type_carrier = res.register_res(self.parse_intrinsic_type());
            if res.has_error() {
                return res;
            }
            let var_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

            expect_token!(self, res, TokenType::Eq, "=");

            advance!(self, res);

            let expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(VarDeclarationNode::new(var_name, var_type, expr.unwrap(), is_mutable, pos_start)));
            return res;
        }

        if self.current_token().matches_keyword("syscall") {
            advance!(self, res);

            expect_token!(self, res, TokenType::Lsquare, "[");

            advance!(self, res);

            let mut exprs: Vec<Box<dyn Node>> = vec![];
            exprs.reserve(4);

            let new_expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            exprs.push(new_expr.unwrap());

            for _ in 0..3 {
                expect_token!(self, res, TokenType::Comma, ",");

                advance!(self, res);

                let new_expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                exprs.push(new_expr.unwrap());
            }

            expect_token!(self, res, TokenType::Rsquare, "]");

            advance!(self, res);

            res.success(Box::new(SyscallNode::new(exprs, pos_start, self.current_token().pos_end().clone())));
            return res;
        }

        let node = res.register_res(self.bin_operation(BinOpFunction::Comp, vec![TokenType::And, TokenType::Or], BinOpFunction::Comp));
        if res.has_error() {
            res.failure(error::invalid_syntax_error_with_parent(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected comparison expression!", res.error().as_ref().unwrap().clone()));
            return res;
        }

        if self.current_token().token_type() == TokenType::ReadBytes {
            let token = self.current_token().clone();

            let bytes: u64 = token.token_value().as_ref().unwrap().parse::<u64>().unwrap();
            if bytes != 1 && bytes != 2 && bytes != 4 && bytes != 8 {
                res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Invalid number of bytes!"));
                return res;
            }

            let size: ValueSize = match bytes {
                1 => ValueSize::Byte,
                2 => ValueSize::Word,
                4 => ValueSize::Dword,
                8 => ValueSize::Qword,
                _ => unreachable!(),
            };

            advance!(self, res);

            res.success(Box::new(ReadBytesNode::new(node.unwrap(), size, self.current_token().pos_end().clone())));
            return res;
        }

        res.success(node.unwrap());
        res
    }

    fn comp_expression(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let node = res.register_res(self.bin_operation(
            BinOpFunction::Arith,
            vec![
                TokenType::Ee,
                TokenType::Ne,
                TokenType::Gt,
                TokenType::Lt,
                TokenType::Gte,
                TokenType::Lte,
                TokenType::PointerAssign
            ],
            BinOpFunction::Arith,
        ));
        if res.has_error() {
            res.failure(error::invalid_syntax_error_with_parent(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected arithmetic expression!", res.error().as_ref().unwrap().clone()));
            return res;
        }

        res.success(node.unwrap());
        res
    }

    fn arith_expression(&mut self) -> ParseResult {
        self.bin_operation(
            BinOpFunction::Term,
            vec![
                TokenType::Plus,
                TokenType::Minus,
            ],
            BinOpFunction::Term,
        )
    }

    fn term(&mut self) -> ParseResult {
        self.bin_operation(
            BinOpFunction::Factor,
            vec![
                TokenType::Mul,
                TokenType::Div,
                TokenType::Modulo,
            ],
            BinOpFunction::Factor,
        )
    }

    fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_token().token_type() == TokenType::Plus
            || self.current_token().token_type() == TokenType::Minus
            || self.current_token().token_type() == TokenType::Not
            || self.current_token().token_type() == TokenType::BitNot
        {
            let token = self.current_token().clone();

            advance!(self, res);

            let factor = res.register_res(self.factor());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(UnaryOpNode::new(token, factor.unwrap())));
            return res;
        }

        if self.current_token().token_type() == TokenType::Mul {
            advance!(self, res);

            let factor = res.register_res(self.factor());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(DereferenceNode::new(factor.unwrap())));
            return res;
        }

        self.bin_operation(
            BinOpFunction::Call,
            vec![
                TokenType::BitAnd,
                TokenType::BitOr,
                TokenType::BitXor,
                TokenType::BitShl,
                TokenType::BitShr,
            ],
            BinOpFunction::Call,
        )
    }

    fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let mut atom = res.register_res(self.atom());
        if res.has_error() {
            return res;
        }

        if self.current_token().token_type() == TokenType::Lparen {
            advance!(self, res);

            let mut arg_nodes: Vec<Box<dyn Node>> = vec![];

            if self.current_token().token_type() == TokenType::Rparen {
                advance!(self, res);
            } else {
                let new_arg = res.register_res(self.expression());
                if res.has_error() {
                    res.failure(error::invalid_syntax_error_with_parent(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected expression as function call argument!", res.error().as_ref().unwrap().clone()));
                    return res;
                }

                arg_nodes.push(new_arg.unwrap());

                // TODO: refactor this!
                while self.current_token().token_type() == TokenType::Comma {
                    advance!(self, res);

                    let new_arg = res.register_res(self.expression());
                    if res.has_error() {
                        res.failure(error::invalid_syntax_error_with_parent(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Expected expression as function call argument!", res.error().as_ref().unwrap().clone()));
                        return res;
                    }

                    arg_nodes.push(new_arg.unwrap());
                }

                expect_token!(self, res, TokenType::Rparen, ")");

                advance!(self, res);
            }

            //res.success(Box::new(CallNode::new(atom.unwrap(), arg_nodes)));
            if atom.as_ref().unwrap().node_type() != NodeType::VarAccess {
                res.failure(error::invalid_syntax_error(self.current_token().pos_start().clone(), self.current_token().pos_end().clone(), "Dynamic calls aren't implemented yet!"));
            }

            atom = Some(Box::new(CallNode::new(atom.as_ref().unwrap().as_any().downcast_ref::<VarAccessNode>().unwrap().var_name().to_string(), arg_nodes, atom.as_ref().unwrap().pos_start().clone())))
        }

        while self.current_token().token_type() == TokenType::Lsquare || self.current_token().token_type() == TokenType::Dot {
            let token = self.current_token().clone();

            let s_pos_start = self.current_token().pos_start().clone();

            advance!(self, res);

            if token.token_type() == TokenType::Lsquare {
                let expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                expect_token!(self, res, TokenType::Rsquare, "]");

                atom = Some(Box::new(BinOpNode::new(atom.unwrap(), Token::new_without_value(TokenType::Offset, s_pos_start, self.current_token().pos_end().clone()), expr.unwrap())));
            } else if token.token_type() == TokenType::Dot {
                expect_token!(self, res, TokenType::Identifier, "accessor");
                expect_token_value!(self, res);

                let accessor = self.current_token().token_value().as_ref().unwrap().clone();
                atom = Some(Box::new(AccessorNode::new(atom.unwrap(), accessor, self.current_token().pos_end().clone())));
            }

            advance!(self, res);
        }

        if self.current_token().matches_keyword("as") {
            advance!(self, res);

            let type_carrier = res.register_res(self.parse_intrinsic_type());
            if res.has_error() {
                return res;
            }
            let cast_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

            atom = Some(Box::new(CastNode::new(atom.unwrap(), cast_type, self.current_token().pos_end().clone())));
        }

        res.success(atom.unwrap());
        res
    }

    fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token().clone();

        let node: Box<dyn Node>;

        if token.token_type() == TokenType::U64 {
            advance!(self, res);

            node = Box::new(NumberNode::new(token, Box::new(U64Type::new())));
        } else if token.token_type() == TokenType::String {
            advance!(self, res);

            node = Box::new(StringNode::new(token));
        } else if token.token_type() == TokenType::Char {
            let actual_char = self.current_token().token_value().as_ref().unwrap().chars().next().unwrap();
            let pos_start = self.current_token().pos_start().clone();
            let pos_end = self.current_token().pos_start().clone();

            advance!(self, res);

            node = Box::new(CharNode::new(actual_char, pos_start, pos_end));
        } else if self.current_token().token_type() == TokenType::Identifier {

            let var_name = self.current_token().token_value().as_ref().unwrap().clone();
            let pos_start = self.current_token().pos_start().clone();

            advance!(self, res);

            if self.macros.contains_key(&var_name) {
                res.success(self.macros[&var_name].clone());
                return res;
            }

            if self.current_token().has_flag(TOKEN_FLAGS_IS_ASSIGN) && (self.current_token().token_type() == TokenType::Plus
                || self.current_token().token_type() == TokenType::Minus
                || self.current_token().token_type() == TokenType::Mul
                || self.current_token().token_type() == TokenType::Div
                || self.current_token().token_type() == TokenType::Modulo
                || self.current_token().token_type() == TokenType::BitAnd
                || self.current_token().token_type() == TokenType::BitOr
                || self.current_token().token_type() == TokenType::BitXor
                || self.current_token().token_type() == TokenType::BitShl
                || self.current_token().token_type() == TokenType::BitShr)
            {
                let op_token = self.current_token().clone();

                advance!(self, res);

                let assign_expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::new(VarAssignNode::new(var_name.clone(),
                                                        Box::new(BinOpNode::new(
                                                            Box::new(VarAccessNode::new(var_name, pos_start.clone(), self.current_token().pos_end().clone())), op_token, assign_expr.unwrap(),
                                                        )), pos_start)));
                return res;
            }

            if self.current_token().token_type() == TokenType::Eq {
                advance!(self, res);

                let expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::new(VarAssignNode::new(var_name, expr.unwrap(), pos_start)));
                return res;
            }

            node = Box::new(VarAccessNode::new(var_name, pos_start, self.current_token().pos_end().clone()));
        } else if token.token_type() == TokenType::Lparen {
            advance!(self, res);

            let expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            if self.current_token().token_type() != TokenType::Rparen {
                res.failure(error::invalid_syntax_error(token.pos_start().clone(), self.current_token().pos_end().clone(), "Expected ')'!"));
                return res;
            }

            advance!(self, res);

            node = expr.unwrap();
        } else if token.token_type() == TokenType::Lsquare {
            let list_expr = res.register_res(self.list_expr());
            if res.has_error() {
                return res;
            }

            node = list_expr.unwrap();
        } else if token.matches_keyword("while") {
            let while_expr = res.register_res(self.while_expr());
            if res.has_error() {
                return res;
            }

            node = while_expr.unwrap();
        } else if token.matches_keyword("for") {
            let for_expr = res.register_res(self.for_expr());
            if res.has_error() {
                return res;
            }

            node = for_expr.unwrap();
        } else if token.matches_keyword("if") {
            let if_expr = res.register_res(self.if_expr());
            if res.has_error() {
                return res;
            }

           node = if_expr.unwrap();
        } else if token.matches_keyword("sizeof") {
            advance!(self, res);

            expect_token!(self, res, TokenType::Lsquare, "[");

            advance!(self, res);

            let type_carrier = res.register_res(self.parse_intrinsic_type());
            if res.has_error() {
                return res;
            }
            let in_type = type_carrier.unwrap().as_any().downcast_ref::<TypeCarrierNode>().unwrap().carried_type().clone();

            expect_token!(self, res, TokenType::Rsquare, "]");

            advance!(self, res);

            node = Box::new(SizeOfNode::new(in_type, token.pos_start().clone(), self.current_token().pos_end().clone()));
        } else {
            res.failure(error::invalid_syntax_error(token.pos_start().clone(), token.pos_end().clone(), "Expected atom!"));
            return res
        }

        res.success(node);
        res
    }

    // endregion
}