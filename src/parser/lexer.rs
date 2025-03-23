use crate::parser::symbol;
use crate::parser::symbol::SYMBOLS;

pub struct Lexer {
    source: Vec<char>, // Code to be scanned
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
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
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source[self.current - 1];
        }
    }

    fn scan_token(&mut self) -> Option<symbol::Symbol> {
        //
    }

    pub fn scan_all_tokens(&mut self) {
        while !self.is_at_end() {
            let _token = self.scan_token();
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while self.get_current_char() == ' ' {
            self.advance();
        }
    }
}
