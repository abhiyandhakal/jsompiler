use super::{expression::Expression, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{KeywordToken, Token};

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub consequence: Box<Option<Statement>>,
}

impl Parser {
    pub fn parse_while_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::While)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'then' after condition".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        let value = self.parenthesis_expression()?;
        println!("{:?}", self.peek().token);
        let consequence = Box::new(self.parse_block_statement()?);
        // Decode expression from statement
        let expression = match value {
            Some(Statement::ExpressionStatement(expr)) => expr,
            _ => {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected expression".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
        };

        Ok(Some(Statement::WhileStatement(WhileStatement {
            condition: expression,
            consequence,
        })))
    }
}
