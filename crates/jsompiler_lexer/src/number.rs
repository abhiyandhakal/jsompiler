use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, LiteralToken, Token, lexeme},
};

impl Lexer {
    pub fn lex_number(&mut self) -> Result<Option<Lexeme>, crate::Error> {
        // Lex value with base (hex, octal, binary)
        let hex_allowed_chars = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let octal_allowed_chars = ['0', '1', '2', '3', '4', '5', '6', '7'];
        let binary_allowed_chars = ['0', '1'];

        let to_check_base: [(char, &[char]); 3] = [
            ('o', &octal_allowed_chars),
            ('b', &binary_allowed_chars),
            ('x', &hex_allowed_chars),
        ];

        if self.source[self.current - 1] == '0' {
            let current_index = self.current;
            // if self.get_current_char().to_ascii_lowercase() == 'x' {
            if let Some(&base) = to_check_base
                .iter()
                .find(|&&c| c.0 == self.get_current_char().to_ascii_lowercase())
            {
                self.advance();
                while base
                    .1
                    .contains(&self.get_current_char().to_ascii_lowercase())
                {
                    self.advance();
                }
                let lexeme_slice: String = self.source[current_index - 1..self.current]
                    .iter()
                    .collect();
                if self.get_current_char().is_alphanumeric() {
                    return Err(Error {
                        pos: self.current,
                        message: format!(
                            "Invalid {} value found: {}{}",
                            match base.0 {
                                'o' => "octal",
                                'b' => "binary",
                                'x' => "hex",
                                _ => unreachable!(),
                            },
                            lexeme_slice,
                            self.source[self.current]
                        ),
                        error_kind: ErrorKind::LexerError,
                        line_number: self.line_number,
                    });
                } else {
                    let lexeme_f64 = u64::from_str_radix(
                        lexeme_slice.trim_start_matches(format!("0{}", base.0).as_str()),
                        base.1.len() as u32,
                    );
                    if !lexeme_f64.is_ok() {
                        return Err(Error {
                            pos: self.current,
                            message: format!(
                                "Invalid {} value found: {}",
                                match base.0 {
                                    'o' => "octal",
                                    'b' => "binary",
                                    'x' => "hex",
                                    _ => unreachable!(),
                                },
                                lexeme_slice
                            ),
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
