use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, Token, lexeme},
};

impl Lexer {
    pub fn lex_comment(&mut self) -> Result<Option<Lexeme>, Error> {
        if self.start < self.source.len() {
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
                    return self.lex_operator_punctuation(c);
                }
            }
        }

        Ok(None)
    }
}
