use super::{Identifier, Parser, Statement};
use crate::class_expression::ClassExpression;
use crate::function_expression::{FunctionExpression, Parameter};
use crate::object_expression::Property;
use crate::template_literal::TemplateLiteral;
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{
    DelimiterToken, KeywordToken, Lexeme, LiteralToken, OperatorToken, Token,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(LiteralToken),
    TemplateLiteral(Box<TemplateLiteral>),
    ThisExpression,
    SpreadElement(Box<Expression>),
    MemberAccess {
        object: Box<Expression>,
        property: Box<Expression>,
    },
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
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    ArrayLiteral {
        elements: Vec<Expression>,
    },
    ObjectLiteral {
        properties: Vec<Property>,
    },
    AwaitExpression(Box<Expression>),
    ClassExpression(ClassExpression),
    FunctionExpression(FunctionExpression),
    GeneratorExpression(FunctionExpression),
    AsyncFunctionExpression(FunctionExpression),
    AsyncGeneratorExpression(FunctionExpression),
    RegularExpressionLiteral {
        pattern: String,
        flags: String,
    },
    ArrowFunctionExpression {
        parameters: Vec<Parameter>,
        body: Box<Vec<Statement>>,
    },
}

impl Parser {
    pub fn parenthesis_expression(&mut self) -> Result<Expression, Vec<Error>> {
        let current = self.current;
        self.advance(); // Consuming open parenthesis
        if self.peek().token == Token::Delimiter(DelimiterToken::CloseParen) {
            if self.next().token != Token::Operator(OperatorToken::Arrow) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected expression or arrow function".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            self.current = current;
            return self.parse_arrow_expression();
        }
        let exp = self.expression();

        match exp {
            Ok(exp) => {
                if self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
                    if self.peek().token == Token::Operator(OperatorToken::Arrow) {
                        self.current = current;
                        return self.parse_arrow_expression();
                    }
                    Ok(exp)
                } else {
                    self.current = current;
                    self.parse_arrow_expression()
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse_arrow_expression(&mut self) -> Result<Expression, Vec<Error>> {
        let params = self.parse_function_parameters();
        if self.peek().token == Token::Operator(OperatorToken::Arrow) {
            self.advance(); //Consume Arrow Operator

            if self.peek().token == Token::Delimiter(DelimiterToken::OpenBrace) {
                let body = self.parse_block_statement();
                match body {
                    Ok(body) => Ok(Expression::ArrowFunctionExpression {
                        parameters: params?,
                        body: Box::new(body),
                    }),
                    Err(err) => Err(err),
                }
            } else {
                let body = self.expression();
                match body {
                    Ok(body) => Ok(Expression::ArrowFunctionExpression {
                        parameters: params?,
                        body: Box::new(vec![Statement::ExpressionStatement(body)]),
                    }),
                    Err(err) => Err(err),
                }
            }
        } else {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '=>'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
    }

    pub fn parse_expression(&mut self) -> Result<Vec<Statement>, Vec<Error>> {
        let expr = self.expression();
        match expr {
            Ok(expr) => Ok(vec![Statement::ExpressionStatement(expr)]),
            Err(err) => {
                for e in err.iter() {
                    self.errors.push(e.clone());
                }
                Err(self.errors.clone())
            }
        }
    }

    pub fn expression(&mut self) -> Result<Expression, Vec<Error>> {
        // Parse array expression
        if self.peek().token == Token::Delimiter(DelimiterToken::OpenBracket) {
            return self.array_expression();
        }
        if self.peek().token == Token::Delimiter(DelimiterToken::OpenParen) {
            return self.parenthesis_expression();
        }
        if self.peek().token == Token::Delimiter(DelimiterToken::OpenBrace) {
            return self.parse_object_expression();
        }
        if self.peek().token == Token::Keyword(KeywordToken::Class) {
            return self.parse_class_expression();
        }
        if self.peek().token == Token::Keyword(KeywordToken::Function) {
            return self.parse_function_expression();
        }
        if self.peek().token == Token::Operator(OperatorToken::Spread) {
            return self.parse_spread_operator();
        }
        if self.peek().token
            == Token::ContextualKeyword(jsompiler_lexer::symbol::ContextualKeywordToken::Async)
        {
            return self.parse_async_function_expression();
        }

        if self.peek().token
            == Token::ContextualKeyword(jsompiler_lexer::symbol::ContextualKeywordToken::Await)
        {
            return self.parse_await_expression();
        }
        if let Token::RegExp {
            pattern: _,
            flags: _,
        } = &self.peek().token
        {
            return self.parse_regular_expression();
        }
        if self.peek().token == Token::Delimiter(DelimiterToken::Tilde) {
            return self.parse_template_literal();
        }

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
        let mut expr = if self.match_token(&Token::Delimiter(DelimiterToken::OpenParen)) {
            let expr = self.expression()?;
            if !self.match_token(&Token::Delimiter(DelimiterToken::CloseParen)) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected ')' after expression".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            expr
        } else if let Token::Keyword(KeywordToken::This) = self.peek().token {
            self.advance(); // Consume 'this'
            Expression::ThisExpression
        } else if let Token::Identifier(_) = self.peek().token {
            self.advance();
            let identifier = self.previous().clone();
            Expression::Identifier(Identifier {
                token: identifier.clone(),
                value: identifier.text.clone(),
            })
        } else if let Some(literal) = self.match_literal() {
            Expression::Literal(literal)
        } else {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected expression".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        };

        // After parsing the primary expression, look for member access or function calls
        loop {
            if self.peek().token == Token::Delimiter(DelimiterToken::Dot) {
                expr = self.parse_member_access(expr)?;
            } else if self.peek().token == Token::Delimiter(DelimiterToken::OpenParen) {
                expr = self.parse_function_call(expr)?;
            } else if self.peek().token == Token::Delimiter(DelimiterToken::OpenBracket) {
                expr = self.parse_member_access(expr)?;
            } else if self.match_token(&Token::Operator(OperatorToken::Increment))
                || self.match_token(&Token::Operator(OperatorToken::Decrement))
            {
                // Postfix increment/decrement
                expr = Expression::Unary {
                    op: self.previous().clone(),
                    op_type: "Postfix".to_string(),
                    expr: Box::new(expr),
                };
            } else {
                break; // No more member access or function calls
            }
        }

        if self.peek().token == Token::Delimiter(DelimiterToken::NewLine) {
            self.advance(); // Consume newline
        }

        Ok(expr)
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

    fn parse_function_call(&mut self, callee: Expression) -> Result<Expression, Vec<Error>> {
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
            callee: Box::new(callee),
            args,
        })
    }

    pub fn array_expression(&mut self) -> Result<Expression, Vec<Error>> {
        self.advance(); // Consume open bracket
        let mut elements = Vec::new();

        while self.peek().token != Token::Delimiter(DelimiterToken::CloseBracket) {
            if self.peek().token == Token::Delimiter(DelimiterToken::Comma) {
                let expr = Expression::Literal(LiteralToken::Undefined);
                elements.push(expr);
            } else {
                let expr = self.expression()?;
                elements.push(expr);
            }

            if !self.match_token(&Token::Delimiter(DelimiterToken::Comma))
                && self.peek().token != Token::Delimiter(DelimiterToken::CloseBracket)
            {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected ',' or ']'".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
        }

        if !self.match_token(&Token::Delimiter(DelimiterToken::CloseBracket)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected ']'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        Ok(Expression::ArrayLiteral { elements })
    }

    fn parse_member_access(&mut self, expr: Expression) -> Result<Expression, Vec<Error>> {
        match self.peek().token {
            Token::Delimiter(DelimiterToken::Dot) => {
                self.advance(); // Consume the dot

                if let Token::Identifier(_) = self.peek().token {
                    self.advance();
                    let property = self.previous().clone();
                    Ok(Expression::MemberAccess {
                        object: Box::new(expr),
                        property: Box::new(Expression::Identifier(Identifier {
                            token: property.clone(),
                            value: property.text.clone(),
                        })),
                    })
                } else if let Token::PrivateIdentifier(_) = self.peek().token {
                    self.advance();
                    let property = self.previous().clone();
                    Ok(Expression::MemberAccess {
                        object: Box::new(expr),
                        property: Box::new(Expression::Identifier(Identifier {
                            token: property.clone(),
                            value: property.text.clone(),
                        })),
                    })
                } else {
                    Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected identifier after '.'".to_string(),
                        line_number: 1,
                        pos: 2,
                    }])
                }
            }

            Token::Delimiter(DelimiterToken::OpenBracket) => {
                self.advance(); // Consume the open bracket
                let property = self.expression()?;
                if !self.match_token(&Token::Delimiter(DelimiterToken::CloseBracket)) {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected ']'".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
                Ok(Expression::MemberAccess {
                    object: Box::new(expr),
                    property: Box::new(property),
                })
            }

            _ => Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected during member access".to_string(),
                line_number: 1,
                pos: 2,
            }]),
        }
    }

    fn parse_spread_operator(&mut self) -> Result<Expression, Vec<Error>> {
        self.advance(); // Consume the spread operator
        let expr = self.expression()?;
        Ok(Expression::SpreadElement(Box::new(expr)))
    }

    fn parse_await_expression(&mut self) -> Result<Expression, Vec<Error>> {
        self.advance(); // Consume 'await'
        let expr = self.expression()?;
        Ok(Expression::AwaitExpression(Box::new(expr)))
    }

    fn parse_async_function_expression(&mut self) -> Result<Expression, Vec<Error>> {
        self.advance(); // Consume 'async'
        let expr = self.parse_function_expression()?;

        if let Expression::FunctionExpression(function_expression) = expr {
            return Ok(Expression::AsyncFunctionExpression(function_expression));
        }
        if let Expression::GeneratorExpression(function_expression) = expr {
            return Ok(Expression::AsyncGeneratorExpression(function_expression));
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected async function expression".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }

    fn parse_regular_expression(&mut self) -> Result<Expression, Vec<Error>> {
        if let Token::RegExp { pattern, flags } = &self.peek().token {
            let pattern = pattern.clone();
            let flags = flags.clone();
            self.advance(); // Consume the regular expression token
            Ok(Expression::RegularExpressionLiteral { pattern, flags })
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected regular expression".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }
}
