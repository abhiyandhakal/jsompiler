use super::{Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, KeywordToken, Token};

impl Parser {
    pub fn parse_break_or_continue_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        if self.peek().token == Token::Keyword(KeywordToken::Break) {
            self.parse_break_statement()
        } else if self.peek().token == Token::Keyword(KeywordToken::Continue) {
            self.parse_continue_statement()
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'break' or 'continue'".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }

    fn parse_break_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        self.advance(); // Consume 'break' keyword

        let label = if let Token::Identifier(_) = self.peek().token {
            Some(self.expression()?)
        } else {
            None
        };

        // Expect semicolon
        if !self.match_token(&Token::Delimiter(DelimiterToken::Semicolon)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ';' after break statement".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(vec![Statement::BreakStatement { label }])
    }

    fn parse_continue_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        self.advance(); // Consume 'continue' keyword

        let label = if let Token::Identifier(_) = self.peek().token {
            Some(self.expression()?)
        } else {
            None
        };

        // Expect semicolon
        if !self.match_token(&Token::Delimiter(DelimiterToken::Semicolon)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ';' after continue statement".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(vec![Statement::ContinueStatement { label }])
    }
}
