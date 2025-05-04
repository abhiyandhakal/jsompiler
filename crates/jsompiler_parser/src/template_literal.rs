use super::{expression::Expression, Parser, Statement};
use crate::{expression, Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, KeywordToken, Token};

#[derive(Debug, Clone)]
pub enum TemplateLiteral {
    SubstitutionTemplate {
        template_head: Expression,
        expression: Expression,
        template_spans: TemplateSpan,
    },
    NonSubstitutionTemplate {
        text: Expression,
    },
}

#[derive(Debug, Clone)]
pub struct TemplateSpan {
    pub template_tail: Expression,
    pub template_middle_list: Option<TemplateMiddleList>,
}

#[derive(Debug, Clone)]
pub struct TemplateMiddleList {
    pub expression: Expression,
    pub template_middle: String,
    pub template_middle_list: Option<Box<TemplateMiddleList>>,
}

impl Parser {
    pub fn parse_template_literal(&mut self) -> Result<Expression, Vec<Error>> {
        if !self.match_token(&Token::Delimiter(DelimiterToken::Tilde)) {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected '`'".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
        let template_head = self.expression()?;

        while self.peek().token != Token::Delimiter(DelimiterToken::Tilde) {
            if self.peek().token == Token::Identifier('$'.to_string()) {
                self.advance(); // consume '$'
                if !self.match_token(&Token::Delimiter(DelimiterToken::OpenBrace)) {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected {".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }

                let expression = self.expression()?;

                if !self.match_token(&Token::Delimiter(DelimiterToken::CloseBrace)) {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected }".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
                let template_tail = self.expression()?;

                if self.peek().token == Token::Delimiter(DelimiterToken::Tilde) {
                    self.advance(); // consume '`'
                    return Ok(Expression::TemplateLiteral(Box::new(
                        TemplateLiteral::SubstitutionTemplate {
                            template_head,
                            expression,
                            template_spans: TemplateSpan {
                                template_tail,
                                template_middle_list: None,
                            },
                        },
                    )));
                } else {
                    return Err(vec![Error {
                        error_kind: ErrorKind::UnexpectedToken,
                        message: "Expected `".to_string(),
                        line_number: 1,
                        pos: 2,
                    }]);
                }
            }
            self.advance();
        }

        if self.peek().token == Token::Delimiter(DelimiterToken::Tilde) {
            self.advance(); // consume '`'
            Ok(Expression::TemplateLiteral(Box::new(
                TemplateLiteral::NonSubstitutionTemplate {
                    text: template_head,
                },
            )))
        } else {
            return Err(vec![Error {
                error_kind: ErrorKind::UnexpectedToken,
                message: "Expected `".to_string(),
                line_number: 1,
                pos: 2,
            }]);
        }
    }
}
