use super::{expression::Expression, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{KeywordToken, Token};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: Box<Vec<Statement>>,
    pub alternative: Option<Box<Vec<Statement>>>,
}

impl Parser {
    pub fn parse_if_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::If)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'then' after condition".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let value = self.parenthesis_expression()?;
        let consequence = Box::new(self.parse_block_statement()?);
        // Decode expression from statement
        let expression = match value.first() {
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

        if self.match_token(&Token::Keyword(KeywordToken::Else)) {
            if self.peek().token == Token::Keyword(KeywordToken::If) {
                let alternative = Box::new(self.parse_if_statement()?);
                return Ok(vec![Statement::IfStatement(IfStatement {
                    condition: expression.clone(),
                    consequence,
                    alternative: Some(alternative),
                })]);
            }
            let alternative = Box::new(self.parse_block_statement()?);
            return Ok(vec![Statement::IfStatement(IfStatement {
                condition: expression.clone(),
                consequence,
                alternative: Some(alternative),
            })]);
        }

        Ok(vec![Statement::IfStatement(IfStatement {
            condition: expression.clone(),
            consequence,
            alternative: None,
        })])
    }
}
