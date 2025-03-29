use crate::{
    Error, ErrorKind,
    lexer::symbol::{KeywordToken, Token},
};

use super::{Parser, Statement, expression::Expression};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn parse_return_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::Return)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'return'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let value = Box::new(self.expression()?);

        Ok(Some(Statement::ReturnStatement(ReturnStatement {
            token: Token::Keyword(KeywordToken::Return),
            value,
        })))
    }
}
