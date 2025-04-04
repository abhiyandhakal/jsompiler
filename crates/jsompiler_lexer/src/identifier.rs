use jsompiler_common::Error;

use crate::{
    Lexer,
    symbol::{Lexeme, SYMBOLS, Token, lexeme},
};

impl Lexer {
    pub fn lex_identifier(&mut self) -> Result<Option<Lexeme>, Error> {
        while self.get_current_char().is_alphanumeric()
            || self.get_current_char() == '_'
            || self.get_current_char() == '$'
        {
            self.advance();
        }
        let token_string = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        let keyword = SYMBOLS.iter().find(|f| *f.0 == token_string);
        if let Some(keyword) = keyword {
            return Ok(Some(keyword.1.clone()));
        }

        Ok(Some(lexeme(
            token_string.clone(),
            Token::Identifier(token_string),
        )))
    }
}
