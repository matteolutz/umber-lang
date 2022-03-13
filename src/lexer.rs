use std::any::Any;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::constants::{DIGITS, ESCAPED_CHARACTERS, LETTERS, LETTERS_AND_DIGITS};
use crate::error::Error;
use crate::error;
use crate::position::Position;
use crate::token::{KEYWORDS, Token, TokenType, TokenValue, TokenValueType};
use crate::token::TokenType::Float;

pub struct Lexer {
    file_name: &'static str,
    file_text: &'static str,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {

    pub fn new(file_name: &'static str, file_text: &'static str) -> Lexer {
        return Lexer {
            file_name,
            file_text,
            pos: Position::new(file_name, file_text),
            current_char: if file_text.len() > 0 { file_text.chars().nth(0) } else { None },
        };
    }


    fn advance(&mut self) -> () {
        self.pos.advance(self.current_char.unwrap());

        if *self.pos.index() < self.file_text.len() {
            //self.current_char = Some(self.file_text[self.pos.index]);
            self.current_char = Some(self.file_text.chars().nth(*self.pos.index()).unwrap());
        } else {
            self.current_char = None;
        }
    }

    pub fn make_tokens(&mut self) -> (Vec<Token>, Option<Error>) {
        let mut tokens: Vec<Token> = vec![Token::new_without_value(TokenType::Eof, self.pos)];

        while self.current_char.is_some() {
            let current = self.current_char.unwrap();

            if [' ', '\t', '\n'].contains(&current) {
                self.advance();
            } else if current == '#' {
                self.skip_comment();
            } else if [';'].contains(&current) {
                tokens.push(Token::new_without_value(TokenType::Newline, self.pos));
                self.advance();
            } else if DIGITS.contains(&current) {
                tokens.push(self.make_number());
            } else if LETTERS.contains(&current) {
                tokens.push(self.make_identifier());
            } else if current == '\"' {
                tokens.push(self.make_string());
            } else if current == '+' {
                tokens.push(Token::new_without_value(TokenType::Plus, self.pos));
                self.advance();
            } else if current == '-' {
                tokens.push(self.make_minus_or_arrow());
            } else if current == '*' {
                tokens.push(Token::new_without_value(TokenType::Mul, self.pos));
                self.advance();
            } else if current == '/' {
                tokens.push(Token::new_without_value(TokenType::Div, self.pos));
                self.advance();
            } else if current == '%' {
                tokens.push(Token::new_without_value(TokenType::Modulo, self.pos));
                self.advance();
            } else if current == '^' {
                tokens.push(Token::new_without_value(TokenType::Pow, self.pos));
                self.advance();
            } else if current == ':' {
                tokens.push(Token::new_without_value(TokenType::Colon, self.pos));
                self.advance();
            } else if current == '(' {
                tokens.push(Token::new_without_value(TokenType::Lparen, self.pos));
                self.advance();
            } else if current == ')' {
                tokens.push(Token::new_without_value(TokenType::Rparen, self.pos));
                self.advance();
            } else if current == '[' {
                tokens.push(Token::new_without_value(TokenType::Lsquare, self.pos));
                self.advance();
            } else if current == ']' {
                tokens.push(Token::new_without_value(TokenType::Rsquare, self.pos));
                self.advance();
            } else if current == '{' {
                tokens.push(Token::new_without_value(TokenType::Lcurly, self.pos));
                self.advance();
            } else if current == '}' {
                tokens.push(Token::new_without_value(TokenType::Rcurly, self.pos));
                self.advance();
            } else if current == '!' {
                tokens.push(self.make_not_equals());
            } else if current == '=' {
                tokens.push(self.make_equals());
            } else if current == '<' {
                tokens.push(self.make_less_than());
            } else if current == '>' {
                tokens.push(self.make_greater_than());
            } else if current == '&' {
                tokens.push(self.make_and());
            } else if current == '|' {
                tokens.push(self.make_or());
            } else if current == ',' {
                tokens.push(Token::new_without_value(TokenType::Comma, self.pos));
                self.advance();
            } else {
                let pos_start = self.pos;
                self.advance();

                return (vec![], Some(error::illegal_character_error(pos_start, self.pos, format!("'{}'", current).as_str())));
            }
        }

        tokens.push(Token::new_without_value(TokenType::Eof, self.pos));
        (tokens, None)
    }

