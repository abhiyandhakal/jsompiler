#[allow(unused_imports)]
use super::symbol::{
    CommentToken, DelimiterToken, KeywordToken, LiteralToken, NumberLiteral, OperatorToken,
    StringLiteral,
};
#[allow(unused_imports)]
use super::{Lexer, Token};

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
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(5_f64))),
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
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(5_f64))),
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
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(5_f64))),
            Token::Operator(OperatorToken::Asterisk),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(10_f64))),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(2_f64))),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::Operator(OperatorToken::Slash),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(4_f64))),
            Token::Operator(OperatorToken::Minus),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(3_f64))),
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
    println!("{:?}", lexer.tokens);
    assert_ne!(lexer.errors, vec![]);
}

#[test]
fn test_fn() {
    let input = r#"function() {
            //
        }"#;
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
            Token::Keyword(KeywordToken::Function),
            Token::Delimiter(DelimiterToken::OpenParen),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::Delimiter(DelimiterToken::OpenBrace),
            Token::Delimiter(DelimiterToken::NewLine),
            Token::Comment(CommentToken::Line("".to_string())),
            Token::Delimiter(DelimiterToken::NewLine),
            Token::Delimiter(DelimiterToken::CloseBrace),
            Token::EOF
        ]
    );
}

#[test]
fn test_arrow_fn() {
    let input = r#"const test = (a, b) => {
            //
        }"#;
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
            Token::Keyword(KeywordToken::Const),
            Token::Identifier("test".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Delimiter(DelimiterToken::OpenParen),
            Token::Identifier("a".to_string()),
            Token::Delimiter(DelimiterToken::Comma),
            Token::Identifier("b".to_string()),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::Operator(OperatorToken::Arrow),
            Token::Delimiter(DelimiterToken::OpenBrace),
            Token::Delimiter(DelimiterToken::NewLine),
            Token::Comment(CommentToken::Line("".to_string())),
            Token::Delimiter(DelimiterToken::NewLine),
            Token::Delimiter(DelimiterToken::CloseBrace),
            Token::EOF
        ]
    );
}

#[test]
fn test_template_string() {
    let input = r#"let x = `"This
        is template string."`"#;
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
            Token::Literal(LiteralToken::String(StringLiteral::Template(
                r#""This
        is template string.""#
                    .to_string()
            ))),
            Token::EOF
        ]
    );
}

#[test]
fn test_nan() {
    let input = "x = NaN";
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
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Number(NumberLiteral::NaN)),
        ]
    );
}
