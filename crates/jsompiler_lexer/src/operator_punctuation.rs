use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, SYMBOLS, Token, lexeme},
};

impl Lexer {
    pub fn lex_operator_punctuation(&mut self, c: char) -> Result<Option<Lexeme>, Error> {
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

        // Handle regex
        if let Some(last_token) = self.tokens.last() {
            if let Token::Operator(_) = last_token.token {
                // Check for empty regex
                if self.current + 1 < self.source.len() && self.source[self.current + 1] == '/' {
                    return Err(Error {
                        error_kind: ErrorKind::LexerError,
                        message: "Empty regular expression literals are not allowed. Use /(?:)/ instead.".to_string(),
                        line_number: self.line_number,
                        pos: self.current,
                    });
                }

                let mut in_class: bool = false;
                let mut escaped = false;

                // Parse regex body
                loop {
                    self.advance();
                    let c = self.get_current_char();
                    
                    if c == '\0' || c == '\n' {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message: "Regex not closed.".to_string(),
                            line_number: self.line_number,
                            pos: self.current,
                        });
                    }

                    // Handle first character restrictions
                    if self.current == self.start + 1 {
                        if c == '*' || c == '/' {
                            return Err(Error {
                                error_kind: ErrorKind::LexerError,
                                message: format!("Invalid first character in regex: '{}'", c),
                                line_number: self.line_number,
                                pos: self.current,
                            });
                        }
                    }

                    if escaped {
                        escaped = false;
                    } else {
                        match c {
                            '\\' => escaped = true,
                            '[' if !in_class => in_class = true,
                            ']' if in_class => in_class = false,
                            '/' if !in_class => break,
                            _ => {}
                        }
                    }
                }

                let pattern = self.source[self.start + 1..self.current]
                    .iter()
                    .collect::<String>();

                // Parse flags
                let flags_start = self.current;
                self.advance();
                
                loop {
                    let c = self.get_current_char();
                    if !c.is_alphabetic() {
                        break;
                    }
                    self.advance();
                }

                let flags = self.source[flags_start + 1..self.current]
                    .iter()
                    .collect::<String>();

                return Ok(Some(lexeme(
                    self.source[self.start..self.current].iter().collect(),
                    Token::RegExp { pattern, flags },
                )));
            }
        }

        if !self.is_beyond_end() {
            let mut longest_match = None;

            for len in (1..=3).rev() {
                let end_index = self.start + len;
                if end_index > self.source.len() {
                    continue;
                }
                let lexeme_slice: String = self.source[self.start..end_index].iter().collect();

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
