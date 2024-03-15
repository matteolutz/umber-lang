use crate::error::Error;
use crate::position::Position;
use crate::token::{Token, TokenType, KEYWORDS, TOKEN_FLAGS_IS_ASSIGN, TOKEN_FLAGS_NULL};
use crate::{error, utils};
use std::path::PathBuf;

pub struct Lexer {
    _file_path: PathBuf,
    file_text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(file_path: PathBuf, file_text: String) -> Self {
        Self {
            pos: Position::new(file_path.clone()),
            current_char: if file_text.len() > 0 {
                Some(file_text.chars().nth(0).unwrap())
            } else {
                None
            },
            _file_path: file_path,
            file_text,
        }
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char.as_ref().unwrap());

        if *self.pos.index() < self.file_text.len() {
            self.current_char = Some(self.file_text.chars().nth(*self.pos.index()).unwrap());
        } else {
            self.current_char = None;
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![Token::new_without_value(
            TokenType::Lcurly,
            self.pos.clone(),
            self.pos.clone(),
        )];

        while self.current_char.is_some() {
            let current = self.current_char.unwrap();

            if current == ' ' || current == '\t' || current == '\n' || current == '\r' {
                self.advance();
            } else if current == ';' {
                tokens.push(Token::new_without_value(
                    TokenType::Newline,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if utils::is_digit(&current) {
                tokens.push(self.make_number()?);
            } else if utils::is_alpha(&current) {
                tokens.push(self.make_identifier());
            } else if current == '"' {
                tokens.push(self.make_string());
            } else if current == '\'' {
                tokens.push(self.make_char()?);
            } else if current == '+' {
                tokens.push(self.make_plus());
            } else if current == '-' {
                tokens.push(self.make_minus_or_arrow());
            } else if current == '*' {
                tokens.push(self.make_mul());
            } else if current == '/' {
                if let Some(token) = self.make_div_or_comment()? {
                    tokens.push(token);
                }
            } else if current == '%' {
                tokens.push(self.make_modulo());
            } else if current == '^' {
                tokens.push(self.make_bit_xor());
            } else if current == ':' {
                tokens.push(Token::new_without_value(
                    TokenType::Colon,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '(' {
                tokens.push(Token::new_without_value(
                    TokenType::Lparen,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == ')' {
                tokens.push(Token::new_without_value(
                    TokenType::Rparen,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '[' {
                tokens.push(Token::new_without_value(
                    TokenType::Lsquare,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == ']' {
                tokens.push(Token::new_without_value(
                    TokenType::Rsquare,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '{' {
                tokens.push(Token::new_without_value(
                    TokenType::Lcurly,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '}' {
                tokens.push(Token::new_without_value(
                    TokenType::Rcurly,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
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
                tokens.push(Token::new_without_value(
                    TokenType::BitNot,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '@' {
                let pos_start = self.pos.clone();

                self.advance();

                if self.current_char.is_some() && self.current_char.unwrap() == '=' {
                    tokens.push(Token::new_without_value(
                        TokenType::PointerAssign,
                        pos_start,
                        self.pos.clone(),
                    ));
                    self.advance();
                    continue;
                }

                if self.current_char.is_none()
                    || !utils::is_digit(self.current_char.as_ref().unwrap())
                {
                    return Err(error::illegal_character_error(
                        pos_start,
                        self.pos.clone(),
                        "Expected number after '@'!",
                    ));
                }

                let number = self.make_number()?;
                if number.token_type() != TokenType::U64 {
                    return Err(error::illegal_character_error(
                        pos_start,
                        self.pos.clone(),
                        "Expected integer number after '@'!",
                    ));
                }

                tokens.push(Token::new_with_value(
                    TokenType::ReadBytes,
                    number.token_value().as_ref().unwrap().clone(),
                    pos_start,
                    self.pos.clone(),
                ));
            } else if current == ',' {
                tokens.push(Token::new_without_value(
                    TokenType::Comma,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else if current == '.' {
                tokens.push(Token::new_without_value(
                    TokenType::Dot,
                    self.pos.clone(),
                    self.pos.clone(),
                ));
                self.advance();
            } else {
                let pos_start = self.pos.clone();
                self.advance();

                return Err(error::illegal_character_error(
                    pos_start,
                    self.pos.clone(),
                    format!("'{}'", current).as_str(),
                ));
            }
        }

        tokens.push(Token::new_without_value(
            TokenType::Rcurly,
            self.pos.clone(),
            self.pos.clone(),
        ));
        tokens.push(Token::new_without_value(
            TokenType::Eof,
            self.pos.clone(),
            self.pos.clone(),
        ));
        Ok(tokens)
    }

    fn skip_comment(&mut self) -> () {
        self.advance();

        while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
            self.advance();
        }

        self.advance();
    }

    fn skip_multiline_comment(&mut self) -> Result<(), Error> {
        let pos_start = self.pos.clone();
        self.advance();

        let mut found_asterisk = false;
        while self.current_char.is_some() {
            if self.current_char.unwrap() == '*' {
                found_asterisk = true;
            } else if self.current_char.unwrap() == '/' && found_asterisk {
                self.advance();
                return Ok(());
            } else {
                found_asterisk = false;
            }

            self.advance();
        }

        Err(error::expected_character_error(
            pos_start,
            self.pos.clone(),
            "Expected '*/'!",
        ))
    }

    fn make_number(&mut self) -> Result<Token, Error> {
        let mut num_str = String::new();
        let mut dot_count: u8 = 0;
        let pos_start = self.pos.clone();

        while self.current_char.is_some()
            && (utils::is_digit(self.current_char.as_ref().unwrap())
                || self.current_char.unwrap() == '.')
        {
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

        Ok(Token::new_with_value(
            if dot_count == 0 {
                TokenType::U64
            } else {
                TokenType::F64
            },
            num_str,
            pos_start,
            self.pos.clone(),
        ))
    }

    fn make_identifier(&mut self) -> Token {
        let mut id_str = String::new();
        let pos_start = self.pos.clone();

        while self.current_char.is_some()
            && (utils::is_digit(self.current_char.as_ref().unwrap())
                || utils::is_alpha(self.current_char.as_ref().unwrap())
                || self.current_char.unwrap() == '_')
        {
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

    fn make_char(&mut self) -> Result<Token, Error> {
        let mut new_char: char;
        let mut escaped = false;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.unwrap() == '\\' {
            escaped = true;
            self.advance();
        }

        if self.current_char.is_none() {
            return Err(error::invalid_syntax_error(
                pos_start,
                self.pos.clone(),
                "Expected escaped character, after '''!",
            ));
        }

        new_char = self.current_char.unwrap();
        if escaped {
            if let Some(c) = utils::escape_char(&new_char) {
                new_char = c;
            } else {
                return Err(error::invalid_syntax_error(
                    pos_start,
                    self.pos.clone(),
                    "Invalid escaped character!",
                ));
            }
        }

        self.advance();

        if self.current_char.is_none() || self.current_char.unwrap() != '\'' {
            return Err(error::invalid_syntax_error(
                pos_start,
                self.pos.clone(),
                "Expected ' after character!",
            ));
        }

        self.advance();

        Ok(Token::new_with_value(
            TokenType::Char,
            new_char.to_string(),
            pos_start,
            self.pos.clone(),
        ))
    }

    fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::Minus;
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '-' {
            self.advance();
            token_type = TokenType::MinusMinus;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
            token_type = TokenType::Minus;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '>' {
            self.advance();
            token_type = TokenType::Arrow;
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
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
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Lte;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '<' {
            self.advance();
            token_type = TokenType::BitShl;

            if self.current_char.is_some() && self.current_char.unwrap() == '=' {
                self.advance();
                flags = TOKEN_FLAGS_IS_ASSIGN;
            }
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
    }

    fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::Gt;
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            token_type = TokenType::Gte;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '>' {
            self.advance();
            token_type = TokenType::BitShr;

            if self.current_char.is_some() && self.current_char.unwrap() == '=' {
                self.advance();
                flags = TOKEN_FLAGS_IS_ASSIGN;
            }
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
    }

    fn make_and(&mut self) -> Token {
        let mut token_type = TokenType::BitAnd;
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '&' {
            self.advance();
            token_type = TokenType::And;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
    }

    fn make_or(&mut self) -> Token {
        let mut token_type = TokenType::BitOr;
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '|' {
            self.advance();
            token_type = TokenType::Or;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
    }

    fn make_plus(&mut self) -> Token {
        let mut token_type = TokenType::Plus;
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '+' {
            self.advance();
            token_type = TokenType::PlusPlus;
        } else if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(token_type, pos_start, self.pos.clone(), flags)
    }

    fn make_mul(&mut self) -> Token {
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(TokenType::Mul, pos_start, self.pos.clone(), flags)
    }

    fn make_div_or_comment(&mut self) -> Result<Option<Token>, Error> {
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        } else if self.current_char.unwrap() == '*' {
            self.skip_multiline_comment()?;
            return Ok(None);
        } else if self.current_char.unwrap() == '/' {
            self.skip_comment();
            return Ok(None);
        }

        Ok(Some(Token::new_with_flags_no_value(
            TokenType::Div,
            pos_start,
            self.pos.clone(),
            flags,
        )))
    }

    fn make_modulo(&mut self) -> Token {
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(TokenType::Modulo, pos_start, self.pos.clone(), flags)
    }

    fn make_bit_xor(&mut self) -> Token {
        let mut flags = TOKEN_FLAGS_NULL;
        let pos_start = self.pos.clone();

        self.advance();

        if self.current_char.is_some() && self.current_char.unwrap() == '=' {
            self.advance();
            flags = TOKEN_FLAGS_IS_ASSIGN;
        }

        Token::new_with_flags_no_value(TokenType::BitXor, pos_start, self.pos.clone(), flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_make_token_identifier() -> Result<(), Error> {
        let mut lexer = Lexer::new(PathBuf::new(), "let five: number = 5;".to_string());
        let tokens = lexer.make_tokens()?;

        assert_eq!(tokens.len(), 9);

        assert_eq!(tokens[0].token_type(), TokenType::Bof);

        assert!(tokens[1].matches_keyword("let"));

        assert_eq!(tokens[2].token_type(), TokenType::Identifier);
        assert!(tokens[2].token_value().is_some());
        assert_eq!(tokens[2].token_value().as_ref().unwrap(), "five");

        assert_eq!(tokens[3].token_type(), TokenType::Colon);

        assert!(tokens[4].matches_keyword("number"));

        assert_eq!(tokens[5].token_type(), TokenType::Eq);

        assert_eq!(tokens[6].token_type(), TokenType::U64);
        assert!(tokens[6].token_value().is_some());
        assert_eq!(tokens[6].token_value().as_ref().unwrap(), "5");

        assert_eq!(tokens[7].token_type(), TokenType::Newline);

        assert_eq!(tokens[8].token_type(), TokenType::Eof);

        Ok(())
    }
}
