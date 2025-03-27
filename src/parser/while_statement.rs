use super::{Parser, Statement, expression::Expression};
use crate::lexer::symbol::{KeywordToken, Token};
use crate::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct WhileStatement {
    condition: Expression,
    consequence: Box<Statement>,
}

impl Parser {
    pub fn parse_while_statement(&mut self) -> Result<Statement, Vec<Error>> {
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
            Statement::ExpressionStatement(expr) => expr,
            _ => {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected expression".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
        };

        Ok(Statement::WhileStatement(WhileStatement {
            condition: expression,
            consequence,
        }))
    }
}
