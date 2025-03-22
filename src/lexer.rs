use crate::symbol;
use crate::symbol::SYMBOLS;

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
        if self.is_at_end() {
            Some(symbol::symbol(
                "EOF".to_string(),
                symbol::Token::EOF,
                symbol::Group::EOF,
            ))
        } else {
            self.skip_whitespaces();
            self.start = self.current;
            let c = self.advance();
            match c {
                //For number tokens
                '0'..='9' => {
                    while self.get_current_char().is_ascii_digit() {
                        self.advance();
                    }
                    println!(
                        "Number: {}",
                        self.source[self.start..self.current]
                            .iter()
                            .collect::<String>()
                    );

                    let token_string = self.source[self.start..self.current]
                        .iter()
                        .collect::<String>();

                    Some(symbol::symbol(
                        token_string,
                        symbol::Token::Number,
                        symbol::Group::Literal,
                    ))
                }

                //For string tokens
                'a'..='z' | 'A'..='Z' | '_' => {
                    while self.get_current_char().is_alphanumeric()
                        || self.get_current_char() == '_'
                    {
                        self.advance();
                    }
                    println!(
                        "Token: {}",
                        self.source[self.start..self.current]
                            .iter()
                            .collect::<String>()
                    );

                    let token_string = self.source[self.start..self.current]
                        .iter()
                        .collect::<String>();

                    if let Some(symbol) = SYMBOLS.get(token_string.as_str()) {
                        println!("'{}' is a keyword: {:?}", token_string, symbol);
                    } else {
                        println!("'{}' is a identifier.", token_string);
                    }

                    Some(symbol::symbol(
                        token_string,
                        symbol::Token::String,
                        symbol::Group::Identifier,
                    ))
                }

                _ => {
                    let token_string = self.source[self.current - 1].to_string();
                    if let Some(symbol) = SYMBOLS.get(token_string.as_str()) {
                        println!("'{}' is a operator: {:?}", token_string, symbol);
                    } else {
                        println!("'{}' is not a operator.", token_string);
                    }

                    None
                }
            }
        }
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
