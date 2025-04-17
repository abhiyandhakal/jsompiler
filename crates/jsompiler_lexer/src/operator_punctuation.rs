use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, SYMBOLS, Token, lexeme},
};

impl Lexer {
    pub fn lex_operator_punctuation(&mut self, c: char) -> Result<Option<Lexeme>, Error> {
        if c == '.' {
            // Don't allow lexing of '.' after floating point number
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

            // For floating point numbers like .123
            if self.peek_next_char().is_some_and(|ch| ch.is_ascii_digit()) {
                return self.lex_number();
            }
        }

        // Handle regex
        if let Some(last_token) = self.tokens.last() {
            if let Token::Operator(_) = last_token.token {
                if self.get_current_char() == '/' {
                    // Check for empty regex
                    if self.peek_next_char() == Some('/') {
                        return Err(Error {
                            error_kind: ErrorKind::LexerError,
                            message:
                                "Empty regular expression literals are not allowed. Use /(?:)/ instead."
                                    .to_string(),
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

                        if c == '\n' || c == '\0' {
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
                        if self.is_at_end() {
                            return Err(Error {
                                error_kind: ErrorKind::LexerError,
                                message: "Regex not closed.".to_string(),
                                line_number: self.line_number,
                                pos: self.current,
                            });
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

                    if symbol.token == Token::Operator(symbol::OperatorToken::Slash) {
                        self.advance();
                    }

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
