use std::any::{Any, TypeId};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::position::Position;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenValueType {
    Int,
    Float,
    String,
    Null
}

#[derive(Clone)]
pub struct TokenValue {
    value_type: TokenValueType,
    value: String
}

impl TokenValue {

    pub fn is_type(&self, value_type: &TokenValueType) -> bool {
        &self.value_type == value_type
    }

    pub fn get_as_i32(&self) -> i32 {
        self.value.parse::<i32>().unwrap()
    }

    pub fn get_as_u32(&self) -> u32 {
        self.value.parse::<u32>().unwrap()
    }

    pub fn get_as_f32(&self) -> f32 {
        self.value.parse::<f32>().unwrap()
    }

    pub fn get_as_str(&self) -> &String {
        &self.value
    }

    pub fn new(value_type: TokenValueType, value: String) -> TokenValue {
        TokenValue {
            value_type,
            value
        }
    }

}

impl PartialEq for TokenValue {
    fn eq(&self, other: &Self) -> bool {
        return self.value_type == other.value_type && self.value == other.value;
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}>{}", self.value_type, self.value)
    }
}

#[derive(Clone)]
pub struct Token {
    tok_type: TokenType,
    tok_value: Option<TokenValue>,
    pos_start: Position,
    pos_end: Position,
}

impl Token {

    pub fn new_without_value(tok_type: TokenType, pos_start: Position) -> Token {
        Token::new(tok_type, Option::None, pos_start, pos_start)
    }

    pub fn new_with_value(tok_type: TokenType, tok_value: TokenValue, pos_start: Position) -> Token {
        Token::new(tok_type, Some(tok_value), pos_start, pos_start)
    }

    pub fn new(tok_type: TokenType, tok_value: Option<TokenValue>, pos_start: Position, pos_end: Position) -> Token {
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

    pub fn token_value(&self) -> &Option<TokenValue> {
        &self.tok_value
    }

    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    pub fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    pub fn matches_string(&self, token_type: TokenType, value_str: &str) -> bool {
        self.matches(token_type, &Some(TokenValue::new(TokenValueType::String, String::from(value_str))))
    }

    pub fn matches_keyword(&self, value_str: &str) -> bool {
        self.matches_string(TokenType::Keyword, value_str)
    }

    pub fn matches(&self, token_type: TokenType, token_value: &Option<TokenValue>) -> bool {
        let type_eq = self.tok_type == token_type;
        let mut value_eq = false;

        if self.tok_value.is_none() && token_value.is_none() {
            value_eq = true;
        } else if self.tok_value.is_some() && token_value.is_some() {
            value_eq = self.tok_value.as_ref().unwrap() == token_value.as_ref().unwrap();
        }

        type_eq && value_eq
    }

}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        /*let type_eq = self.tok_type == other.tok_type;
        let mut value_eq = false;

        if self.tok_value.is_none() && other.tok_value.is_none() {
            value_eq = true;
        } else if self.tok_value.is_some() && other.tok_value.is_some() {
            value_eq = self.tok_value.as_ref().unwrap() == other.tok_value.as_ref().unwrap();
        }

        type_eq && value_eq*/
        self.matches(other.token_type(), other.token_value())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.tok_type, self.tok_value.as_ref().unwrap_or(&TokenValue{ value_type: TokenValueType::Null, value: String::from("NULL") }))
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Int,
    Float,
    String,
    Identifier,
    Keyword,
    Plus,
    Minus,
    Mul,
    Div,
    Modulo,
    Pow,
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
    Not,
    Comma,
    Arrow,
    Newline,
    Bof,
    Eof,
}

pub static KEYWORDS: [&'static str; 16] = [
    "let",
    "mut",
    "if",
    "elif",
    "else",
    "for",
    "to",
    "step",
    "while",
    "fun",
    "then",
    "return",
    "continue",
    "break",
    "import",

    "u64"
];