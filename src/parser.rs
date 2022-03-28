use std::collections::HashMap;

use crate::error;
use crate::error::Error;
use crate::nodes::{Node, NodeType};
use crate::nodes::asm_node::AssemblyNode;
use crate::nodes::binop_node::BinOpNode;
use crate::nodes::break_node::BreakNode;
use crate::nodes::call_node::CallNode;
use crate::nodes::continue_node::ContinueNode;
use crate::nodes::functiondef_node::FunctionDefinitionNode;
use crate::nodes::number_node::NumberNode;
use crate::nodes::return_node::ReturnNode;
use crate::nodes::statements_node::StatementsNode;
use crate::nodes::string_node::StringNode;
use crate::nodes::unaryop_node::UnaryOpNode;
use crate::nodes::var_node::access::VarAccessNode;
use crate::nodes::var_node::assign::VarAssignNode;
use crate::nodes::var_node::declare::VarDeclarationNode;
use crate::results::parse::ParseResult;
use crate::token::{Token, TokenType};
use crate::values::value_type::array_type::ArrayType;
use crate::values::value_type::bool_type::BoolType;
use crate::values::value_type::number_type::NumberType;
use crate::values::value_type::string_type::StringType;
use crate::values::value_type::ValueType;
use crate::values::value_type::void_type::VoidType;

#[derive(Copy, Clone)]
enum BinOpFunction {
    Arith = 0,
    Comp = 1,
    Term = 2,
    Factor = 3,
    Call = 4,
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        if tokens.is_empty() {
            panic!("No tokens were provided!");
        }

