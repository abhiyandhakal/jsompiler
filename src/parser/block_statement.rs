use crate::{
    Error, ErrorKind,
    lexer::symbol::{DelimiterToken, Token},
};

use super::{Parser, Statement};

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Parser {
    pub fn parse_block_statement(&mut self) -> Result<Statement, Vec<Error>> {
        self.advance();
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
                self.advance()
            };

            self.advance();
            if self.is_at_end() {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected '}'".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            let statement = self.parse_statement()?;
            statements.push(statement);
            if self.peek().token == Token::EOF {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected '}'".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
                self.advance();
            }
            if self.next().token == Token::Delimiter(DelimiterToken::CloseBrace) {
                println!("Closing block statement");
                self.advance();
                self.advance();
                break;
            }
        }
        Ok(Statement::BlockStatement(BlockStatement {
            token: Token::Delimiter(DelimiterToken::OpenBrace),
            statements,
        }))
    }
}
