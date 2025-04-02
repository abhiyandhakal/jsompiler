use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, LiteralToken, Token, lexeme},
};

impl Lexer {
    pub fn lex_string(&mut self, ch: char) -> Result<Option<Lexeme>, Error> {
        let mut processed_string = "".to_string();
        while self.get_current_char() != ch {
            if self.get_current_char() == '\0' {
                return Err(Error {
                    error_kind: ErrorKind::LexerError,
                    message: format!("String ({ch}) not closed."),
                    line_number: self.line_number,
                    pos: self.current,
                });
            }

            if ch != '`' && self.get_current_char() == '\n' {
                return Err(Error {
                    error_kind: ErrorKind::LexerError,
                    message: format!("String ({ch}) not closed."),
                    line_number: self.line_number,
                    pos: self.current,
                });
            }

            if self.get_current_char() == '\\' {
                if self.current + 1 < self.source.len() {
                    let next_char = self.source[self.current + 1];
                    match next_char {
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
                        c => {
                            if c == ch {
                                processed_string.push(ch);
                                self.advance(); // Skip ch
                            } else {
                                processed_string.push(next_char);
                                self.advance(); // Skip `\`
                            }
                        }
                    }
                }
            } else {
                processed_string.push(self.get_current_char());
            }
            self.advance();
        }
        self.advance(); // consume the closing quote
        if ch == '`' {
            return Ok(Some(lexeme(
                self.source[self.start..self.current].iter().collect(),
                Token::Literal(LiteralToken::String(symbol::StringLiteral::Template(
                    processed_string,
                ))),
            )));
        }
        Ok(Some(lexeme(
            self.source[self.start..self.current].iter().collect(),
            Token::Literal(LiteralToken::String(symbol::StringLiteral::Regular(
                processed_string,
            ))),
        )))
    }
}
