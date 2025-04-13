use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::symbol::{Lexeme, Token, lexeme};

#[derive(Debug, PartialEq, Clone)]
pub enum JSXToken {
    // Tag delimiters
    LessThan,         // <
    GreaterThan,      // >
    LessThanSlash,    // </
    SlashGreaterThan, // />
    Slash,            // /
    Colon,            // :
    Dot,              // .

    // Braces for expressions or spreads
    LeftBrace,  // {
    RightBrace, // }

    // Spread operator
    Ellipsis, // ...

    // Equals sign for attributes
    Equals, // =

    // Strings in attributes
    StringLiteral(String), // "value"

    // JSX identifiers
    Identifier(String), // div, MyComponent, etc.

    // Text between JSX tags (excluding angle brackets, curly braces)
    Text(String), // hello world

    // Embedded expression start (handled in parser/lexer state)
    ExpressionStart, // {
    ExpressionEnd,   // }

    // HTML character references (parsed or passed raw)
    HtmlEntity(String), // &amp;

    // Fragments
    FragmentStart, // <>
    FragmentEnd,   // </>

    // End of JSX input
    EOF,
}

lazy_static! {
    pub static ref JSX_SYMBOLS: HashMap<&'static str, Lexeme> = {
        let mut m = HashMap::new();

        // Keywords
        m.insert("<", lexeme("<".to_string(), Token::JSX(JSXToken::LessThan)));
        m.insert(">", lexeme(">".to_string(), Token::JSX(JSXToken::GreaterThan)));
        m.insert("</", lexeme("</".to_string(), Token::JSX(JSXToken::LessThanSlash)));
        m.insert("/>", lexeme("/>".to_string(), Token::JSX(JSXToken::SlashGreaterThan)));
        m.insert("/", lexeme("/".to_string(), Token::JSX(JSXToken::Slash)));
        m.insert(":", lexeme(":".to_string(), Token::JSX(JSXToken::Colon)));
        m.insert(".", lexeme(".".to_string(), Token::JSX(JSXToken::Dot)));
        m.insert("{", lexeme("{".to_string(), Token::JSX(JSXToken::LeftBrace)));
        m.insert("}", lexeme("}".to_string(), Token::JSX(JSXToken::RightBrace)));
        m.insert("=", lexeme("=".to_string(), Token::JSX(JSXToken::Equals)));
        m.insert("...", lexeme("...".to_string(), Token::JSX(JSXToken::Ellipsis)));
        m.insert("<>", lexeme("<>".to_string(), Token::JSX(JSXToken::FragmentStart)));
        m.insert("</>", lexeme("</>".to_string(), Token::JSX(JSXToken::FragmentEnd)));

        m
    };
}
