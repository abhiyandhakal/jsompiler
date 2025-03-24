use crate::lexer::symbol::{Lexeme, LiteralToken, OperatorToken, Token};
use crate::{Error, ErrorKind};

#[derive(Debug)]
pub enum Node {
    Expression(Expression),
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

impl Parser {
    pub fn new(tokens: Vec<Lexeme>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
            ast: Vec::new(),
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
        self.unary()
    }

    fn unary(&mut self) -> Result<Expression, Vec<Error>> {
        if self.match_token(&Token::Operator(OperatorToken::Minus))
            || self.match_token(&Token::Operator(OperatorToken::Not))
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
        if let Token::Identifier(_) = self.peek().token {
            self.advance();
            let identifier = self.previous().clone();
            Ok(Expression::Identifier(Identifier {
                token: identifier.clone(),
                value: identifier.text.clone(),
            }))
        } else if let Some(literal) = self.match_literal() {
            Ok(Expression::Literal(literal))
        } else {
            Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected expression".to_string(),
                line_number: 1,
                pos: 2,
            }])
        }
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
    }
}
