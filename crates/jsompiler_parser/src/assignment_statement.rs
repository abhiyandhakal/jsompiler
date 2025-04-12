use super::{expression::Expression, Parser, Statement};
use jsompiler_common::{Error, ErrorKind};
use jsompiler_lexer::symbol::{OperatorToken, Token};

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub target: Expression,
    pub value: Expression,
}

impl Parser {
    pub fn parse_assignment_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        let target = self.expression()?;

        // check for an assignment
        if self.match_token(&Token::Operator(OperatorToken::EqualTo)) {
            match &target {
                Expression::Identifier(_)
                | Expression::MemberAccess {
                    object: _,
                    property: _,
                } => {
                    let value = self.expression()?;
                    return Ok(Some(Statement::AssignmentStatement(AssignmentStatement {
                        target,
                        value,
                    })));
                }
                _ => {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Invalid left-hand side in assignment".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
            }
        }

        // if we don't have an assignment, we can treat it as an expression statement
        match self.peek().token {
            Token::Identifier(_) | Token::Literal(_) => {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Missing =".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            _ => {}
        }

        Ok(Some(Statement::ExpressionStatement(target)))
    }
}
