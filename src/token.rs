use std::fmt::{Debug, Display, Formatter};

use crate::position::Position;

#[derive(Clone)]
pub struct Token {
    tok_type: TokenType,
    tok_value: Option<String>,
    pos_start: Position,
    pos_end: Position,
}

impl Token {

    pub fn new_without_value(tok_type: TokenType, pos_start: Position, pos_end: Position) -> Token {
        Token::new(tok_type, None, pos_start, pos_end)
    }

    pub fn new_with_value(tok_type: TokenType, tok_value: String, pos_start: Position, pos_end: Position) -> Token {
        Token::new(tok_type, Some(tok_value), pos_start, pos_end)
    }

    pub fn new(tok_type: TokenType, tok_value: Option<String>, pos_start: Position, pos_end: Position) -> Token {
        Token {
            tok_type,
            tok_value,
            pos_start,
            pos_end
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.tok_type
    }

    pub fn token_value(&self) -> &Option<String> {
        &self.tok_value
    }

    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    pub fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    pub fn matches_keyword(&self, value_str: &str) -> bool {
        self.matches(TokenType::Keyword, value_str)
    }

    pub fn matches(&self, token_type: TokenType, token_value: &str) -> bool {
        if self.tok_value.is_none() {
            return false;
        }

        self.tok_type == token_type && self.tok_value.as_ref().unwrap() == token_value
    }

}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.matches(other.token_type(), other.token_value().as_ref().unwrap().as_str())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.tok_type, self.tok_value.as_ref().unwrap_or(&String::from("<NULL>")))
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Int,
    Float,
    String,
    Char,
    Identifier,
    Keyword,
    Plus,
    Minus,
    Mul,
    Div,
    Modulo,
    Eq,
    Colon,
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lcurly,
    Rcurly,
    Ee,
    Ne,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    BitShl,
    BitShr,
    BitNot,
    Not,
    Comma,
    Arrow,
    Newline,
    Dereference,
    Offset,
    ReadBytes,
    PointerAssign,
    Bof,
    Eof,
}

pub static KEYWORDS: [&'static str; 23] = [
    "let",
    "mut",
    "const",
    "if",
    "else",
    "for",
    "while",
    "fun",
    "return",
    "continue",
    "break",
    "extern",
    "asm",
    "sizeof",
    "syscall",
    "number",
    "string",
    "bool",
    "char",
    "void",
    "as",
    "static",
    "struct"
];