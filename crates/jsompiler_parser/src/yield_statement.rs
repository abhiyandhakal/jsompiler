use super::{expression::Expression, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, Token};

#[derive(Debug, Clone)]
pub struct YieldStatement {
    pub expression: Option<Expression>,
}

impl Parser {
    pub fn parse_yield_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        self.advance(); // Consume 'yield' keyword

        let expression = if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        // Expect semicolon
        if !self.match_token(&Token::Delimiter(DelimiterToken::Semicolon)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ';' after yield statement".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(Some(Statement::YieldStatement(YieldStatement {
            expression,
        })))
    }
}
