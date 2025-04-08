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
                c => {
                    return self.lex_operator_punctuation(c);
                }
            }
        }

        Ok(None)
    }
}
