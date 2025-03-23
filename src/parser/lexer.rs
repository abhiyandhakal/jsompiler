use super::symbol::{Lexeme, LiteralToken, SYMBOLS, Token, lexeme};
use crate::{Error, ErrorKind, parser::symbol};

pub struct Lexer {
    pub source: Vec<char>, // Code to be scanned
    pub tokens: Vec<Lexeme>,
    pub errors: Vec<crate::Error>,
    pub start: usize,
    pub current: usize,
    pub line_number: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            tokens: vec![],
            errors: vec![],
            line_number: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn get_current_char(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source[self.current];
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        if self.get_current_char() == '\n' {
            self.line_number += 1;
        }

        if self.is_at_end() {
            return '\0';
        } else {
            return self.source[self.current - 1];
        }
    }

    fn scan_token(&mut self) -> Result<Lexeme, crate::Error> {
        if self.is_at_end() {
            return Ok(symbol::Lexeme {
                text: "EOF".to_string(),
                len: 0,
                token: symbol::Token::EOF,
            });
        }

        self.skip_whitespaces();
        self.start = self.current;
        let c = self.advance();

        match c {
            // Numbers
            '0'..='9' => {
                while self.get_current_char().is_ascii_digit() {
                    self.advance();
                }
                let token_string: String = self.source[self.start..self.current].iter().collect();

                return Ok(lexeme(
                    token_string.clone(),
                    Token::Literal(LiteralToken::Number(token_string)),
                ));
            }
            // Keywords and identifiers
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                while self.get_current_char().is_alphanumeric()
                    || self.get_current_char() == '_'
                    || self.get_current_char() == '$'
                {
                    self.advance();
                }
                let token_string = self.source[self.start..self.current]
                    .iter()
                    .collect::<String>();

                let keyword = SYMBOLS.iter().find(|f| f.0.to_string() == token_string);
                if let Some(keyword) = keyword {
                    return Ok(keyword.1.clone());
                }

                return Ok(lexeme(
                    token_string.clone(),
                    Token::Identifier(token_string),
                ));
            }
            '`' => {
                while self.get_current_char() != '`' {
                    if self.advance() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "Template string (`) not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }
                }
                return Ok(lexeme(
                    self.source[self.start..=self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                        self.source[self.start + 1..self.current].iter().collect(),
                    ))),
                ));
            }
            '"' => {
                loop {
                    self.advance();
                    if self.get_current_char() == '"' {
                        break;
                    }

                    if self.get_current_char() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "String (\") not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }

                    if self.get_current_char() == '\n' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "String (\") not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }
                }
                return Ok(lexeme(
                    self.source[self.start..=self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                        self.source[self.start + 1..self.current].iter().collect(),
                    ))),
                ));
            }
            '\'' => {
                loop {
                    self.advance();
                    if self.get_current_char() == '\'' {
                        break;
                    }

                    if self.get_current_char() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "String (\") not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }

                    if self.get_current_char() == '\n' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "String (') not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }
                }
                return Ok(lexeme(
                    self.source[self.start..=self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                        self.source[self.start + 1..self.current].iter().collect(),
                    ))),
                ));
            }
            _ => {}
        }

        Err(Error {
            error_kind: ErrorKind::LexerError,
            message: "Unexpected character".to_string(),
            line_number: self.line_number,
            pos: self.start,
        })
    }

    pub fn scan_all_tokens(&mut self) {
        loop {
            match self.scan_token() {
                Ok(token) => {
                    self.tokens.push(token.clone());

                    if token.token == Token::EOF {
                        break;
                    }
                }
                Err(error) => {
                    self.errors.push(error);
                    self.go_to_new_line();
                }
            }
        }
    }

    fn go_to_new_line(&mut self) {
        if let Some(index) = self
            .source
            .iter()
            .skip(self.current)
            .position(|&c| c == '\n')
            .map(|pos| pos + self.current)
        {
            self.start = index + 1;
            self.current = index + 1;
        } else {
            self.start = self.source.len();
            self.current = self.source.len();
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while self.get_current_char() == ' ' {
            self.advance();
        }
    }
}
