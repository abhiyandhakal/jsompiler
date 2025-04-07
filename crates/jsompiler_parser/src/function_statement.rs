use super::{Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, KeywordToken, Token};

#[derive(Clone, Debug)]
pub struct FunctionStatement {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: Box<Option<Statement>>,
}
impl Parser {
    pub fn parse_function_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        // Expect `function` keyword
        if !self.match_token(&Token::Keyword(KeywordToken::Function)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'function' keyword".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Expect function name (identifier)
        let name = if let Token::Identifier(_) = self.peek().token {
            self.advance();
            Identifier {
                token: self.previous().clone(),
                value: self.previous().text.clone(),
            }
        } else {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected function name".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        };

        let parameters = self.parse_function_parameters()?;

        // Expect '{' (Start of function body)
        if !self.match_token(&Token::Delimiter(DelimiterToken::OpenBrace)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '{' before function body".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Parse function body (a block statement)
        let body = self.parse_block_statement()?;

        // Return FunctionStatement node
        Ok(Some(Statement::FunctionStatement(FunctionStatement {
            name,
            parameters,
            body: Box::new(body),
        })))
    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, Vec<Error>> {
        // Expect '('
        if !self.match_token(&Token::Delimiter(
            jsompiler_lexer::symbol::DelimiterToken::OpenParen,
        )) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '(' after function name".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        // Parse parameters
        let mut parameters = Vec::new();
        while let Token::Identifier(_) = self.peek().token {
            self.advance();
            parameters.push(Identifier {
                token: self.previous().clone(),
                value: self.previous().text.clone(),
            });

            if !self.match_token(&Token::Delimiter(DelimiterToken::Comma)) {
                break; // Stop if there's no comma
            }
        }

        // Expect ')'
        if !self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ')' after parameters".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(parameters)
    }
}
