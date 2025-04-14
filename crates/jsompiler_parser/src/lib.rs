mod assignment_statement;
mod block_statement;
mod class_expression;
mod expression;
mod for_loop_statement;
mod function_statement;
mod if_statement;
mod let_statement;
mod object_expression;
mod return_statement;
mod while_statement;
mod yield_statement;

use assignment_statement::AssignmentStatement;
use block_statement::BlockStatement;
use expression::Expression;
use for_loop_statement::ForLoopStatement;
use function_statement::FunctionStatement;
use if_statement::IfStatement;
use let_statement::LetStatement;
use return_statement::ReturnStatement;
use while_statement::WhileStatement;
use yield_statement::YieldStatement;

use jsompiler_common::{Error, ErrorKind};
use jsompiler_lexer::symbol::{
    ContextualKeywordToken, DelimiterToken, KeywordToken, Lexeme, LiteralToken, OperatorToken,
    Token,
};

#[derive(Debug)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(Expression),
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    FunctionStatement(FunctionStatement),
    AssignmentStatement(AssignmentStatement),
    ForLoopStatement(ForLoopStatement),
    YieldStatement(YieldStatement),
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
            match &self.peek().token {
                Token::Delimiter(DelimiterToken::Semicolon)
                | Token::Delimiter(DelimiterToken::NewLine)
                | Token::Delimiter(DelimiterToken::CloseBrace) => {
                    self.advance();
                }
                Token::EOF => break,
                _ => {}
            }
            match self.parse_statement() {
                Ok(Some(statement)) => {
                    self.ast.push(Node::Statement(statement));
                }
                Err(errors) => {
                    self.errors.extend(errors);
                    break;
                }
                _ => {}
            }
        }
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        match &self.peek().token {
            Token::Delimiter(DelimiterToken::Semicolon)
            | Token::Delimiter(DelimiterToken::NewLine) => {
                self.advance();
                self.parse_statement()
            }
            Token::Literal(_)
            | Token::Operator(_)
            | Token::Keyword(KeywordToken::Class)
            | Token::Delimiter(DelimiterToken::OpenBracket) => self.parse_expression(),
            Token::Identifier(_) | Token::Keyword(KeywordToken::This) => {
                self.parse_assignment_statement()
            }
            Token::ContextualKeyword(ContextualKeywordToken::Let)
            | Token::Keyword(KeywordToken::Var)
            | Token::Keyword(KeywordToken::Const) => {
                let stmts = self.parse_let_statement()?;
                for stmt in stmts {
                    let stmt = stmt.unwrap();
                    self.ast.push(Node::Statement(stmt));
                }
                Ok(None)
            }
            Token::Keyword(KeywordToken::Return) => self.parse_return_statement(),
            Token::Keyword(KeywordToken::If) => self.parse_if_statement(),
            Token::Keyword(KeywordToken::While) => self.parse_while_statement(),
            Token::Keyword(KeywordToken::For) => self.parser_for_loop_statement(),
            Token::Keyword(KeywordToken::Function) => self.parse_function_statement(),
            Token::ContextualKeyword(ContextualKeywordToken::Yield) => self.parse_yield_statement(),
            Token::Delimiter(DelimiterToken::OpenBrace) => self.parse_brace_block_or_object(),
            Token::Delimiter(DelimiterToken::OpenParen) => self.parenthesis_expression(),
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
                self.advance();
                Some(LiteralToken::Number(value))
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

    fn next(&self) -> &Lexeme {
        &self.tokens[self.current + 1]
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
