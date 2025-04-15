use jsompiler_common::{Error, ErrorKind};
use num_bigint::BigInt;

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
        self.advance();
        if self.source[self.current - 1] == '0' {
            let current_index = self.current;
            if let Some(&base) = to_check_base
                .iter()
                .find(|&&c| c.0 == self.get_current_char().to_ascii_lowercase())
            {
                self.advance();
                while base
                    .1
                    .contains(&self.get_current_char().to_ascii_lowercase())
                    || (self.get_current_char() == '_'
                        && self
                            .peek_next_char()
                            .is_some_and(|ch| base.1.contains(&ch.to_ascii_lowercase())))
                {
                    self.advance();
                }
                let lexeme_slice: String = self.source[current_index - 1..self.current]
                    .iter()
                    .collect();
                return self.output_base_value(lexeme_slice, base);
            }
            self.current = current_index;
            loop {
                let curr = self.get_current_char();
                if curr != '.'
                    && curr != 'e'
                    && !curr.is_ascii_digit()
                    && self.get_current_char() != '_'
                {
                    break;
                }
                self.advance();
            }
            let lexeme_slice: String = self.source[current_index..self.current].iter().collect();
            let mut is_octal = true;
            for ch in lexeme_slice.chars() {
                if ch.is_ascii_digit() && !octal_allowed_chars.contains(&ch) {
                    is_octal = false;
                    break;
                }
            }
            if is_octal && !lexeme_slice.starts_with('.') && lexeme_slice.len() != 0 {
                return self.output_base_value(lexeme_slice, ('o', &octal_allowed_chars));
            }
            self.current = current_index;
        }
        self.lex_nobase_numbers()
    }

    fn output_base_value(
        &mut self,
        lexeme_slice: String,
        base: (char, &[char]),
    ) -> Result<Option<Lexeme>, crate::Error> {
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
        }
        let omitted = omit_underscores_from_numbers(
            &lexeme_slice
                .trim_start_matches(format!("0{}", base.0).as_str())
                .to_string(),
            false,
        );
        if omitted.is_none() {
            self.current -= 1;
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
        }
        let omitted = omitted.unwrap();
        let lexeme_f64 = u64::from_str_radix(omitted.as_str(), base.1.len() as u32);
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

    fn lex_nobase_numbers(&mut self) -> Result<Option<Lexeme>, crate::Error> {
        loop {
            let ch = self.get_current_char();
            if !ch.is_ascii_digit()
                && !(ch == '_'
                    && self
                        .peek_next_char()
                        .is_some_and(|ch| ch.is_ascii_digit() || ch == 'e'))
                && ch != 'e'
            {
                break;
            }
            self.advance();
        }
        let token_string: String = self.source[self.start..self.current].iter().collect();
        if token_string.chars().last() == Some('e') {
            return Err(Error::new(
                ErrorKind::LexerError,
                format!("Invalid number {token_string}"),
                self.line_number,
                self.current,
            ));
        }

        if self.get_current_char() == '.'
            || (self.get_current_char() == '_'
                && self.peek_next_char().is_some_and(|ch| ch.is_ascii_digit()))
        {
            self.advance();
            while self.get_current_char().is_ascii_digit()
                || (self.get_current_char() == '_'
                    && self.peek_next_char().is_some_and(|ch| ch.is_ascii_digit()))
            {
                self.advance();
            }
            let token_string: String = self.source[self.start..self.current].iter().collect();
            let omitted = omit_underscores_from_numbers(&token_string, true);
            if omitted.is_none() {
                return Err(Error::new(
                    ErrorKind::LexerError,
                    format!("Invalid number {token_string}"),
                    self.line_number,
                    self.current,
                ));
            }
            let omitted = omitted.unwrap();
            let token_num = omitted.parse::<f64>();
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

        let omitted = omit_underscores_from_numbers(&token_string, true);
        if omitted.is_none() {
            return Err(Error::new(
                ErrorKind::LexerError,
                format!("Invalid number {token_string}"),
                self.line_number,
                self.current,
            ));
        }
        let omitted = omitted.unwrap();
        let token_num = omitted.parse::<f64>();
        if !token_num.is_ok() {
            return Err(Error::new(
                ErrorKind::LexerError,
                format!("Invalid number {token_string}"),
                self.line_number,
                self.current,
            ));
        }
        let token_num = token_num.unwrap();

        // Check if bigint
        if self.get_current_char() == 'n' {
            self.advance(); // Consume 'n'
            let token_num = token_num as i32;
            return Ok(Some(lexeme(
                token_string,
                Token::Literal(LiteralToken::Number(symbol::NumberLiteral::BigInt(
                    BigInt::from(token_num),
                ))),
            )));
        }
        Ok(Some(lexeme(
            token_string,
            Token::Literal(LiteralToken::Number(symbol::NumberLiteral::Value(
                token_num,
            ))),
        )))
    }
}

fn omit_underscores_from_numbers(number_string: &String, is_decimal: bool) -> Option<String> {
    let mut new_str = "".to_string();
    let mut collect_chars = vec![];
    for (i, ch) in number_string.char_indices() {
        collect_chars.push(ch);
        if ch == '_'
            && (i == 0
                || i == number_string.chars().count() - 1
                || (is_decimal && collect_chars[i.saturating_sub(1)] == 'e'))
        {
            return None;
        }
        if is_decimal && ch == 'e' && collect_chars[i.saturating_sub(1)] == '_' {
            return None;
        }
        if ch != '_' {
            new_str.push(ch);
        }
    }
    Some(new_str)
}
