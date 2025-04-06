#[allow(unused_imports)]
use super::symbol::{
    CommentToken, ContextualKeywordToken, DelimiterToken, KeywordToken, LiteralToken,
    NumberLiteral, OperatorToken,
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
            Token::ContextualKeyword(ContextualKeywordToken::Let),
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
            Token::ContextualKeyword(ContextualKeywordToken::Let),
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
            Token::ContextualKeyword(ContextualKeywordToken::Let),
            Token::Identifier("msg".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::String("hello".to_string())),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::String(" ".to_string())),
            Token::Operator(OperatorToken::Plus),
            Token::Literal(LiteralToken::String("world".to_string())),
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
            Token::ContextualKeyword(ContextualKeywordToken::Let),
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
        is ${template()} string."`"#;
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
            Token::ContextualKeyword(ContextualKeywordToken::Let),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Delimiter(DelimiterToken::Tilde),
            Token::Literal(LiteralToken::String(
                r#""This
        is "#
                    .to_string()
            )),
            Token::Identifier("$".to_string()),
            Token::Delimiter(DelimiterToken::OpenBrace),
            Token::Identifier("template".to_string()),
            Token::Delimiter(DelimiterToken::OpenParen),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::Delimiter(DelimiterToken::CloseBrace),
            Token::Literal(LiteralToken::String(r#" string.""#.to_string())),
            Token::Delimiter(DelimiterToken::Tilde),
            Token::EOF
        ]
    );
}

#[test]
fn test_literal_keywords() {
    let input = "x = NaN; y = undefined; z = null";
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
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("y".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Undefined),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("z".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Null),
            Token::EOF
        ]
    );
}

#[test]
fn test_unary() {
    let input = "x = y++; x = ++y; x = y--; x = --y;";
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
            Token::Identifier("y".to_string()),
            Token::Operator(OperatorToken::Increment),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Operator(OperatorToken::Increment),
            Token::Identifier("y".to_string()),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Identifier("y".to_string()),
            Token::Operator(OperatorToken::Decrement),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Operator(OperatorToken::Decrement),
            Token::Identifier("y".to_string()),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::EOF
        ]
    );
}

#[test]
fn test_base_values() {
    let input = "x = 0x123; x = 0o123; x = 0b1";
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
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(291_f64))),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(83_f64))),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::Number(NumberLiteral::Value(1_f64))),
            Token::EOF
        ]
    );
}

#[test]
fn test_invalid_hex_value() {
    let input = "x = 0x12l;";
    let mut lexer = Lexer::new(input.to_string());
    lexer.scan_all_tokens();
    assert_ne!(lexer.errors, vec![]);
}

#[test]
fn test_escape_characters_in_string() {
    let input = r#"x = 'She\'s good.'; x = "hell\"o"; x = `hell\`o`"#;
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
            Token::Literal(LiteralToken::String("She's good.".to_string())),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Literal(LiteralToken::String("hell\"o".to_string())),
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("x".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::Delimiter(DelimiterToken::Tilde),
            Token::Literal(LiteralToken::String("hell`o".to_string())),
            Token::Delimiter(DelimiterToken::Tilde),
            Token::EOF
        ]
    );
}

#[test]
fn test_regex() {
    let input = "x = /ab2+c/; y = /abc/gi.collect()";
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
            Token::RegExp {
                pattern: "ab2+c".to_owned(),
                flags: "".to_owned()
            },
            Token::Delimiter(DelimiterToken::Semicolon),
            Token::Identifier("y".to_string()),
            Token::Operator(OperatorToken::EqualTo),
            Token::RegExp {
                pattern: "abc".to_owned(),
                flags: "gi".to_owned()
            },
            Token::Delimiter(DelimiterToken::Dot),
            Token::Identifier("collect".to_string()),
            Token::Delimiter(DelimiterToken::OpenParen),
            Token::Delimiter(DelimiterToken::CloseParen),
            Token::EOF
        ]
    );
}

#[test]
fn test_invalid_regex() {
    let inputs = vec![
        "x = //", // Empty regex
        "x = /*/", // Invalid first char
        "x = ///", // Invalid first char
        "x = /abc", // Unclosed regex
    ];

    for input in inputs {
        let mut lexer = Lexer::new(input.to_string());
        lexer.scan_all_tokens();
        assert_ne!(lexer.errors, vec![], "Input '{}' should produce errors", input);
    }
}
