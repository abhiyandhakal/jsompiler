use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, LiteralToken, Token, lexeme},
};

impl Lexer {
    pub fn lex_backtick_string(&mut self) -> Result<Option<Lexeme>, Error> {
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

    pub fn lex_doublequote_string(&mut self) -> Result<Option<Lexeme>, Error> {
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

    pub fn lex_singlequote_string(&mut self) -> Result<Option<Lexeme>, Error> {
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
}
