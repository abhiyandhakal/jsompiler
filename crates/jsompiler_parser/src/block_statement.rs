use super::{Parser, Statement};
use jsompiler_common::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, Token};

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Option<Statement>>,
}

impl Parser {
    pub fn parse_block_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
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
        Ok(Some(Statement::BlockStatement(BlockStatement {
            token: Token::Delimiter(DelimiterToken::OpenBrace),
            statements,
        })))
    }
}
