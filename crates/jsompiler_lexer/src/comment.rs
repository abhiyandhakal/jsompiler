use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, Token, lexeme},
};

impl Lexer {
    pub fn lex_comment(&mut self) -> Result<Option<Lexeme>, Error> {
        if self.start + 1 < self.source.len() {
            let next_char = self.source[self.start + 1];
            match next_char {
                // Single Line Comment
                '/' => {
                    loop {
                        if self.get_current_char() == '\n' || self.is_at_end() {
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
                    if self.is_at_end() {
                        return Err(Error {
                            pos: self.current,
                            line_number: self.line_number,
                            message: "Block comment (/*) not closed.".to_string(),
                            error_kind: ErrorKind::LexerError,
                        });
                    }

                    self.advance();

                    if self.get_current_char() == '*' && self.peek_next_char() == Some('/') {
                        self.advance(); // consume '*'
                        self.advance(); // consume '/'

                        if self.start + 2 > self.current - 2 {
                            return Err(Error {
                                pos: self.current,
                                line_number: self.line_number,
                                message: "Block comment (/*) not closed.".to_string(),
                                error_kind: ErrorKind::LexerError,
                            });
                        }

                        return Ok(Some(lexeme(
                            self.source[self.start..self.current].iter().collect(),
                            Token::Comment(symbol::CommentToken::Block(
                                self.source[self.start + 2..self.current - 2]
                                    .iter()
                                    .collect(),
                            )),
                        )));
                    }
                },
                // Regex and other operators
                _ => {
                    return self.lex_operator_punctuation('/');
                }
            }
        }
        return self.lex_operator_punctuation('/');
    }

    pub fn lex_hashbang(&mut self) -> Result<Option<Lexeme>, Error> {
        if self.peek_next_char() != Some('!') {
            return Err(Error {
                pos: self.current,
                line_number: self.line_number,
                message: "Invalid character #".to_string(),
                error_kind: ErrorKind::LexerError,
            });
        }
        self.advance();
        let start = self.current + 1;
        loop {
            let ch = self.get_current_char();
            if ch == '\n' || ch == '\0' {
                break;
            }
            self.advance();
        }

        Ok(Some(lexeme(
            self.source[self.start..self.current].iter().collect(),
            Token::Comment(symbol::CommentToken::HashBang(
                self.source[start..self.current].iter().collect(),
            )),
        )))
    }
}
