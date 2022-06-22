use std::fmt::{Debug, Display, Formatter};

use crate::position::Position;

pub const TOKEN_FLAGS_NULL: u8 = 0;

pub const TOKEN_FLAGS_IS_ASSIGN: u8 = 1 << 0;

#[derive(Clone)]
pub struct Token {
    tok_type: TokenType,
    tok_value: Option<String>,
    pos_start: Position,
    pos_end: Position,
    flags: u8,
}

impl Token {

    pub fn new_without_value(tok_type: TokenType, pos_start: Position, pos_end: Position) -> Token {
        Token::new(tok_type, None, pos_start, pos_end, TOKEN_FLAGS_NULL)
    }

    pub fn new_with_value(tok_type: TokenType, tok_value: String, pos_start: Position, pos_end: Position) -> Token {
        Token::new(tok_type, Some(tok_value), pos_start, pos_end, TOKEN_FLAGS_NULL)
    }

    pub fn new_with_flags(tok_type: TokenType, tok_value: String, pos_start: Position, pos_end: Position, flags: u8) -> Token {
        Token::new(tok_type, Some(tok_value), pos_start, pos_end, flags)
    }

    pub fn new_with_flags_no_value(tok_type: TokenType, pos_start: Position, pos_end: Position, flags: u8) -> Token {
        Token::new(tok_type, None, pos_start, pos_end, flags)
    }

    pub fn new(tok_type: TokenType, tok_value: Option<String>, pos_start: Position, pos_end: Position, flags: u8) -> Token {
        Token {
            tok_type,
            tok_value,
            pos_start,
            pos_end,
            flags
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

    pub fn flags(&self) -> &u8 { &self.flags }

    pub fn matches_keyword(&self, value_str: &str) -> bool {
        self.matches(TokenType::Keyword, value_str)
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        self.flags & flag == flag
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
    U64,
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
    PlusAssign,
    MinusAssign,
    MulAssign,
    DivAssign,
    ModuloAssign,
    PlusPlus,
    MinusMinus,
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
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    BitShl,
    BitShlAssign,
    BitShr,
    BitShrAssign,
    BitNot,
    BitNotAssign,
    Not,
    Comma,
    Arrow,
    Newline,
    Dereference,
    Dot,
    Offset,
    ReadBytes,
    PointerAssign,
    Bof,
    Eof,
}

pub static KEYWORDS: [&'static str; 28] = [
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
    "u64",
    "u32",
    "u16",
    "u8",
    "string",
    "bool",
    "char",
    "void",
    "as",
    "static",
    "struct",
    "import",
    "macro"
];