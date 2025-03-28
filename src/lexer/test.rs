#[allow(unused_imports)]
use crate::lexer::symbol::{
    DelimiterToken, KeywordToken, LiteralToken, OperatorToken, StringLiteral,
};
#[allow(unused_imports)]
use crate::lexer::{Lexer, Token};

#[test]
fn test_lexer_var_declaration() {
    let input = "let x = 5;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_eq!(lexer.errors, vec![]);
    assert_eq!(
        lexer
            .tokens
            .iter()
            .map(|l| l.token.clone())
            .collect::<Vec<_>>(),
        vec![
            Token::Keyword(KeywordToken::Let),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Number("5".to_string())),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::EOF
        ]
    );
}

#[test]
fn test_lexer_var_declaration_no_semicolon() {
    let input = "let x = 5";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_eq!(lexer.errors, vec![]);
    assert_eq!(
        lexer
            .tokens
            .iter()
            .map(|l| l.token.clone())
            .collect::<Vec<_>>(),
        vec![
            Token::Keyword(KeywordToken::Let),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Number("5".to_string())),
            Token::EOF
        ]
    );
}

#[test]
fn test_lexer_string_concat() {
    let input = "let msg = \"hello\" + \" \" + \"world\";";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_eq!(lexer.errors, vec![]);
    assert_eq!(
        lexer
            .tokens
            .iter()
            .map(|l| l.token.clone())
            .collect::<Vec<_>>(),
        vec![
            Token::Keyword(KeywordToken::Let),
            Token::Identifier("msg".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::String(StringLiteral::Regular(
                "hello".to_string()
            ))),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::String(StringLiteral::Regular(
                " ".to_string()
            ))),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::String(StringLiteral::Regular(
                "world".to_string()
            ))),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::EOF
        ]
    );
}

#[test]
fn test_lexer_arithmetic() {
    let input = "let result = (5 * 10 + 2) / 4 - 3;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_eq!(lexer.errors, vec![]);
    assert_eq!(
        lexer
            .tokens
            .iter()
            .map(|l| l.token.clone())
            .collect::<Vec<_>>(),
        vec![
            Token::Keyword(KeywordToken::Let),
            Token::Identifier("result".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Delimiter(DelimiterToken::OpenParen),
            Token::Literal(LiteralToken::Number("5".to_string())),
            Token::Operator(OperatorToken::Asterisk),
            Token::Literal(LiteralToken::Number("10".to_string())),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::Number("2".to_string())),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::Operator(OperatorToken::Slash),
            Token::Literal(LiteralToken::Number("4".to_string())),
            Token::Operator(OperatorToken::Minus),
            Token::Literal(LiteralToken::Number("3".to_string())),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::EOF
        ]
    );
}

#[test]
fn test_lexer_invalid_input() {
    let input = "let @invalid = 5;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_ne!(lexer.errors, vec![]);

    let input = "let x = \"unclosed string;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_ne!(lexer.errors, vec![]);

    let input = "let x = 5.5.5;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_ne!(lexer.errors, vec![]);
}
