mod comment;
mod identifier;
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
        let c = self.advance();

        match c {
            // New Line
            '\n' => {
                // self.advance();
                Ok(Some(lexeme(
                    "\n".to_string(),
                    Token::Delimiter(DelimiterToken::NewLine),
                )))
            }
            // Numbers
            '0'..='9' => self.lex_number(),
            // Keywords and identifiers
            'a'..='z' | 'A'..='Z' | '_' | '$' => self.lex_identifier(),
            // String
            '`' => self.lex_template_string(),
            '"' => self.lex_string('"'),
            '\'' => self.lex_string('\''),
            '/' => self.lex_comment(),
            c => self.lex_operator_punctuation(c),
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
        while self.get_current_char() == ' ' || self.get_current_char() == '\t' {
            self.advance();
        }
    }
}
