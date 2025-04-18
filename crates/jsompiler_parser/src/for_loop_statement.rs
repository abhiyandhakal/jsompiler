use super::{Parser, Statement};
use crate::Error;
use jsompiler_lexer::symbol::{DelimiterToken, KeywordToken, Token};

#[derive(Debug, Clone)]
pub struct ForLoopStatement {
    pub initializer: Box<Vec<Statement>>,
    pub condition: Box<Vec<Statement>>,
    pub increment: Box<Vec<Statement>>,
    pub body: Box<Vec<Statement>>,
}

impl Parser {
    pub fn parser_for_loop_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::For)) {
            return Err(vec![Error {
                error_kind: crate::ErrorKind::UnexpectedToken,
                message: "Expected 'for' keyword".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Expect '('
        if !self.match_token(&Token::Delimiter(DelimiterToken::OpenParen)) {
            return Err(vec![Error {
                error_kind: crate::ErrorKind::UnexpectedToken,
                message: "Expected '(' after 'for' keyword".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Parse initializer
        let mut initializer = vec![];
        if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
            self.advance(); // Consume ';'
        } else {
            initializer = self.parse_statement()?;
            self.advance(); // Consume ';'
        }
        println!("Initializers: {:#?}", initializer);

        // Parse condition
        let mut condition = vec![];
        if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
            self.advance(); // Consume ';'
        } else {
            condition = self.parse_statement()?;
            self.advance(); // Consume ';'
        }
        println!("Condition: {:#?}", condition);

        // Parse increment
        let mut increment = vec![];
        println!("Peek: {:#?}", self.peek());
        if self.peek().token != Token::Delimiter(DelimiterToken::CloseParen) {
            increment = self.parse_statement()?;
        }
        println!("Increment: {:#?}", increment);

        // Expect ')'
        if !self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
            return Err(vec![Error {
                error_kind: crate::ErrorKind::UnexpectedToken,
                message: "Expected ')' after for loop condition".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Parse body
        let body;
        if self.peek().token == Token::Delimiter(DelimiterToken::OpenBrace) {
            body = self.parse_block_statement()?;
        } else {
            return Err(vec![Error {
                error_kind: crate::ErrorKind::UnexpectedToken,
                message: "Expected '{' ".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(vec![Statement::ForLoopStatement(ForLoopStatement {
            initializer: Box::new(initializer),
            condition: Box::new(condition),
            increment: Box::new(increment),
            body: Box::new(body),
        })])
    }
}
