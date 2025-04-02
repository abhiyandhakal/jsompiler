use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{self, Lexeme, SYMBOLS},
};

impl Lexer {
    pub fn lex_operator_punctuation(&mut self, c: char) -> Result<Option<Lexeme>, Error> {
        // Don't allow lexing of '.' after floating point number
        if c == '.' {
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
