pub mod symbol;
mod test;

use std::f64;

use jsompiler_common::{Error, ErrorKind};
use symbol::{DelimiterToken, Lexeme, LiteralToken, SYMBOLS, Token, lexeme};

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
            '0'..='9' => {
                // Lex hex value
                let mut hex_allowed_chars: Vec<_> = ('0'..='9').collect();
                let hex_alphabets: Vec<_> = ('a'..='f').collect();
                hex_allowed_chars.extend(hex_alphabets);

                if self.source[self.current - 1] == '0' {
                    let current_index = self.current;
                    if self.get_current_char() == 'x' {
                        self.advance();
                        while hex_allowed_chars.contains(&self.get_current_char()) {
                            self.advance();
                        }
                        let lexeme_slice: String = self.source[current_index - 1..self.current]
                            .iter()
                            .collect();
                        if self.get_current_char().is_alphanumeric() {
                            return Err(Error {
                                pos: self.current,
                                message: format!(
                                    "Invalid hex value found: {lexeme_slice}{}",
                                    self.source[self.current]
                                ),
                                error_kind: ErrorKind::LexerError,
                                line_number: self.line_number,
                            });
                        } else {
                            let lexeme_f64 =
                                u64::from_str_radix(lexeme_slice.trim_start_matches("0x"), 16);
                            if !lexeme_f64.is_ok() {
                                return Err(Error {
                                    pos: self.current,
                                    message: format!("Invalid hex value found: {lexeme_slice}"),
                                    error_kind: ErrorKind::LexerError,
                                    line_number: self.line_number,
                                });
                            }
                            let lexeme_u64 = lexeme_f64.unwrap();
                            return Ok(Some(lexeme(
                                lexeme_slice,
                                Token::Literal(LiteralToken::Number(symbol::NumberLiteral::Value(
                                    lexeme_u64 as f64,
                                ))),
                            )));
                        }
                    } else {
                        self.current = current_index;
                    }
                }

                while self.get_current_char().is_ascii_digit() {
                    self.advance();
                }
                let token_string: String = self.source[self.start..self.current].iter().collect();

                if self.get_current_char() == '.' {
                    self.advance();
                    while self.get_current_char().is_ascii_digit() {
                        self.advance();
                    }
                    let token_string: String =
                        self.source[self.start..self.current].iter().collect();
                    let token_num = token_string.parse::<f64>();
                    if !token_num.is_ok() {
                        return Err(Error::new(
                            ErrorKind::LexerError,
                            format!("Invalid number {token_string}"),
                            self.line_number,
                            self.current,
                        ));
                    }
                    let token_num = token_num.unwrap();
                    return Ok(Some(lexeme(
                        token_string,
                        Token::Literal(LiteralToken::Number(symbol::NumberLiteral::Value(
                            token_num,
                        ))),
                    )));
                }
                let token_num = token_string.parse::<f64>();
                if !token_num.is_ok() {
                    return Err(Error::new(
                        ErrorKind::LexerError,
                        format!("Invalid number {token_string}"),
                        self.line_number,
                        self.current,
                    ));
                }
                let token_num = token_num.unwrap();

                Ok(Some(lexeme(
                    token_string,
                    Token::Literal(LiteralToken::Number(symbol::NumberLiteral::Value(
                        token_num,
                    ))),
                )))
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

                let keyword = SYMBOLS.iter().find(|f| *f.0 == token_string);
                if let Some(keyword) = keyword {
                    return Ok(Some(keyword.1.clone()));
                }

                Ok(Some(lexeme(
                    token_string.clone(),
                    Token::Identifier(token_string),
                )))
            }
            '`' => {
                let mut processed_string = "".to_string();
                while self.get_current_char() != '`' {
                    if self.get_current_char() == '\0' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "Template string (`) not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }

                    if self.get_current_char() == '\\' {
                        if self.current + 1 < self.source.len() {
                            let next_char = self.source[self.current + 1];
                            match next_char {
                                '`' => {
                                    processed_string.push('`');
                                    self.advance(); // Skip `\`
                                }
                                '\\' => {
                                    processed_string.push('\\');
                                    self.advance(); // Skip `\`
                                }
                                'n' => {
                                    processed_string.push('\n');
                                    self.advance(); // Skip `\`
                                }
                                't' => {
                                    processed_string.push('\t');
                                    self.advance(); // Skip `\`
                                }
                                _ => {
                                    processed_string.push(next_char);
                                    self.advance(); // Skip `\`
                                }
                            }
                        }
                    } else {
                        processed_string.push(self.get_current_char());
                    }
                    self.advance();
                }
                self.advance(); // consume the closing quote
                Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                        processed_string,
                    ))),
                )))
            }
            '"' => {
                let mut processed_string = "".to_string();
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

                    if self.get_current_char() == '\\' {
                        if self.current + 1 < self.source.len() {
                            let next_char = self.source[self.current + 1];
                            match next_char {
                                '"' => {
                                    processed_string.push('"');
                                    self.advance(); // Skip `\`
                                }
                                '\\' => {
                                    processed_string.push('\\');
                                    self.advance(); // Skip `\`
                                }
                                'n' => {
                                    processed_string.push('\n');
                                    self.advance(); // Skip `\`
                                }
                                't' => {
                                    processed_string.push('\t');
                                    self.advance(); // Skip `\`
                                }
                                _ => {
                                    processed_string.push(next_char);
                                    self.advance(); // Skip `\`
                                }
                            }
                        }
                    } else {
                        processed_string.push(self.get_current_char());
                    }
                    self.advance();
                }
                self.advance(); // consume the closing quote
                Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Regular(
                        processed_string,
                    ))),
                )))
            }
            '\'' => {
                let mut processed_string = "".to_string();
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

                    if self.get_current_char() == '\\' {
                        if self.current + 1 < self.source.len() {
                            let next_char = self.source[self.current + 1];
                            match next_char {
                                '\'' => {
                                    processed_string.push('\'');
                                    self.advance(); // Skip `\`
                                }
                                '\\' => {
                                    processed_string.push('\\');
                                    self.advance(); // Skip `\`
                                }
                                'n' => {
                                    processed_string.push('\n');
                                    self.advance(); // Skip `\`
                                }
                                't' => {
                                    processed_string.push('\t');
                                    self.advance(); // Skip `\`
                                }
                                _ => {
                                    processed_string.push(next_char);
                                    self.advance(); // Skip `\`
                                }
                            }
                        }
                    } else {
                        processed_string.push(self.get_current_char());
                    }
                    self.advance();
                }
                self.advance(); // consume the closing quote
                Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::Literal(LiteralToken::String(symbol::StringLiteral::Regular(
                        processed_string,
                    ))),
                )))
            }
            '/' => {
                if self.start < self.source.len() {
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
                                    == *"*/"
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
                        c => {
                            let last_token = self.tokens.last();
                            let is_else;
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
                                if !self.is_beyond_end() {
                                    let mut longest_match = None;

                                    for len in (1..=3).rev() {
                                        let end_index = self.start + len;
                                        if end_index > self.source.len() {
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

                                    if longest_match.is_none() {
                                        return Err(Error {
                                            error_kind: ErrorKind::LexerError,
                                            message: format!("Unexpected character: {c}")
                                                .to_string(),
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

                Ok(None)
            }
            c => {
                // Don't allow lexing of '.' after floating point number
                if c == '.' {
                    if let Some(lexeme) = self.tokens.last() {
                        if lexeme.text.contains(".") {
                            return Err(Error {
                                pos: self.current,
                                line_number: self.line_number,
                                message: format!("Invalid number: \"{}.\"", lexeme.text),
                                error_kind: ErrorKind::LexerError,
                            });
                        }
                    }
                }

                if !self.is_beyond_end() {
                    let mut longest_match = None;

                    for len in (1..=3).rev() {
                        let end_index = self.start + len;
                        if end_index > self.source.len() {
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

                    if longest_match.is_none() {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: format!("Unexpected character: {c}").to_string(),
                            line_number: self.line_number,
                            pos: self.start,
                        });
                    } else {
                        return Ok(longest_match);
                    }
                }
                Ok(Some(symbol::Lexeme {
                    text: "EOF".to_string(),
                    len: 0,
                    token: symbol::Token::EOF,
                }))
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
        while self.get_current_char() == ' ' || self.get_current_char() == '\t' {
            self.advance();
        }
    }
}
