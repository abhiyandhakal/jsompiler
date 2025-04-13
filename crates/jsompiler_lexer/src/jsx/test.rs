#[allow(unused_imports)]
use super::symbol::JSXToken;
#[allow(unused_imports)]
use crate::{Lexer, symbol::Token};

#[test]
fn test_jsx_identifier_and_punctuation() {
    let input = "<$div_test></$div_test>";
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
            Token::JSX(JSXToken::LessThan),
            Token::JSX(JSXToken::Identifier("$div_test".to_string())),
            Token::JSX(JSXToken::GreaterThan),
            Token::JSX(JSXToken::LessThanSlash),
            Token::JSX(JSXToken::Identifier("$div_test".to_string())),
            Token::JSX(JSXToken::GreaterThan),
            Token::EOF
        ]
    );
}
