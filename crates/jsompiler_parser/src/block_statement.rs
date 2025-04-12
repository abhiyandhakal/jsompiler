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
        self.advance(); // consume '{'
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match &self.peek().token {
                Token::Delimiter(DelimiterToken::CloseBrace) => {
                    self.advance(); // consume '}'
                    break;
                }
                Token::Delimiter(DelimiterToken::Semicolon)
                | Token::Delimiter(DelimiterToken::NewLine) => {
                    self.advance();
                }
                Token::EOF => {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected '}'".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
                _ => {
                    let stmt = self.parse_statement()?;
                    statements.push(stmt);
                }
            }
        }

        Ok(Some(Statement::BlockStatement(BlockStatement {
            token: Token::Delimiter(DelimiterToken::OpenBrace),
            statements,
        })))
    }
}
