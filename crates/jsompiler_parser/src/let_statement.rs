use super::expression::Expression;
use super::{Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{KeywordToken, OperatorToken, Token};

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn parse_let_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        if !self.match_token(&Token::Keyword(KeywordToken::Let))
            && !self.match_token(&Token::Keyword(KeywordToken::Var))
            && !self.match_token(&Token::Keyword(KeywordToken::Const))
        {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'let', 'var' or 'const'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        if let Token::Identifier(_) = &self.peek().token {
            let token = self.previous().clone();
            self.advance();
            let name = self.previous().clone();

            if !self.match_token(&Token::Operator(OperatorToken::EqualTo)) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected '=' after variable name".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            let value = Box::new(self.expression()?);

            Ok(Some(Statement::LetStatement(LetStatement {
                token: token.token,
                name: Identifier {
                    token: name.clone(),
                    value: name.text,
                },
                value,
            })))
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected identifier after 'let'".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }
}
