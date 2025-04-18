use super::{expression::Expression, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{KeywordToken, Token};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn parse_return_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::Return)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'return'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let value = Box::new(self.expression()?);

        Ok(vec![Statement::ReturnStatement(ReturnStatement {
            token: Token::Keyword(KeywordToken::Return),
            value,
        })])
    }
}
