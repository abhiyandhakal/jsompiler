use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, SYMBOLS, Token, lexeme},
};

impl Lexer {
    pub fn lex_comment(&mut self) -> Result<Option<Lexeme>, Error> {
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
                                    message: format!("Unexpected character: {c}").to_string(),
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
}