        Parser {
            tokens,
            token_index: 0,
        }
    }

    fn current_token(&self) -> &Token { return &self.tokens[self.token_index]; }

    fn advance(&mut self) -> () {
        self.token_index += 1;
    }

    fn reverse(&mut self, amount: usize) -> () {
        self.token_index -= amount;
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_token().token_type() == TokenType::Bof {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected BOF!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        let stmts = res.register_res(self.statements(true));
        if res.has_error() {
            return res;
        }

        if !res.has_error() && self.current_token().token_type() != TokenType::Eof {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected statements!"));
            return res;
        }

        res.success(stmts.unwrap());
        res
    }

    // region Helper functions

    fn parse_intrinsic_type(&mut self) -> (Option<Box<dyn ValueType>>, Option<Error>) {
        if self.current_token().token_type() != TokenType::Keyword {
            panic!("TokenType '{:?}' can't be handled in function `get_intrinsic_type``!", self.current_token().token_type());
        }

        let s = self.current_token().token_value().as_ref().unwrap().as_str();

        let base_type: Box<dyn ValueType> = match s {
            "number" => Box::new(NumberType::new()),
            "string" => Box::new(StringType::new()),
            "bool" => Box::new(BoolType::new()),
            "void" => Box::new(VoidType::new()),
            _ => return (None, Some(Error::new(*self.current_token().pos_start(), *self.current_token().pos_end(), "NotAnIntrinsicType".to_string(), format!("'{}' is not an intrinsic type!", self.current_token().token_value().as_ref().unwrap()))))
        };

        self.advance();

        if self.current_token().token_type() == TokenType::Lsquare {
            self.advance();

            if self.current_token().token_type() != TokenType::Int {
                self.reverse(2);
                return (None, Some(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected integer after '[' on an array type!")));
            }

            // TODO: error handling
            let size: u64 = self.current_token().token_value().as_ref().unwrap().parse::<u64>().unwrap();

            self.advance();

            if self.current_token().token_type() != TokenType::Rsquare {
                self.reverse(3);
                return (None, Some(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ']' after integer on an array type!")));
            }

            return (Some(Box::new(ArrayType::new(size, base_type))), None)
        }

        self.reverse(1);

        (Some(base_type), None)
        /*if s == "number" {
            return Box::new(NumberType::new());
        } else if s == "string" {
            return Box::new(StringType::new());
        } else if s == "bool" {
            return Box::new(BoolType::new());
        } else if s == "void" {
            return Box::new(VoidType::new());
        }

        panic!("'{}' is not an intrinsic type!", s)*/
    }

    // endregion

    // region Parsing functions

    fn if_expr(&mut self) -> ParseResult {
        let res = ParseResult::new();

        res
    }

    fn for_expr(&mut self) -> ParseResult {
        let res = ParseResult::new();

        res
    }

    fn while_expr(&mut self) -> ParseResult {
        let res = ParseResult::new();

        res
    }

    fn list_expr(&mut self) -> ParseResult {
        let res = ParseResult::new();

        res
    }

    fn dict_expr(&mut self) -> ParseResult {
        let res = ParseResult::new();

        res
    }

    fn function_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = *self.current_token().pos_start();

        if !self.current_token().matches_keyword("fun") {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected 'fun'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        if self.current_token().token_type() != TokenType::Identifier {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected identifier!"));
            return res;
        }

        let func_name = self.current_token().token_value().as_ref().unwrap().clone();

        res.register_advancement();
        self.advance();

        if self.current_token().token_type() != TokenType::Lparen {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '('!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        let mut args: HashMap<String, Box<dyn ValueType>> = HashMap::new();

        if self.current_token().token_type() == TokenType::Identifier {
            if self.current_token().token_value().is_none() {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                return res;
            }

            let arg_name = self.current_token().token_value().as_ref().unwrap().clone();

            if args.contains_key(&arg_name) {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), format!("Argument name '{}' was already declared!", &arg_name).as_str()));
                return res;
            }

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Colon {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ':'!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Keyword {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected type keyword!"));
                return res;
            }

            let i_type = self.parse_intrinsic_type();
            if i_type.1.is_some() {
                res.failure(i_type.1.unwrap());
                return res;
            }

            args.insert(arg_name, i_type.0.unwrap());

            res.register_advancement();
            self.advance();

            while self.current_token().token_type() == TokenType::Comma {
                res.register_advancement();
                self.advance();

                if self.current_token().token_value().is_none() {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                    return res;
                }

                let arg_name = self.current_token().token_value().as_ref().unwrap().clone();

                if args.contains_key(&arg_name) {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), format!("Argument name '{}' was already declared!", &arg_name).as_str()));
                    return res;
                }

                res.register_advancement();
                self.advance();

                if self.current_token().token_type() != TokenType::Colon {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ':'!"));
                    return res;
                }

                res.register_advancement();
                self.advance();

                if self.current_token().token_type() != TokenType::Keyword {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected type keyword!"));
                    return res;
                }

                let i_type = self.parse_intrinsic_type();
                if i_type.1.is_some() {
                    res.failure(i_type.1.unwrap());
                    return res;
                }

                args.insert(arg_name, i_type.0.unwrap());

                res.register_advancement();
                self.advance();
            }
        }

        if self.current_token().token_type() != TokenType::Rparen {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ')'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        if self.current_token().token_type() != TokenType::Colon {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ':'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        if self.current_token().token_type() != TokenType::Keyword {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected type keyword!"));
            return res;
        }

        let i_type = self.parse_intrinsic_type();
        if i_type.1.is_some() {
            res.failure(i_type.1.unwrap());
            return res;
        }

        let return_type = i_type.0.unwrap();

        res.register_advancement();
        self.advance();

        if self.current_token().token_type() == TokenType::Arrow {
            res.register_advancement();
            self.advance();

            let node_to_return = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(FunctionDefinitionNode::new(func_name, args, return_type, node_to_return.unwrap(), true, pos_start)));
            return res;
        }

        if self.current_token().token_type() != TokenType::Lcurly {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '{'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        let func_body = res.register_res(self.statements(false));
        if res.has_error() {
            return res;
        }

        if self.current_token().token_type() != TokenType::Rcurly {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '}'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        res.success(Box::new(FunctionDefinitionNode::new(func_name, args, return_type, func_body.unwrap(), false, pos_start)));
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

            res.register_advancement();
            self.advance();

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

        let mut statements: Vec<Box<dyn Node>> = vec![];
        let pos_start = *self.current_token().pos_start();

        while self.current_token().token_type() == TokenType::Newline {
            res.register_advancement();
            self.advance();
        }

        let statement = res.register_res(self.statement(is_top_level));
        if res.has_error() {
            return res;
        }
        statements.push(statement.unwrap());

        let mut more_statements = true;

        loop {
            let mut newline_count: usize = 0;

            while self.current_token().token_type() == TokenType::Newline {
                res.register_advancement();
                self.advance();
                newline_count += 1;
            }

            if newline_count == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            let statement = res.try_register_res(self.statement(is_top_level));
            if statement.is_none() {
                self.reverse(res.to_reverse_count());
                more_statements = false;
                continue;
            }

            statements.push(statement.unwrap());
        }

        res.success(Box::new(StatementsNode::new(statements, pos_start, *self.current_token().pos_start())));
        res
    }

    fn statement(&mut self, is_top_level: bool) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = *self.current_token().pos_start();

        if is_top_level {
            if self.current_token().matches_keyword("import") {
                todo!("import");
            }

            if self.current_token().matches_keyword("fun") {
                let func_def = res.register_res(self.function_def());

                if res.has_error() {
                    return res;
                }

                res.success(func_def.unwrap());
                return res;
            }

            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected top level statement!"));
        } else {
            if self.current_token().matches_keyword("return") {
                res.register_advancement();
                self.advance();

                let expr = res.try_register_res(self.expression());
                if expr.is_none() {
                    self.reverse(res.to_reverse_count());
                }

                res.success(Box::new(ReturnNode::new(expr, pos_start, *self.current_token().pos_end())));
                return res;
            }

            if self.current_token().matches_keyword("continue") {
                res.register_advancement();
                self.advance();

                res.success(Box::new(ContinueNode::new(pos_start, *self.current_token().pos_end())));
                return res;
            }

            if self.current_token().matches_keyword("break") {
                res.register_advancement();
                self.advance();

                res.success(Box::new(BreakNode::new(pos_start, *self.current_token().pos_end())));
                return res;
            }

            if self.current_token().matches_keyword("asm__") {
                let pos_start = *self.current_token().pos_start();

                res.register_advancement();
                self.advance();

                if self.current_token().token_type() != TokenType::Lparen {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '('!"));
                    return res;
                }

                res.register_advancement();
                self.advance();

                if self.current_token().token_type() != TokenType::String {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected string!"));
                    return res;
                }

                let asm_str = self.current_token().token_value().as_ref().unwrap().clone();

                res.register_advancement();
                self.advance();

                if self.current_token().token_type() != TokenType::Rparen {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ')'!"));
                    return res;
                }

                res.register_advancement();
                self.advance();

                res.success(Box::new(AssemblyNode::new(asm_str, pos_start, *self.current_token().pos_end())));
                return res;
            }

            let expr = res.register_res(self.expression());
            if res.has_error() {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected non top level statement or expression!"));
                return res;
            }

            res.success(expr.unwrap());
        }

        res
    }

    fn expression(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = *self.current_token().pos_start();

        if self.current_token().matches_keyword("let") {
            res.register_advancement();
            self.advance();

            let mut is_mutable = false;
            if self.current_token().matches_keyword("mut") {
                is_mutable = true;
                res.register_advancement();
                self.advance();
            }

            if self.current_token().token_type() != TokenType::Identifier {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), if is_mutable { "Expected identifier!" } else { "Expected 'mut' or identifier!" }));
                return res;
            }

            if self.current_token().token_value().is_none() {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                return res;
            }

            let var_name = self.current_token().token_value().as_ref().unwrap().clone();

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Colon {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ':'!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Keyword {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected type keyword!"));
                return res;
            }

            let i_type = self.parse_intrinsic_type();
            if i_type.1.is_some() {
                res.failure(i_type.1.unwrap());
                return res;
            }

            let value_type = i_type.0.unwrap();

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Eq {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '='!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            let expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(VarDeclarationNode::new(var_name, value_type, expr.unwrap(), is_mutable, pos_start)));
            return res;
        }

        let node = res.register_res(self.bin_operation(BinOpFunction::Comp, vec![TokenType::And, TokenType::Or], BinOpFunction::Comp));
        if res.has_error() {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected comparison expression!"));
            return res;
        }

        res.success(node.unwrap());
        res
    }

    fn comp_expression(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_token().token_type() == TokenType::Not {
            let op_token = self.current_token().clone();

            res.register_advancement();
            self.advance();

            let node = res.register_res(self.comp_expression());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(UnaryOpNode::new(op_token, node.unwrap())));
            return res;
        }

        let node = res.register_res(self.bin_operation(
            BinOpFunction::Arith,
            vec![
                TokenType::Ee,
                TokenType::Ee,
                TokenType::Gt,
                TokenType::Lt,
                TokenType::Gte,
                TokenType::Lte,
            ],
            BinOpFunction::Arith,
        ));
        if res.has_error() {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected arithmetic expression!"));
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
                TokenType::BitAnd,
                TokenType::BitOr,
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
        if self.current_token().token_type() == TokenType::Plus || self.current_token().token_type() == TokenType::Minus {
            let mut res = ParseResult::new();
            let token = self.current_token().clone();

            res.register_advancement();
            self.advance();

            let factor = res.register_res(self.factor());
            if res.has_error() {
                return res;
            }

            res.success(Box::new(UnaryOpNode::new(token, factor.unwrap())));
            return res;
        }

        return self.power();
    }

    fn power(&mut self) -> ParseResult {
        self.bin_operation(
            BinOpFunction::Call,
            vec![TokenType::Pow],
            BinOpFunction::Factor,
        )
    }

    fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let atom = res.register_res(self.atom());
        if res.has_error() {
            return res;
        }

        if self.current_token().token_type() == TokenType::Lparen {
            res.register_advancement();
            self.advance();

            let mut arg_nodes: Vec<Box<dyn Node>> = vec![];

            if self.current_token().token_type() == TokenType::Rparen {
                res.register_advancement();
                self.advance();
            } else {
                let new_arg = res.register_res(self.expression());
                if res.has_error() {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected expression!"));
                    return res;
                }

                arg_nodes.push(new_arg.unwrap());

                // TODO: refactor this!
                while self.current_token().token_type() == TokenType::Comma {
                    res.register_advancement();
                    self.advance();

                    let new_arg = res.register_res(self.expression());
                    if res.has_error() {
                        res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected expression!"));
                        return res;
                    }

                    arg_nodes.push(new_arg.unwrap());
                }

                if self.current_token().token_type() != TokenType::Rparen {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ')'!"));
                    return res;
                }

                res.register_advancement();
                self.advance();
            }

            //res.success(Box::new(CallNode::new(atom.unwrap(), arg_nodes)));
            if atom.as_ref().unwrap().node_type() != NodeType::VarAccess {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Dynamic calls aren't implemented yet!"));
            }

            res.success(Box::new(CallNode::new(atom.as_ref().unwrap().as_any().downcast_ref::<VarAccessNode>().unwrap().var_name().to_string(), arg_nodes, *atom.as_ref().unwrap().pos_start())));
            return res;
        }

        res.success(atom.unwrap());
        res
    }

    fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token().clone();

        if token.token_type() == TokenType::Int || token.token_type() == TokenType::Float {
            res.register_advancement();
            self.advance();

            res.success(Box::new(NumberNode::new(token)));
            return res;
        }

        if token.token_type() == TokenType::String {
            res.register_advancement();
            self.advance();

            res.success(Box::new(StringNode::new(token)));
            return res;
        }

        if token.token_type() == TokenType::Identifier {
            res.register_advancement();
            self.advance();

            let var_name = token.token_value().as_ref().unwrap().clone();

            if self.current_token().token_type() == TokenType::Eq {
                res.register_advancement();
                self.advance();

                let expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::new(VarAssignNode::new(var_name, expr.unwrap(), *token.pos_start())));
                return res;
            }

            res.success(Box::new(VarAccessNode::new(var_name, *token.pos_start(), *token.pos_end())));
            return res;
        }

        if token.token_type() == TokenType::Lparen {
            res.register_advancement();
            self.advance();

            let expr = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            if self.current_token().token_type() != TokenType::Rparen {
                res.failure(error::invalid_syntax_error(*token.pos_start(), *self.current_token().pos_end(), "Expected ')'!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            res.success(expr.unwrap());
            return res;
        }

        // TODO: list!

        // TODO: if, for, while

        res.failure(error::invalid_syntax_error(*token.pos_start(), *token.pos_end(), "Expected atom!"));
        res
    }

    // endregion
}