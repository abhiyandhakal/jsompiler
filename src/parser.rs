use crate::lexer::symbol::{
    DelimiterToken, KeywordToken, Lexeme, LiteralToken, OperatorToken, Token,
};
use crate::{Error, ErrorKind};

#[derive(Debug)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(LiteralToken),
    Unary {
        op: Lexeme,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: Lexeme,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Lexeme,
    pub value: String,
}

pub struct Parser {
    tokens: Vec<Lexeme>,
    current: usize,
    pub errors: Vec<Error>,
    pub ast: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub value: Box<Expression>,
}

impl Parser {
    pub fn new(tokens: Vec<Lexeme>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
            ast: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(statement) => {
                    self.ast.push(Node::Statement(statement));
                }
                Err(errors) => self.errors.extend(errors),
            }
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, Vec<Error>> {
        if let Ok(expr) = self.expression() {
            self.ast.push(Node::Expression(expr.clone()));
            Ok(expr)
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

    fn expression(&mut self) -> Result<Expression, Vec<Error>> {
        self.term() // Start from highest precedence binary operations
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

    fn parse_statement(&mut self) -> Result<Statement, Vec<Error>> {
        match self.peek().token {
            Token::Keyword(KeywordToken::Let) => self.parse_let_statement(),
            Token::Keyword(KeywordToken::Var) => self.parse_let_statement(),
            Token::Keyword(KeywordToken::Const) => self.parse_let_statement(),
            _ => Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected statement".to_string(),
                line_number: 1,
                pos: 2,
            }]),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Vec<Error>> {
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

        if let Token::Identifier(_) = self.peek().token {
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

            Ok(Statement::LetStatement(LetStatement {
                token: name.token,
                value,
            }))
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected identifier after 'let'".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
    }

    fn match_operator(&mut self, operators: &[OperatorToken]) -> Option<OperatorToken> {
        for op in operators {
            if self.match_token(&Token::Operator(op.clone())) {
                return Some(op.clone());
            }
        }
        None
    }

    fn match_literal(&mut self) -> Option<LiteralToken> {
        let peek_val = self.peek().token.clone();
        match peek_val {
            Token::Literal(LiteralToken::Number(value)) => {
                let value = value.parse::<i64>().ok()?;
                self.advance();
                Some(LiteralToken::Number(value.to_string()))
            }
            Token::Literal(LiteralToken::Boolean(value)) => {
                self.advance();
                Some(LiteralToken::Boolean(value))
            }
            Token::Literal(LiteralToken::String(value)) => {
                self.advance();
                Some(LiteralToken::String(value.clone()))
            }
            _ => None,
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token == *token
    }

    fn previous(&self) -> &Lexeme {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn peek(&self) -> &Lexeme {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].token == Token::EOF
            || self.tokens[self.current].token == Token::Delimiter(DelimiterToken::Semicolon)
    }
}
