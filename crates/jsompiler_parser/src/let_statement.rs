use super::expression::Expression;
use super::{Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{
    ContextualKeywordToken, DelimiterToken, KeywordToken, LiteralToken, OperatorToken, Token,
};

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn parse_let_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        if !self.match_token(&Token::ContextualKeyword(ContextualKeywordToken::Let))
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

        let mut declarations = Vec::new();
        let keyword_token = self.previous().clone(); // Store 'let', 'var', or 'const'

        loop {
            // Expect an identifier
            if let Token::Identifier(_) = &self.peek().token {
                self.advance();
                let name = self.previous().clone();

                // Check for optional assignment
                let value = if self.match_token(&Token::Operator(OperatorToken::EqualTo)) {
                    Some(Box::new(self.expression()?))
                } else {
                    None
                };

                declarations.push(Statement::LetStatement(LetStatement {
                    token: keyword_token.token.clone(),
                    name: Identifier {
                        token: name.clone(),
                        value: name.text,
                    },
                    value: value
                        .unwrap_or_else(|| Box::new(Expression::Literal(LiteralToken::Undefined))),
                }));
            } else {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected identifier after 'let'".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            // Stop if no more comma
            if !self.match_token(&Token::Delimiter(DelimiterToken::Comma)) {
                break;
            }
        }

        // Ensure a valid statement terminator
        if !self.match_token(&Token::Delimiter(DelimiterToken::Semicolon))
            && !self.match_token(&Token::Delimiter(DelimiterToken::NewLine))
        {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ';' or newline after variable declaration".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(declarations)
    }
}
