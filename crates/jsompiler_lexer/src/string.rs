use jsompiler_common::{Error, ErrorKind};

use crate::{
    Lexer,
    symbol::{DelimiterToken, Lexeme, LiteralToken, Token, lexeme},
};

impl Lexer {
    pub fn lex_template_string(&mut self) -> Result<Option<Lexeme>, Error> {
        let mut processed_string = "".to_string();
        self.tokens.push(lexeme(
            "`".to_string(),
            Token::Delimiter(DelimiterToken::Tilde),
        ));

        while self.get_current_char() != '`' {
            if let Err(e) = self.process_char_in_string('`', &mut processed_string) {
                return Err(e);
            }

            if self.get_current_char() == '$'
                && (self.source.len() > self.current + 1 && self.source[self.current + 1] == '{')
            {
                self.tokens.push(lexeme(
                    processed_string.clone(),
                    Token::Literal(LiteralToken::String(processed_string.clone())),
                ));
                processed_string = "".to_string();

                let mut open_brace_count = -1;
                loop {
                    self.start = self.current;
                    let c = self.advance();
                    let op = match c {
                        '\n' => {
                            // self.advance();
                            Ok(Some(lexeme(
                                "\n".to_string(),
                                Token::Delimiter(DelimiterToken::NewLine),
                            )))
                        }
                        '0'..='9' => self.lex_number(),
                        'a'..='z' | 'A'..='Z' | '_' | '$' => self.lex_identifier(),
                        '/' => self.lex_comment(),
                        '}' => {
                            open_brace_count -= 1;
                            self.lex_operator_punctuation('}')
                        }
                        '{' => {
                            if open_brace_count != -1 {
                                open_brace_count += 1
                            } else {
                                open_brace_count = 1
                            };
                            self.lex_operator_punctuation('{')
                        }
                        c => self.lex_operator_punctuation(c),
                    };

                    match op {
                        Ok(Some(lexeme)) => self.tokens.push(lexeme),
                        Err(e) => return Err(e),
                        _ => {}
                    }

                    if open_brace_count == 0 {
                        break;
                    }
                }
            }
        }
        self.tokens.push(lexeme(
            processed_string.clone(),
            Token::Literal(LiteralToken::String(processed_string)),
        ));
        self.advance(); // consume the closing quote

        return Ok(Some(lexeme(
            "`".to_string(),
            Token::Delimiter(DelimiterToken::Tilde),
        )));
    }

    pub fn lex_string(&mut self, ch: char) -> Result<Option<Lexeme>, Error> {
        let mut processed_string = "".to_string();
        while self.get_current_char() != ch {
            if let Err(e) = self.process_char_in_string(ch, &mut processed_string) {
                return Err(e);
            }
        }
        self.advance(); // consume the closing quote
        Ok(Some(lexeme(
            self.source[self.start..self.current].iter().collect(),
            Token::Literal(LiteralToken::String(processed_string)),
        )))
    }

    fn process_char_in_string(
        &mut self,
        ch: char,
        processed_string: &mut String,
    ) -> Result<(), Error> {
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
        Ok(())
    }
}
