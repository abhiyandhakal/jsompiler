pub mod symbol;
pub mod test;

use jsompiler_common::{Error, ErrorKind};
use symbol::{JSX_SYMBOLS, JSXToken};

use crate::{
    Lexer,
    symbol::{Lexeme, Token, lexeme},
};

impl Lexer {
    pub fn lex_jsx(&mut self) -> Result<Option<Lexeme>, Error> {
        let mut is_jsx = false;
        // if let Some(last_token) = self.tokens.last() {
        match self.tokens.last() {
            Some(last_token) => match last_token.token {
                Token::Operator(_) => is_jsx = true,
                Token::Delimiter(_) => is_jsx = true,
                _ => {}
            },
            None => is_jsx = true,
        }
        if !is_jsx {
            return self.lex_operator_punctuation('<');
        }

        let mut element_count = -1;

        loop {
            let op: Result<Option<Lexeme>, Error> = match self.get_current_char() {
                'A'..='Z' | 'a'..='z' | '_' | '$' => self.lex_jsx_identifier(),
                _ => {
                    let res = self.lex_jsx_operator_punctuation();

                    if let Ok(Some(token)) = &res {
                        let token = &token.token;
                        if token == &Token::JSX(JSXToken::LessThanSlash)
                            || token == &Token::JSX(JSXToken::SlashGreaterThan)
                        {
                            element_count -= 1;
                        }

                        if token == &Token::JSX(JSXToken::LessThan) {
                            if element_count < 0 {
                                element_count = 1;
                            } else {
                                element_count += 1;
                            }
                        }
                    }

                    res
                }
            };

            let mut token = None;
            if let Ok(Some(op)) = op {
                token = Some(op.clone().token);
                self.tokens.push(op);
            }

            if (element_count == 0
                && (token == Some(Token::JSX(JSXToken::GreaterThan))
                    || token == Some(Token::JSX(JSXToken::SlashGreaterThan))))
                || token == Some(Token::EOF)
            {
                break;
            }

            self.advance();
        }

        self.advance();
        Ok(None)
    }

    fn lex_jsx_operator_punctuation(&mut self) -> Result<Option<Lexeme>, Error> {
        if self.is_beyond_end() {
            return Ok(Some(Lexeme {
                text: "EOF".to_string(),
                len: 0,
                token: Token::EOF,
            }));
        }
        let mut longest_match = None;
        let start = self.current;

        for len in (1..=3).rev() {
            let end_index = start + len;
            if end_index > self.source.len() {
                continue;
            }
            let lexeme_slice: String = self.source[start..end_index].iter().collect();

            if let Some(symbol) = JSX_SYMBOLS.get(lexeme_slice.as_str()) {
                longest_match = Some(symbol.clone());
                self.current += len - 1;
                break;
            }
        }

        if longest_match.is_none() {
            return Err(Error {
                error_kind: ErrorKind::LexerError,
                message: format!("Unexpected character: {}", self.get_current_char()).to_string(),
                line_number: self.line_number,
                pos: start,
            });
        } else {
            return Ok(longest_match);
        }
    }

    fn lex_jsx_identifier(&mut self) -> Result<Option<Lexeme>, Error> {
        let start_index = self.current;
        loop {
            let ch = self.get_current_char();
            if !ch.is_alphanumeric() && ch != '_' && ch != '$' {
                self.current -= 1;
                break;
            }
            self.advance();
        }
        let token_string = self.source[start_index..=self.current]
            .iter()
            .collect::<String>();

        Ok(Some(lexeme(
            token_string.clone(),
            Token::JSX(JSXToken::Identifier(token_string)),
        )))
    }
}
