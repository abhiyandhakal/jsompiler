use super::symbol::{Lexeme, LiteralToken, Token, lexeme};
use crate::parser::symbol;

pub struct Lexer {
    pub source: Vec<char>, // Code to be scanned
    pub tokens: Vec<Lexeme>,
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

    fn scan_token(&mut self) -> Option<symbol::Lexeme> {
        if self.is_at_end() {
            return Some(symbol::Lexeme {
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

                Some(lexeme(
                    token_string.clone(),
                    Token::Literal(LiteralToken::Number(token_string)),
                ));
            }
            _ => {}
        }

        None
    }

    pub fn scan_all_tokens(&mut self) {
        while !self.is_at_end() {
            if let Some(token) = self.scan_token() {
                self.tokens.push(token);
            }
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while self.get_current_char() == ' ' {
            self.advance();
        }
    }
}
