use crate::{error, utils};
use crate::error::Error;
use crate::position::Position;
use crate::token::{KEYWORDS, Token, TokenType};
use crate::token::TokenType::Lcurly;

pub struct Lexer {
    file_name: Box<String>,
    file_text: Box<String>,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {

    pub fn new(file_name: Box<String>, file_text: Box<String>) -> Lexer {
        return Lexer {
            file_name: file_name.clone(),
            file_text: file_text.clone(),
            pos: Position::new(file_name.clone(), file_text.clone()),
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
        let mut tokens: Vec<Token> = vec![Token::new_without_value(TokenType::Eof, self.pos.clone(), self.pos.clone())];

        while self.current_char.is_some() {
            let current = self.current_char.unwrap();

            if current == ' ' || current == '\t' || current == '\n' {
                self.advance();
            } else if current == '#' {
                panic!("unallowed character");
            } else if current == ';' {
                tokens.push(Token::new_without_value(TokenType::Newline, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if utils::is_digit(&current) {
                tokens.push(self.make_number());
            } else if utils::is_alpha(&current) {
                tokens.push(self.make_identifier());
            } else if current == '"' {
                tokens.push(self.make_string());
            } else if current == '\'' {
                let (token, error) = self.make_char();
                if error.is_some() {
                    return (vec![], error);
                }
                tokens.push(token.unwrap());
            } else if current == '+' {
                tokens.push(Token::new_without_value(TokenType::Plus, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '-' {
                tokens.push(self.make_minus_or_arrow());
            } else if current == '*' {
                tokens.push(Token::new_without_value(TokenType::Mul, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '/' {
                let pos_start = self.pos.clone();

                self.advance();

                if self.current_char.unwrap() != '/' && self.current_char.unwrap() != '*' {
                    tokens.push(Token::new_without_value(TokenType::Div, pos_start, self.pos.clone()));
                } else if self.current_char.unwrap() == '*' {
                    self.skip_multiline_comment();
                } else {
                    self.skip_comment();
                }

            } else if current == '%' {
                tokens.push(Token::new_without_value(TokenType::Modulo, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '^' {
                tokens.push(Token::new_without_value(TokenType::BitXor, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == ':' {
                tokens.push(Token::new_without_value(TokenType::Colon, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '(' {
                tokens.push(Token::new_without_value(TokenType::Lparen, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == ')' {
                tokens.push(Token::new_without_value(TokenType::Rparen, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '[' {
                tokens.push(Token::new_without_value(TokenType::Lsquare, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == ']' {
                tokens.push(Token::new_without_value(TokenType::Rsquare, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '{' {
                tokens.push(Token::new_without_value(TokenType::Lcurly, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '}' {
                tokens.push(Token::new_without_value(TokenType::Rcurly, self.pos.clone(), self.pos.clone()));
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
            } else if current == '~' {
                tokens.push(Token::new_without_value(TokenType::BitNot, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else if current == '@' {
                let pos_start = self.pos.clone();

                self.advance();

                if self.current_char.is_none() || !utils::is_digit(self.current_char.as_ref().unwrap()) {
                    return (vec![], Some(error::illegal_character_error(pos_start, self.pos.clone(), "Expected number after '@'!")));
                }

                let number = self.make_number();
                if number.token_type() != TokenType::Int {
                    return (vec![], Some(error::illegal_character_error(pos_start, self.pos.clone(), "Expected integer number after '@'!")));
                }

                tokens.push(Token::new_with_value(TokenType::ReadBytes, number.token_value().as_ref().unwrap().clone(), pos_start, self.pos.clone()));
            } else if current == ',' {
                tokens.push(Token::new_without_value(TokenType::Comma, self.pos.clone(), self.pos.clone()));
                self.advance();
            } else {
                let pos_start = self.pos.clone();
                self.advance();

                return (vec![], Some(error::illegal_character_error(pos_start, self.pos.clone(), format!("'{}'", current).as_str())));
            }
        }

        tokens.push(Token::new_without_value(TokenType::Eof, self.pos.clone(), self.pos.clone()));
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
                return;
            } else {
                found_asterisk = false;
            }

            self.advance();
        }

        panic!("no closing thing found!");
    }

    fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count: u8 = 0;
        let pos_start = self.pos.clone();

        while self.current_char.is_some() && (utils::is_digit(self.current_char.as_ref().unwrap()) || self.current_char.unwrap() == '.') {
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
            Token::new_with_value(TokenType::Int, num_str, pos_start, self.pos.clone())
        } else {
            panic!("Floats are not supported for now!");
        }
    }

    fn make_identifier(&mut self) -> Token {
        let mut id_str = String::new();
        let pos_start = self.pos.clone();

        while self.current_char.is_some() && (utils::is_digit(self.current_char.as_ref().unwrap()) || utils::is_alpha(self.current_char.as_ref().unwrap()) || self.current_char.unwrap() == '_') {
            id_str.push(self.current_char.unwrap());
            self.advance();
        }

        let token_type = if KEYWORDS.contains(&id_str.as_str()) {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        };
        Token::new_with_value(token_type, id_str, pos_start, self.pos.clone())
    }

    fn make_string(&mut self) -> Token {
        let mut new_string = String::new();
        let pos_start = self.pos.clone();

        self.advance();

        while self.current_char.is_some() && self.current_char.unwrap() != '\"' {
            let current = self.current_char.unwrap();

            new_string.push(current);

            self.advance();
        }

        self.advance();

        Token::new_with_value(TokenType::String, new_string, pos_start, self.pos.clone())
    }

    fn make_char(&mut self) -> (Option<Token>, Option<Error>) {
        let mut new_char: char;
        let mut escaped = false;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.unwrap() == '\\' {
            escaped = true;
            self.advance();
        }

        if self.current_char.is_none() {
            return (None, Some(error::invalid_syntax_error(pos_start, self.pos.clone(), "Expected escaped character, after '''!")));
        }

        new_char = if escaped { utils::escape_char(self.current_char.as_ref().unwrap()) } else { self.current_char.unwrap() };

        self.advance();

        if self.current_char.is_none() || self.current_char.unwrap() != '\'' {
            return (None, Some(error::invalid_syntax_error(pos_start, self.pos.clone(), "Expected ' after character!")));
        }

        self.advance();

        (Some(Token::new_with_value(TokenType::Char, new_char.to_string(), pos_start, self.pos.clone())), None)
    }

    fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::Minus;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '>' {
            self.advance();
            token_type = TokenType::Arrow;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_not_equals(&mut self) -> Token {
        let mut token_type = TokenType::Not;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Ne;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_equals(&mut self) -> Token {
        let mut token_type = TokenType::Eq;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Ee;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_less_than(&mut self) -> Token {
        let mut token_type = TokenType::Lt;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Lte;
        } else  if self.current_char.is_some() && self.current_char.unwrap() == '<' {
            self.advance();
            token_type = TokenType::BitShl;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::Gt;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Gte;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '>' {
            self.advance();
            token_type = TokenType::BitShr;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_and(&mut self) -> Token {
        let mut token_type = TokenType::BitAnd;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '&' {
            self.advance();
            token_type = TokenType::And;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

    fn make_or(&mut self) -> Token {
        let mut token_type = TokenType::BitOr;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '|' {
            self.advance();
            token_type = TokenType::Or;
        }

        Token::new_without_value(token_type, pos_start, self.pos.clone())
    }

}