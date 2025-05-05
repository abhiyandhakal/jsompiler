use super::{expression::Expression, Parser};
use crate::{Error, ErrorKind};
use jsompiler_lexer::symbol::{DelimiterToken, Token};

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
    pub template_middle: Expression,
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

        if self.peek().token == Token::Delimiter(DelimiterToken::Tilde) {
            self.advance(); // consume closing '`'
            return Ok(Expression::TemplateLiteral(Box::new(
                TemplateLiteral::NonSubstitutionTemplate {
                    text: template_head,
                },
            )));
        }

        // Handle templates with substitutions
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

            let first_expression = self.expression()?;

            if !self.match_token(&Token::Delimiter(DelimiterToken::CloseBrace)) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected }".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            // Parse the template spans (potentially multiple)
            let template_spans = self.parse_template_spans()?;

            if !self.match_token(&Token::Delimiter(DelimiterToken::Tilde)) {
                return Err(vec![Error {
                    error_kind: ErrorKind::UnexpectedToken,
                    message: "Expected `".to_string(),
                    line_number: 1,
                    pos: 2,
                }]);
            }

            return Ok(Expression::TemplateLiteral(Box::new(
                TemplateLiteral::SubstitutionTemplate {
                    template_head,
                    expression: first_expression,
                    template_spans,
                },
            )));
        }

        Err(vec![Error {
            error_kind: ErrorKind::UnexpectedToken,
            message: "Expected $ or `".to_string(),
            line_number: 1,
            pos: 2,
        }])
    }

    fn parse_template_spans(&mut self) -> Result<TemplateSpan, Vec<Error>> {
        let template_tail = self.expression()?;

        // No more substitutions
        if self.peek().token == Token::Delimiter(DelimiterToken::Tilde) {
            return Ok(TemplateSpan {
                template_tail,
                template_middle_list: None,
            });
        }

        // We have another substitution
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

            // Recursively parse the rest of the template
            let next_spans = self.parse_template_spans()?;

            let template_middle_list = TemplateMiddleList {
                expression,
                template_middle: template_tail.clone(),
                template_middle_list: next_spans.template_middle_list.map(Box::new),
            };

            return Ok(TemplateSpan {
                template_tail: next_spans.template_tail,
                template_middle_list: Some(template_middle_list),
            });
        }

        Err(vec![Error {
            error_kind: ErrorKind::UnexpectedToken,
            message: "Internal parser error in template spans".to_string(),
            line_number: 1,
            pos: 2,
        }])
    }
}

