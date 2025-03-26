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
        println!("Parsing block statement");
        if !self.match_token(&Token::Delimiter(DelimiterToken::OpenBrace)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '{'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        println!("{:?}", self.peek().token);
        self.advance();
        println!("{:?}", self.peek().token);
        let mut statements = Vec::new();
        let statement = self.parse_statement()?;
        println!("Parsed statement: {:?}", statement);
        statements.push(statement);
        Ok(Statement::BlockStatement(BlockStatement {
            token: Token::Delimiter(DelimiterToken::OpenBrace),
            statements,
        }))
    }
}
