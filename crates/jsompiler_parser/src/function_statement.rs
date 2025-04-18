use super::{Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, KeywordToken, OperatorToken, Token};

#[derive(Clone, Debug)]
pub struct FunctionStatement {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: Box<Vec<Statement>>,
}

#[derive(Clone, Debug)]
pub enum Parameter {
    Identifier(Identifier),
    RestParameter(Identifier),
}

impl Parser {
    pub fn parse_function_statement(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
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
        Ok(vec![Statement::FunctionStatement(FunctionStatement {
            name,
            parameters,
            body: Box::new(body),
        })])
    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, Vec<Error>> {
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
        let mut rest_param_seen = false;

        loop {
            println!("Current token: {:#?}", self.peek().token);
            match self.peek().token {
                Token::Identifier(_) => {
                    self.advance();
                    parameters.push(Parameter::Identifier(Identifier {
                        token: self.previous().clone(),
                        value: self.previous().text.clone(),
                    }));

                    if self.peek().token == Token::Delimiter(DelimiterToken::Comma) {
                        self.advance(); // Consume the comma
                    } else if self.peek().token == Token::Delimiter(DelimiterToken::CloseParen) {
                        break; // Stop if we reach the closing parenthesis
                    } else {
                        return Err(vec![Error {
                            error_kind: ErrorKind::UnexpectedToken,
                            message: "Expected ',' or ')' after parameter".to_string(),
                            line_number: 1,
                            pos: 2,
                        }]);
                    }
                }

                Token::Operator(OperatorToken::Spread) => {
                    self.advance();
                    // Rest parameter must be the last parameter
                    if rest_param_seen {
                        return Err(vec![Error {
                            error_kind: ErrorKind::SyntaxError,
                            message: "Only one rest parameter is allowed".to_string(),
                            line_number: 1,
                            pos: 2,
                        }]);
                    }
                    if let Token::Identifier(_) = self.peek().token {
                        self.advance();
                        rest_param_seen = true;
                        parameters.push(Parameter::RestParameter(Identifier {
                            token: self.previous().clone(),
                            value: self.previous().text.clone(),
                        }));

                        // If we've seen a rest parameter, no more parameters are allowed
                        if rest_param_seen
                            && self.peek().token != Token::Delimiter(DelimiterToken::CloseParen)
                        {
                            return Err(vec![Error {
                                error_kind: ErrorKind::SyntaxError,
                                message: "Rest parameter must be the last parameter".to_string(),
                                line_number: 1,
                                pos: 2,
                            }]);
                        }
                    } else {
                        return Err(vec![Error {
                            error_kind: ErrorKind::UnexpectedToken,
                            message: "Expected identifier after '...'".to_string(),
                            line_number: 1,
                            pos: 2,
                        }]);
                    }
                }

                Token::Delimiter(DelimiterToken::CloseParen) => {
                    break; // Stop if we reach the closing parenthesis
                }

                _ => {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Unexpected parameter in function".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
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
