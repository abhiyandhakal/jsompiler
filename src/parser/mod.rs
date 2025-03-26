mod block_statement;
mod expression;
mod let_statement;
mod return_statement;

use block_statement::BlockStatement;
use expression::Expression;
use let_statement::LetStatement;
use return_statement::ReturnStatement;

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
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    BlockStatement(BlockStatement),
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

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            // println!("Looping");
            match self.parse_statement() {
                Ok(statement) => {
                    self.ast.push(Node::Statement(statement));
                }
                Err(errors) => self.errors.extend(errors),
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Vec<Error>> {
        match self.peek().token {
            Token::Delimiter(DelimiterToken::Semicolon)
            | Token::Delimiter(DelimiterToken::NewLine)
            | Token::Delimiter(DelimiterToken::CloseBrace) => {
                self.advance();
                self.parse_statement()
            }
            Token::Keyword(KeywordToken::Let) => self.parse_let_statement(),
            Token::Keyword(KeywordToken::Var) => self.parse_let_statement(),
            Token::Keyword(KeywordToken::Const) => self.parse_let_statement(),
            Token::Keyword(KeywordToken::Return) => self.parse_return_statement(),
            Token::Delimiter(DelimiterToken::OpenBrace) => self.parse_block_statement(),
            _ => Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected statement".to_string(),
                line_number: 1,
                pos: 2,
            }]),
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
        println!(
            "Current: {}, Tokens: {:?}",
            self.current, self.tokens[self.current].token
        );
        self.tokens[self.current].token == Token::EOF
    }
}
