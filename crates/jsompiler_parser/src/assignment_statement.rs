use crate::{
    Error, ErrorKind,
    lexer::symbol::{Lexeme, OperatorToken, Token},
};

use super::{Parser, Statement, expression::Expression};

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub token: Lexeme,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn parse_assignment_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        let identifier = self.peek().clone();
        self.advance(); // Consume the identifier

        if !self.match_token(&Token::Operator(OperatorToken::EqualTo)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '='".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let value = Box::new(self.expression()?);

        Ok(Some(Statement::AssignmentStatement(AssignmentStatement {
            token: identifier,
            value,
        })))
    }
}
