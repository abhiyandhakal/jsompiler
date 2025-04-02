use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, LiteralToken, Token, lexeme},
};

impl Lexer {
    pub fn lex_number(&mut self) -> Result<Option<Lexeme>, crate::Error> {
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
                    let lexeme_f64 = u64::from_str_radix(lexeme_slice.trim_start_matches("0x"), 16);
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
            let token_string: String = self.source[self.start..self.current].iter().collect();
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
}
