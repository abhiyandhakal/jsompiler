use crate::{expression::Expression, Identifier, Parser, Statement};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{
    ContextualKeywordToken, DelimiterToken, KeywordToken, LiteralToken, NumberLiteral,
    OperatorToken, Token,
};

// Enhanced Property types for object literals
#[derive(Debug, Clone)]
pub enum PropertyKey {
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(NumberLiteral),
    Computed(Box<Expression>), // For computed properties like { [expr]: value }
}

#[derive(Debug, Clone)]
pub enum Property {
    KeyValue {
        key: PropertyKey,
        value: Box<Expression>,
    },
    Shorthand(String), // For shorthand properties like { x } instead of { x: x }
    Method {
        key: PropertyKey,
        function: Statement,
    },
    Getter {
        key: PropertyKey,
        body: Statement,
    },
    Setter {
        key: PropertyKey,
        param: Identifier,
        body: Statement,
    },
    Spread(Expression), // For spread properties like { ...obj }
}

impl Parser {
    pub fn parse_object_expression(&mut self) -> Result<Expression, Vec<Error>> {
        println!("Parsing object literal");
        let _start = self.current;
        self.advance(); // Consume '{'

        let mut properties = Vec::new();

        // Parse object properties until we hit a closing brace
        while self.peek().token != Token::Delimiter(DelimiterToken::CloseBrace) && !self.is_at_end()
        {
            // Skip any newlines
            while self.peek().token == Token::Delimiter(DelimiterToken::NewLine) {
                self.advance();
            }

            // Check for Closing Brace
            if self.peek().token == Token::Delimiter(DelimiterToken::CloseBrace) {
                break;
            }

            // Parse a property
            let property = self.parse_object_property()?;
            properties.push(property);

            // Check for comma
            if self.peek().token == Token::Delimiter(DelimiterToken::Comma) {
                self.advance();
            } else if self.peek().token != Token::Delimiter(DelimiterToken::CloseBrace) {
                return Err(vec![Error {
                    error_kind: ErrorKind::SyntaxError,
                    message: "Expected ',' or '}'".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
        }

        // Consume closing brace
        if self.peek().token != Token::Delimiter(DelimiterToken::CloseBrace) {
            return Err(vec![Error {
                error_kind: ErrorKind::SyntaxError,
                message: "Expected '}'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        self.advance();

        Ok(Expression::ObjectLiteral { properties })
    }

    fn parse_object_property(&mut self) -> Result<Property, Vec<Error>> {
        // println!("Parsing object property");
        // Check for getter/setter methods
        if self.peek().token == Token::ContextualKeyword(ContextualKeywordToken::Get)
            || self.peek().token == Token::ContextualKeyword(ContextualKeywordToken::Set)
        {
            return self.parse_accessor_property();
        }

        if self.peek().token == Token::Operator(OperatorToken::Spread) {
            let expr = self.expression()?;
            return Ok(Property::Spread(expr));
        }
        // Parse the property key
        let key = self.parse_property_key()?;

        // Check if it's a method
        if self.peek().token == Token::Delimiter(DelimiterToken::OpenParen) {
            return self.parse_method_property(key);
        }

        // Check if it's a shorthand property (no colon)
        if let PropertyKey::Identifier(name) = &key {
            if self.peek().token != Token::Operator(OperatorToken::Colon)
                && self.peek().token == Token::Delimiter(DelimiterToken::Comma)
            {
                // It's a shorthand property like { name }
                return Ok(Property::Shorthand(name.clone()));
            }
        }

        // Regular key-value property
        if self.peek().token != Token::Operator(OperatorToken::Colon) {
            return Err(vec![Error {
                error_kind: ErrorKind::SyntaxError,
                message: "Expected ':' after property name".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        self.advance(); // Consume ':'

        let value = self.expression()?;

        Ok(Property::KeyValue {
            key,
            value: Box::new(value),
        })
    }

    fn parse_property_key(&mut self) -> Result<PropertyKey, Vec<Error>> {
        match &self.peek().token {
            Token::Identifier(name) => {
                let key_name = name.clone();
                self.advance();
                Ok(PropertyKey::Identifier(key_name))
            }
            Token::Literal(LiteralToken::String(s)) => {
                let key_str = s.clone();
                self.advance();
                Ok(PropertyKey::StringLiteral(key_str))
            }
            Token::Literal(LiteralToken::Number(n)) => {
                let key_num = n.clone();
                self.advance();
                Ok(PropertyKey::NumberLiteral(key_num))
            }
            Token::Delimiter(DelimiterToken::OpenBracket) => {
                self.advance(); // Consume '['
                let expr = self.expression()?;
                if self.peek().token != Token::Delimiter(DelimiterToken::CloseBracket) {
                    return Err(vec![Error {
                        error_kind: ErrorKind::SyntaxError,
                        message: "Expected ']'".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
                self.advance(); // Consume ']'
                Ok(PropertyKey::Computed(Box::new(expr)))
            }
            _ => Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected property name".to_string(),
                line_number: 1,
                pos: 2,
            }]),
        }
    }

    fn parse_method_property(&mut self, key: PropertyKey) -> Result<Property, Vec<Error>> {
        // Consume '('
        self.advance();

        // Parse parameters
        let function = self.parse_function_statement()?;
        match function {
            Some(func) => Ok(Property::Method {
                key,
                function: func,
            }),
            None => {
                return Err(vec![Error {
                    error_kind: ErrorKind::SyntaxError,
                    message: "Expected function body".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
        }
    }

    fn parse_accessor_property(&mut self) -> Result<Property, Vec<Error>> {
        let is_getter = self.peek().token == Token::ContextualKeyword(ContextualKeywordToken::Get);
        self.advance(); // Consume 'get' or 'set'

        let key = self.parse_property_key()?;
        println!("Parsed accessor property key: {:?}", key);

        if self.peek().token != Token::Delimiter(DelimiterToken::OpenParen) {
            return Err(vec![Error {
                error_kind: ErrorKind::SyntaxError,
                message: "Expected '(' after accessor name".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        self.advance(); // Consume '('

        if is_getter {
            // Getter should have no parameters
            if self.peek().token != Token::Delimiter(DelimiterToken::CloseParen) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Getter should not have parameters".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            self.advance(); // Consume ')'

            let body = self.parse_function_body()?;
            println!("Parsed getter body: {:#?}", body);
            Ok(Property::Getter { key, body })
        } else {
            // Setter should have exactly one parameter
            match &self.peek().token {
                Token::Identifier(_) => { /* OK, it's an identifier */ }
                _ => {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected parameter name for setter".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
            }

            let param_name = match &self.peek().token {
                Token::Identifier(name) => Identifier {
                    token: self.peek().clone(),
                    value: name.to_string(),
                },
                _ => unreachable!(), // We already checked it's an identifier
            };
            self.advance(); // Consume parameter name

            if self.peek().token != Token::Delimiter(DelimiterToken::CloseParen) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected ')' after setter parameter".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }
            self.advance(); // Consume ')'

            let body = self.parse_function_body()?;
            Ok(Property::Setter {
                key,
                param: param_name,
                body,
            })
        }
    }

    fn parse_function_body(&mut self) -> Result<Statement, Vec<Error>> {
        println!("Parsing function body");
        if self.peek().token != Token::Delimiter(DelimiterToken::OpenBrace) {
            return Err(vec![Error {
                error_kind: ErrorKind::SyntaxError,
                message: "Expected '{' to start function body".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }

        let sstms = self.parse_block_statement()?;
        match sstms {
            Some(block_stmt) => return Ok(block_stmt),
            None => Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected a block statement in function body".to_string(),
                line_number: 1,
                pos: 2,
            }]),
        }
    }

    // Add this helper method to recognize object literals vs block statements
    pub fn parse_brace_block_or_object(&mut self) -> Result<Option<Statement>, Vec<Error>> {
        let checkpoint = self.current;
        self.advance(); // Consume `{`

        // Skip any newlines
        while self.peek().token == Token::Delimiter(DelimiterToken::NewLine) {
            self.advance();
        }

        // Check if it's potentially an object literal
        let is_object_literal = match &self.peek().token {
            // Common start tokens for object literals
            Token::Identifier(_)
            | Token::Literal(LiteralToken::String(_))
            | Token::Literal(LiteralToken::Number(_))
            | Token::ContextualKeyword(ContextualKeywordToken::Get)
            | Token::ContextualKeyword(ContextualKeywordToken::Set)
            | Token::Delimiter(DelimiterToken::OpenBracket) => true,

            // If it's a closing brace, it's an empty object
            Token::Delimiter(DelimiterToken::CloseBrace) => true,

            // If we see block-specific tokens, it's definitely a block
            Token::ContextualKeyword(ContextualKeywordToken::Let)
            | Token::Keyword(KeywordToken::Var)
            | Token::Keyword(KeywordToken::Const)
            | Token::Keyword(KeywordToken::Function)
            | Token::Keyword(KeywordToken::If)
            | Token::Keyword(KeywordToken::For)
            | Token::Keyword(KeywordToken::While)
            | Token::Keyword(KeywordToken::Return) => false,

            // For other tokens, we need more context
            _ => false,
        };

        // Reset position
        self.current = checkpoint;

        if is_object_literal {
            let expr = self.parse_object_expression()?;
            Ok(Some(Statement::ExpressionStatement(expr)))
        } else {
            self.parse_block_statement()
        }
    }
}
