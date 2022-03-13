use std::any::Any;
use std::detect::__is_feature_detected::adx;
use crate::results::parse::ParseResult;
use crate::token::{Token, TokenType, TokenValue, TokenValueType};
use crate::error;
use crate::nodes::binop::BinOpNode;
use crate::nodes::call::CallNode;
use crate::nodes::functiondef::FunctionDefinitionNode;
use crate::nodes::list::ListNode;
use crate::nodes::nbreak::BreakNode;
use crate::nodes::ncontinue::ContinueNode;
use crate::nodes::{Node, NodeType};
use crate::nodes::nreturn::ReturnNode;
use crate::nodes::number::NumberNode;
use crate::nodes::statements::StatementsNode;
use crate::nodes::string::StringNode;
use crate::nodes::unaryop::UnaryOpNode;
use crate::nodes::var::access::VarAccessNode;
use crate::nodes::var::assign::VarAssignNode;
use crate::nodes::var::declare::VarDeclarationNode;
use crate::parser::BinOpFunction::Call;
use crate::position::Position;

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

        let stmts = res.register_res(self.statements());
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

    // region Parsing functions

    fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        res
    }

    fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        res
    }

    fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        res
    }

    fn list_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        res
    }

    fn dict_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

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

        let mut func_name: Option<String> = None;

        if self.current_token().token_type() == TokenType::Identifier {
            if self.current_token().token_value().is_none() {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                return res;
            }

            func_name = Some(self.current_token().token_value().as_ref().unwrap().get_as_str().clone());

            res.register_advancement();
            self.advance();
        }

        if self.current_token().token_type() != TokenType::Lparen {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '('!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        let mut arg_names: Vec<String> = vec![];
        if self.current_token().token_type() == TokenType::Identifier {
            if self.current_token().token_value().is_none() {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                return res;
            }

            arg_names.push(self.current_token().token_value().as_ref().unwrap().get_as_str().clone());

            res.register_advancement();
            self.advance();

            while self.current_token().token_type() == TokenType::Comma {
                res.register_advancement();
                self.advance();

                if self.current_token().token_value().is_none() {
                    res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                    return res;
                }

                arg_names.push(self.current_token().token_value().as_ref().unwrap().get_as_str().clone());

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

        if self.current_token().token_type() == TokenType::Arrow {
            res.register_advancement();
            self.advance();

            let node_to_return = res.register_res(self.expression());
            if res.has_error() {
                return res;
            }

            res.success(Box::from(FunctionDefinitionNode::new(func_name, arg_names, node_to_return.unwrap(), true, pos_start)));
            return res;
        }

        if self.current_token().token_type() != TokenType::Lcurly {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '{'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        let func_body = res.register_res(self.statements());
        if res.has_error() {
            return res;
        }

        if self.current_token().token_type() != TokenType::Rcurly {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '}'!"));
            return res;
        }

        res.register_advancement();
        self.advance();

        res.success(Box::from(FunctionDefinitionNode::new(func_name, arg_names, func_body.unwrap(), false, pos_start)));
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

            left = Some(Box::from(BinOpNode::new(left.unwrap(), op_token, right.unwrap())));
        }

        res.success(left.unwrap());
        res
    }

    fn statements(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let mut statements: Vec<Box<dyn Node>> = vec![];
        let pos_start = *self.current_token().pos_start();

        while self.current_token().token_type() == TokenType::Newline {
            res.register_advancement();
            self.advance();
        }

        let statement = res.register_res(self.statement());
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

            let statement = res.try_register_res(self.statement());
            if statement.is_none() {
                self.reverse(res.to_reverse_count().clone());
                more_statements = false;
                continue;
            }

            statements.push(statement.unwrap());
        }

        res.success(Box::from(StatementsNode::new(statements, pos_start, *self.current_token().pos_start())));
        res
    }

    fn statement(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = *self.current_token().pos_start();

        if self.current_token().matches_keyword("return") {

            res.register_advancement();
            self.advance();

            let expr = res.try_register_res(self.expression());
            if expr.is_none() {
                self.reverse(res.to_reverse_count());
            }

            res.success(Box::from(ReturnNode::new(expr, pos_start, *self.current_token().pos_end())));
            return res;
        }

        if self.current_token().matches_keyword("continue") {
            res.register_advancement();
            self.advance();

            res.success(Box::from(ContinueNode::new(pos_start, *self.current_token().pos_end())));
            return res;
        }

        if self.current_token().matches_keyword("break") {
            res.register_advancement();
            self.advance();

            res.success(Box::from(BreakNode::new(pos_start, *self.current_token().pos_end())));
            return res;
        }

        if self.current_token().matches_keyword("import") {
            todo!("import");
        }

        let expr = res.register_res(self.expression());
        if res.has_error() {
            res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected 'return', 'continue', 'break', 'import' or expression!"));
            return res;
        }

        res.success(expr.unwrap());
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

            if self.current_token().token_value().is_none() || !self.current_token().token_value().as_ref().unwrap().is_type(&TokenValueType::String) {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Identifier malformed!"));
                return res;
            }

            let var_name = self.current_token().token_value().as_ref().unwrap().get_as_str().clone();

            res.register_advancement();
            self.advance();

            /*if self.current_token().token_type() != TokenType::Colon {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected ':'!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            if self.current_token().token_type() != TokenType::Identifier {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected type identifier!"));
                return res;
            }

            let value_type = self.current_token().token_value().as_ref().unwrap().get_as_str().clone();

            res.register_advancement();
            self.advance();*/

            if self.current_token().token_type() != TokenType::Eq {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Expected '='!"));
                return res;
            }

            res.register_advancement();
            self.advance();

            let expr = res.register_res(self.expression());
            if res.has_error() {
                panic!("value has err!");
                return res;
            }

            res.success(Box::from(VarDeclarationNode::new(var_name, String::from(""), expr.unwrap(), is_mutable, pos_start)));
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

            res.success(Box::from(UnaryOpNode::new(op_token, node.unwrap())));
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
            BinOpFunction::Arith
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
                TokenType::BitOr
            ],
            BinOpFunction::Term
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
            BinOpFunction::Factor
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

            res.success(Box::from(UnaryOpNode::new(token, factor.unwrap())));
            return res;
        }

        return self.power();
    }

    fn power(&mut self) -> ParseResult {
        self.bin_operation(
            BinOpFunction::Call,
            vec![TokenType::Pow],
            BinOpFunction::Factor
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

            //res.success(Box::from(CallNode::new(atom.unwrap(), arg_nodes)));
            if atom.as_ref().unwrap().node_type() != NodeType::VarAccess {
                res.failure(error::invalid_syntax_error(*self.current_token().pos_start(), *self.current_token().pos_end(), "Dynamic calls aren't implemented yet!"));
            }

            res.success(Box::from(CallNode::new(atom.as_ref().unwrap().as_any().downcast_ref::<VarAccessNode>().unwrap().var_name().clone(), arg_nodes, *atom.as_ref().unwrap().pos_start())));
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

            res.success(Box::from(NumberNode::new(token)));
            return res;
        }

        if token.token_type() == TokenType::String {
            res.register_advancement();
            self.advance();

            res.success(Box::from(StringNode::new(token)));
            return res;
        }

        if token.token_type() == TokenType::Identifier {
            res.register_advancement();
            self.advance();

            let var_name = token.token_value().as_ref().unwrap().get_as_str().clone();

            if self.current_token().token_type() == TokenType::Eq {
                res.register_advancement();
                self.advance();

                let expr = res.register_res(self.expression());
                if res.has_error() {
                    return res;
                }

                res.success(Box::from(VarAssignNode::new(var_name, expr.unwrap(), *token.pos_start())));
                return res;
            }

            res.success(Box::from(VarAccessNode::new(var_name, *token.pos_start(), *token.pos_end())));
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

        if token.matches_keyword("fun") {
            let func_def = res.register_res(self.function_def());

            if res.has_error() {
                return res;
            }

            res.success(func_def.unwrap());
            return res;
        }

        res.failure(error::invalid_syntax_error(*token.pos_start(), *token.pos_end(), "Expected atom!"));
        res
    }

    // endregion
}