    fn skip_comment(&mut self) -> () {
        self.advance();

        while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
            self.advance();
        }

        self.advance();
    }

    fn skip_multiline_comment(&mut self) -> () {
        self.advance();

        let mut found_asterisk = false;
        while self.current_char.is_some() {

            if self.current_char.unwrap() == '*' {
                found_asterisk = true;
            } else if self.current_char.unwrap() == '/' && found_asterisk {
                self.advance();
                break
            } else {
                found_asterisk = false;
            }

            self.advance();
        }

    }

    fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count: u8 = 0;
        let pos_start = self.pos;

        let number_chars: [char; 11] = concat_arrays::concat_arrays!(DIGITS, ['.']);

        while self.current_char.is_some() && number_chars.contains(&self.current_char.unwrap()) {
            let current = self.current_char.unwrap();

            if current == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
                num_str.push('.');
            } else {
                num_str.push(current);
            }

            self.advance();
        }

        if dot_count == 0 {
            Token::new(TokenType::Int, Some(TokenValue::new(TokenValueType::Int, num_str)), pos_start, self.pos)
        } else {
            // Token::new(TokenType::Float, Some(TokenValue::new(TokenValueType::Float, num_str)), pos_start, self.pos)
            panic!("Floats are not supported for now!");
        }
    }

    fn make_identifier(&mut self) -> Token {
        let mut id_str = String::new();
        let mut pos_start = self.pos;

        let identifier_chars: [char; 63] = concat_arrays::concat_arrays!(LETTERS_AND_DIGITS, ['_']);

        while self.current_char.is_some() && identifier_chars.contains(&self.current_char.unwrap()) {
            id_str.push(self.current_char.unwrap());
            self.advance();
        }

        let token_type = if KEYWORDS.contains(&id_str.as_str()) {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        };
        Token::new(token_type, Some(TokenValue::new(TokenValueType::String, id_str)), pos_start, self.pos)
    }

    fn make_string(&mut self) -> Token {
        let mut new_string = String::new();
        let pos_start = self.pos;
        let mut escape_character = false;

        self.advance();

        while self.current_char.is_some() && (self.current_char.unwrap() != '\"' || escape_character) {
            let current = self.current_char.unwrap();

            let mut escaped_characters_map: HashMap<char, char> = HashMap::from_iter(ESCAPED_CHARACTERS);

            if escape_character {
                new_string.push(escaped_characters_map[&current]);
                escape_character = false;
            } else {
                if current == '\\' {
                    escape_character = true;
                } else {
                    new_string.push(current);
                }
            }

            self.advance();
        }

        self.advance();

        Token::new(TokenType::String, Some(TokenValue::new(TokenValueType::String, new_string)), pos_start, self.pos)
    }

    fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::Minus;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '>' {
            self.advance();
            token_type = TokenType::Arrow;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_not_equals(&mut self) -> Token {
        let mut token_type = TokenType::Not;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Ne;
        }

        return Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_equals(&mut self) -> Token {
        let mut token_type = TokenType::Eq;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Ee;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_less_than(&mut self) -> Token {
        let mut token_type = TokenType::Lt;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Lte;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::Gt;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Gte;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_and(&mut self) -> Token {
        let mut token_type = TokenType::BitAnd;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '&' {
            self.advance();
            token_type = TokenType::And;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

    fn make_or(&mut self) -> Token {
        let mut token_type = TokenType::BitOr;
        let pos_start = self.pos;

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '|' {
            self.advance();
            token_type = TokenType::Or;
        }

        Token::new(token_type, None, pos_start, self.pos)
    }

}