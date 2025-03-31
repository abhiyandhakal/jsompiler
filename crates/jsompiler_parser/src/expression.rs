use super::{Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, Lexeme, LiteralToken, OperatorToken, Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(LiteralToken),
    Unary {
        op: Lexeme,
        op_type: String,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: Lexeme,
        right: Box<Expression>,
    },
    FunctionCall {
        name: Identifier,
        args: Vec<Expression>,
    },
}

impl Parser {
    pub fn parenthesis_expression(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        // (5+4)
        self.advance(); // Consuming open parenthesis
        let exp = self.parse_expression();
        if self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
            exp
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ')'".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }
    pub fn parse_expression(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        if let Ok(expr) = self.expression() {
            // Push statement to AST
            // self.ast
            //     .push(parser::Statement::ExpressionStatement(expr.clone()));
            Ok(Some(Statement::ExpressionStatement(expr)))
        } else {
            self.errors.push(Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected expression".to_string(),
                line_number: 1,
                pos: 2,
            });
            Err(self.errors.clone())
        }
    }

    pub fn expression(&mut self) -> Result<Expression, Vec<Error>> {
        self.comparison() // Start from highest precedence binary operations
    }

    fn unary(&mut self) -> Result<Expression, Vec<Error>> {
        if self.match_token(&Token::Operator(OperatorToken::Minus))
            || self.match_token(&Token::Operator(OperatorToken::Not))
            || self.match_token(&Token::Operator(OperatorToken::Plus))
            || self.match_token(&Token::Operator(OperatorToken::Increment))
            || self.match_token(&Token::Operator(OperatorToken::Decrement))
        {
            let op = self.previous().clone();
            let expr = self.unary()?;
            Ok(Expression::Unary {
                op,
                op_type: "Prefix".to_string(),
                expr: Box::new(expr),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Vec<Error>> {
        if self.match_token(&Token::Delimiter(DelimiterToken::OpenParen)) {
            let expr = self.expression()?; // Parse inner expression

            if !self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected ')' after expression".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            return Ok(expr); // Return the inner expression
        }

        if let Token::Identifier(_) = self.peek().token {
            self.advance();
            // Check if it's a function call
            if self.peek().token == Token::Delimiter(DelimiterToken::OpenParen) {
                return self.parse_function_call(self.previous().clone());
            }

            // Check for postfix increment/decrement
            if self.match_token(&Token::Operator(OperatorToken::Increment))
                || self.match_token(&Token::Operator(OperatorToken::Decrement))
            {
                return Ok(Expression::Unary {
                    op: self.previous().clone(),
                    op_type: "Postfix".to_string(),
                    expr: Box::new(Expression::Identifier(Identifier {
                        token: self.previous().clone(),
                        value: self.previous().text.clone(),
                    })),
                });
            }

            let identifier = self.previous().clone();
            return Ok(Expression::Identifier(Identifier {
                token: identifier.clone(),
                value: identifier.text.clone(),
            }));
        }

        if let Some(literal) = self.match_literal() {
            return Ok(Expression::Literal(literal));
        }

        Err(vec![Error {
            error_kind: ErrorKind::UnexpectedToken,
            message: "Expected expression".to_string(),
            line_number: 1,
            pos: 2,
        }])
    }

    fn factor(&mut self) -> Result<Expression, Vec<Error>> {
        let mut left = self.unary()?; // Parse unary first

        while let Some(op) = self.match_operator(&[OperatorToken::Asterisk, OperatorToken::Slash]) {
            let right = self.unary()?; // Parse the second operand
            left = Expression::Binary {
                left: Box::new(left),
                op: Lexeme {
                    token: Token::Operator(op.clone()),
                    text: op.to_string(),
                    len: 1,
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, Vec<Error>> {
        let mut left = self.factor()?; // Parse the first operand

        while let Some(op) = self.match_operator(&[OperatorToken::Plus, OperatorToken::Minus]) {
            let right = self.factor()?; // Parse the second operand
            left = Expression::Binary {
                left: Box::new(left),
                op: Lexeme {
                    token: Token::Operator(op.clone()),
                    text: op.to_string(),
                    len: 1,
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, Vec<Error>> {
        let mut left = self.term()?; // Parse arithmetic first

        while let Some(op) = self.match_operator(&[
            OperatorToken::NotEqual,
            OperatorToken::DoubleEqual,
            OperatorToken::StrictEqual,
            OperatorToken::StrictNotEqual,
            OperatorToken::Less,
            OperatorToken::LessEqual,
            OperatorToken::Greater,
            OperatorToken::GreaterEqual,
        ]) {
            let right = self.term()?; // Parse the right-hand side
            left = Expression::Binary {
                left: Box::new(left),
                op: Lexeme {
                    token: Token::Operator(op.clone()),
                    text: op.to_string(),
                    len: 1,
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_function_call(&mut self, name: Lexeme) -> Result<Expression, Vec<Error>> {
        println!("Parsing function call");
        self.advance(); // Consume open parenthesis

        let mut args = Vec::new();

        // Parse arguments
        while self.peek().token != Token::Delimiter(DelimiterToken::CloseParen) {
            let lexeme = self.peek().clone();
            if let Token::Identifier(_) = lexeme.token {
                args.push(Expression::Identifier(Identifier {
                    token: lexeme.clone(),
                    value: lexeme.text.clone(),
                }));
                self.advance();
            } else if let Token::Literal(_) = lexeme.token {
                if let Some(literal) = self.match_literal() {
                    args.push(Expression::Literal(literal));
                }
            }
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

        Ok(Expression::FunctionCall {
            name: Identifier {
                token: name.clone(),
                value: name.text.clone(),
            },
            args,
        })
    }
}
