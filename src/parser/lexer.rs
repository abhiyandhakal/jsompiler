use super::symbol::{DelimiterToken, Lexeme, LiteralToken, SYMBOLS, Token, lexeme};
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

    fn scan_token(&mut self) -> Result<Option<Lexeme>, crate::Error> {
        if self.is_at_end() {
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
                return Ok(Some(lexeme(
                    "\n".to_string(),
                    Token::Delimiter(DelimiterToken::NewLine),
                )));
            }
            // Numbers
            '0'..='9' => {
                while self.get_current_char().is_ascii_digit() {
                    self.advance();
                }
                let token_string: String = self.source[self.start..self.current].iter().collect();

                return Ok(Some(lexeme(
                    token_string.clone(),
                    Token::Literal(LiteralToken::Number(token_string)),
                )));
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
                    return Ok(Some(keyword.1.clone()));
                }

                return Ok(Some(lexeme(
                    token_string.clone(),
                    Token::Identifier(token_string),
                )));
            }
            '`' => {
                while self.get_current_char() != '`' {
                    if self.get_current_char() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "Template string (`) not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }
                    self.advance();
                }
                self.advance(); // consume the closing quote
                return Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                        self.source[self.start + 1..self.current - 1]
                            .iter()
                            .collect(),
                    ))),
                )));
            }
            '"' => {
                while self.get_current_char() != '"' {
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
                    self.advance();
                }
                self.advance(); // consume the closing quote
                return Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Regular(
                        self.source[self.start + 1..self.current - 1]
                            .iter()
                            .collect(),
                    ))),
                )));
            }
            '\'' => {
                while self.get_current_char() != '\'' {
                    if self.get_current_char() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "String (') not closed.".to_string(),
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
                    self.advance();
                }
                self.advance(); // consume the closing quote
                return Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Regular(
                        self.source[self.start + 1..self.current - 1]
                            .iter()
                            .collect(),
                    ))),
                )));
            }
            '/' => {
                if self.start + 1 <= self.source.len() {
                    let next_char = self.source[self.start + 1];
                    match next_char {
                        // Single Line Comment
                        '/' => {
                            loop {
                                if self.get_current_char() == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                            return Ok(Some(lexeme(
                                self.source[self.start..self.current].iter().collect(),
                                Token::Comment(symbol::CommentToken::Line(
                                    self.source[self.start + 2..self.current].iter().collect(),
                                )),
                            )));
                        }
                        // Block Comment
                        '*' => loop {
                            if self.get_current_char() == '\0' {
                                return Err(Error {
                                    error_kind: ErrorKind::LexerError,
                                    message: "Block comment (/*) not closed.".to_string(),
                                    line_number: self.line_number,
                                    pos: self.current,
                                });
                            }

                            if self.current + 1 < self.source.len() {
                                if self.source[self.current..=self.current + 1]
                                    .iter()
                                    .collect::<String>()
                                    == "*/".to_string()
                                {
                                    self.advance();
                                    self.advance();
                                    return Ok(Some(lexeme(
                                        self.source[self.start..self.current].iter().collect(),
                                        Token::Comment(symbol::CommentToken::Block(
                                            self.source[self.start + 2..self.current - 2]
                                                .iter()
                                                .collect(),
                                        )),
                                    )));
                                }
                            } else {
                                return Err(Error {
                                    error_kind: ErrorKind::LexerError,
                                    message: "Block comment (/*) not closed.".to_string(),
                                    line_number: self.line_number,
                                    pos: self.current,
                                });
                            }

                            self.advance();
                        },
                        // Regex and other operators
                        _ => {
                            let last_token = self.tokens.last();
                            let mut is_else = false;
                            if let Some(last_token) = last_token {
                                if let Token::Operator(_) = last_token.token {
                                    loop {
                                        if self.get_current_char() == '\0'
                                            || self.get_current_char() == '\n'
                                        {
                                            return Err(Error {
                                                error_kind: ErrorKind::LexerError,
                                                message: "Regex not closed.".to_string(),
                                                line_number: self.line_number,
                                                pos: self.current,
                                            });
                                        }

                                        if self.get_current_char() == '/' {
                                            break;
                                        }
                                        self.advance();
                                    }
                                    let pattern = self.source[self.start + 1..self.current]
                                        .iter()
                                        .collect::<String>();
                                    let flags_start = self.current;
                                    self.advance();
                                    loop {
                                        if self.get_current_char().is_alphabetic() {
                                            self.advance();
                                        } else {
                                            break;
                                        }
                                    }
                                    let flags = self.source[flags_start + 1..self.current]
                                        .iter()
                                        .collect::<String>();
                                    return Ok(Some(lexeme(
                                        self.source[self.start..self.current].iter().collect(),
                                        Token::RegExp { pattern, flags },
                                    )));
                                } else {
                                    is_else = true;
                                }
                            } else {
                                is_else = true;
                            }
                            if is_else {
                                if !self.is_at_end() {
                                    let mut longest_match = None;

                                    for len in (1..=3).rev() {
                                        let end_index = self.start + len;
                                        if end_index >= self.source.len() {
                                            continue;
                                        }
                                        let lexeme_slice: String =
                                            self.source[self.start..end_index].iter().collect();

                                        if let Some(symbol) = SYMBOLS.get(lexeme_slice.as_str()) {
                                            longest_match = Some(symbol.clone());
                                            self.current += len - 1;
                                            break;
                                        }
                                    }

                                    if let None = longest_match {
                                        return Err(Error {
                                            error_kind: ErrorKind::LexerError,
                                            message: "Unexpected character".to_string(),
                                            line_number: self.line_number,
                                            pos: self.start,
                                        });
                                    } else {
                                        return Ok(longest_match);
                                    }
                                }
                                return Ok(Some(symbol::Lexeme {
                                    text: "EOF".to_string(),
                                    len: 0,
                                    token: symbol::Token::EOF,
                                }));
                            }
                        }
                    }
                }

                return Ok(None);
            }
            _ => {
                if !self.is_at_end() {
                    let mut longest_match = None;

                    for len in (1..=3).rev() {
                        let end_index = self.start + len;
                        if end_index >= self.source.len() {
                            continue;
                        }
                        let lexeme_slice: String =
                            self.source[self.start..end_index].iter().collect();

                        if let Some(symbol) = SYMBOLS.get(lexeme_slice.as_str()) {
                            longest_match = Some(symbol.clone());
                            self.current += len - 1;
                            break;
                        }
                    }

                    if let None = longest_match {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "Unexpected character".to_string(),
                            line_number: self.line_number,
                            pos: self.start,
                        });
                    } else {
                        return Ok(longest_match);
                    }
                }
                return Ok(Some(symbol::Lexeme {
                    text: "EOF".to_string(),
                    len: 0,
                    token: symbol::Token::EOF,
                }));
            }
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
        while self.get_current_char() == ' ' {
            self.advance();
        }
    }
}
