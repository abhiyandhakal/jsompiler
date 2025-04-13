mod comment;
mod identifier;
mod jsx;
mod number;
mod operator_punctuation;
mod string;
pub mod symbol;
mod test;

use jsompiler_common::Error;
use symbol::{DelimiterToken, Lexeme, Token, lexeme};

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

    fn is_beyond_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn get_current_char(&self) -> char {
        if self.current >= self.source.len() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        if self.get_current_char() == '\n' {
            self.line_number += 1;
        }

        if self.is_beyond_end() {
            '\0'
        } else {
            self.source[self.current - 1]
        }
    }

    fn scan_token(&mut self) -> Result<Option<Lexeme>, crate::Error> {
        if self.is_beyond_end() {
            return Ok(Some(symbol::Lexeme {
                text: "EOF".to_string(),
                len: 0,
                token: symbol::Token::EOF,
            }));
        }

        self.skip_whitespaces();
        self.start = self.current;

        let c = self.get_current_char();

        match c {
            '\n' => {
                self.advance(); // only advance here
                Ok(Some(lexeme(
                    "\n".to_string(),
                    Token::Delimiter(DelimiterToken::NewLine),
                )))
            }
            '0'..='9' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' | '$' | '#' => self.lex_identifier(), // consume inside
            '`' => {
                self.advance();
                self.lex_template_string()
            }
            '"' | '\'' => {
                self.advance();
                self.lex_string(c)
            }

            '/' => self.lex_comment(), // lex_comment handles advancing
            '<' => self.lex_jsx(),
            _ => {
                let c = self.advance();
                self.lex_operator_punctuation(c)
            } // advance here and pass the char
        }
    }

    pub fn scan_all_tokens(&mut self) {
        loop {
            match self.scan_token() {
                Ok(Some(token)) => {
                    self.tokens.push(token.clone());

                    if token.token == Token::EOF {
                        break;
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    self.errors.push(error);
                    if !self.go_to_new_line() {
                        break;
                    }
                }
            }
        }
    }

    fn go_to_new_line(&mut self) -> bool {
        if let Some(index) = self
            .source
            .iter()
            .skip(self.current)
            .position(|&c| c == '\n')
            .map(|pos| pos + self.current)
        {
            self.start = index + 1;
            self.current = index + 1;
            true
        } else {
            self.start = self.source.len();
            self.current = self.source.len();
            false
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while self.get_current_char() == ' ' || self.get_current_char() == '\t' {
            self.advance();
        }
    }
}
