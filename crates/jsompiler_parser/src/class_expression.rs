use crate::function_expression::Parameter;
use crate::{expression::Expression, Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{
    ContextualKeywordToken, DelimiterToken, KeywordToken, LiteralToken, OperatorToken, Token,
};

#[derive(Debug, Clone)]
pub struct ClassExpression {
    pub identifier: Option<Identifier>,
    pub heritage: Box<Option<Expression>>,
    pub body: Vec<ClassElement>,
}

#[derive(Debug, Clone)]
pub enum ClassElement {
    MethodDefinition {
        name: ClassElementName,
        params: Vec<Parameter>,
        body: Vec<Statement>,
        is_static: bool,
    },
    AsyncMethodDefinition {
        name: ClassElementName,
        params: Vec<Parameter>,
        body: Vec<Statement>,
        is_static: bool,
    },
    FieldDefinition {
        name: ClassElementName,
        value: Option<Expression>,
        is_static: bool,
    },
    StaticBlock {
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum ClassElementName {
    PropertyName(String),
    PrivateIdentifier(String),
}

impl Parser {
    pub fn parse_class_expression(&mut self) -> Result<Expression, Vec<Error>> {
        println!("Parsing class expression");
        if !self.match_token(&Token::Keyword(KeywordToken::Class)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected 'class' keyword".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let identifier: Option<Identifier>;
        if let Token::Identifier(_) = self.peek().token {
            self.advance();
            identifier = Some(Identifier {
                token: self.previous().clone(),
                value: self.previous().text.clone(),
            });
        } else {
            identifier = None;
        }

        // Check for heritage (extends)
        let heritage: Option<Expression>;
        if self.match_token(&Token::Keyword(KeywordToken::Extends)) {
            heritage = Some(self.expression()?);
        } else {
            heritage = None;
        }

        if self.peek().token != Token::Delimiter(DelimiterToken::OpenBrace) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '{' after class name".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let body = self.parse_class_body();
        match body {
            Ok(body) => Ok(Expression::ClassExpression(ClassExpression {
                identifier,
                heritage: Box::new(heritage),
                body,
            })),
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn parse_class_body(&mut self) -> Result<Vec<ClassElement>, Vec<Error>> {
        self.advance(); // Consume '{'

        while self.peek().token == Token::Delimiter(DelimiterToken::NewLine)
            || self.peek().token == Token::Delimiter(DelimiterToken::Semicolon)
        {
            self.advance();
        }

        let mut elements = Vec::new();

        while self.peek().token != Token::Delimiter(DelimiterToken::CloseBrace) && !self.is_at_end()
        {
            while self.peek().token == Token::Delimiter(DelimiterToken::NewLine)
                || self.peek().token == Token::Delimiter(DelimiterToken::Semicolon)
            {
                self.advance();
            }

            if self.peek().token == Token::Delimiter(DelimiterToken::CloseBrace) {
                break;
            }

            // Parse a class element
            let element = self.parse_class_element();
            match element {
                Ok(e) => elements.push(e),
                Err(err) => return Err(err),
            }
        }

        if self.peek().token != Token::Delimiter(DelimiterToken::CloseBrace) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '}' to end class body".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        self.advance(); // Consume '}'

        Ok(elements)
    }

    // Parse a single class element (method, field, or static block)
    fn parse_class_element(&mut self) -> Result<ClassElement, Vec<Error>> {
        let is_static =
            if self.peek().token == Token::ContextualKeyword(ContextualKeywordToken::Static) {
                self.advance();
                true
            } else {
                false
            };

        if is_static && self.peek().token == Token::Delimiter(DelimiterToken::OpenBrace) {
            return self.parse_static_block();
        }

        let is_async =
            if self.peek().token == Token::ContextualKeyword(ContextualKeywordToken::Async) {
                self.advance();
                true
            } else {
                false
            };

        let name = match self.peek().token {
            Token::Identifier(_) => ClassElementName::PropertyName(self.peek().text.clone()),
            Token::PrivateIdentifier(_) => {
                ClassElementName::PrivateIdentifier(self.peek().text.clone())
            }
            Token::Literal(LiteralToken::String(_)) => {
                ClassElementName::PropertyName(self.peek().text.clone())
            }
            _ => {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected class element name".to_string(),
                    line_number: 1,
                    pos: 2,
                }])
            }
        };

        self.advance(); // Consume the name

        if self.peek().token == Token::Delimiter(DelimiterToken::OpenParen) {
            return self.parse_method(name, is_static, is_async);
        } else {
            // Field definition
            if is_async {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Async fields are not allowed ".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            let value = if self.peek().token == Token::Operator(OperatorToken::EqualTo) {
                self.advance(); // Consume '='
                Some(self.expression()?)
            } else {
                None
            };

            if self.peek().token != Token::Delimiter(DelimiterToken::Semicolon)
                && self.peek().token != Token::Delimiter(DelimiterToken::NewLine)
            {
                return Err(vec![Error {
                    error_kind: ErrorKind::SyntaxError,
                    message: "Expected ';' after field definition".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            if self.peek().token == Token::Delimiter(DelimiterToken::Semicolon) {
                self.advance();
            }

            Ok(ClassElement::FieldDefinition {
                name,
                value,
                is_static,
            })
        }
    }

    fn parse_static_block(&mut self) -> Result<ClassElement, Vec<Error>> {
        let body = self.parse_block_statement()?;
        Ok(ClassElement::StaticBlock { body })
    }

    fn parse_method(
        &mut self,
        name: ClassElementName,
        is_static: bool,
        is_async: bool,
    ) -> Result<ClassElement, Vec<Error>> {
        let params = self.parse_function_parameters()?;

        let body = self.parse_block_statement()?;

        if is_async {
            Ok(ClassElement::AsyncMethodDefinition {
                name,
                params,
                body,
                is_static,
            })
        } else {
            Ok(ClassElement::MethodDefinition {
                name,
                params,
                body,
                is_static,
            })
        }
    }
}
