use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub len: usize,
    pub token: Token,
    pub group: Group,
}

#[derive(Debug)]
pub enum Group {
    Keyword,
    Operator,
    Identifier,
    Delimiter,
    Literal,
    Comment,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Const,
    Let,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    Break,
    Continue,

    // Operators
    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Percent,        // %
    EqualTo,        // =
    DoubleEqual,    // ==
    StrictEqual,    // ===
    NotEqual,       // !=
    StrictNotEqual, // !==
    Greater,        // >
    Less,           // <
    GreaterEqual,   // >=
    LessEqual,      // <=
    And,            // &&
    Or,             // ||
    Not,            // !
    Arrow,          // =>

    // Delimiters
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .

    // Literals
    Number, // 123, 3.14
    String, // "hello", 'world', `template`

    // Comments
    LineComment,  // // This is a comment
    BlockComment, // /* This is a block comment */

    // End of file
    EOF,
}

pub fn symbol(name: String, token: Token, group: Group) -> Symbol {
    Symbol {
        name: name.clone(),
        len: name.len(),
        token,
        group,
    }
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<&'static str, Symbol> = {
        let mut m = HashMap::new();

        // Keywords
        m.insert("const", symbol("const".to_string(), Token::Const, Group::Keyword));
        m.insert("let", symbol("let".to_string(), Token::Let, Group::Keyword));
        m.insert("var", symbol("var".to_string(), Token::Var, Group::Keyword));
        m.insert("function", symbol("function".to_string(), Token::Function, Group::Keyword));
        m.insert("return", symbol("return".to_string(), Token::Return, Group::Keyword));
        m.insert("if", symbol("if".to_string(), Token::If, Group::Keyword));
        m.insert("else", symbol("else".to_string(), Token::Else, Group::Keyword));
        m.insert("while", symbol("while".to_string(), Token::While, Group::Keyword));
        m.insert("for", symbol("for".to_string(), Token::For, Group::Keyword));
        m.insert("break", symbol("break".to_string(), Token::Break, Group::Keyword));
        m.insert("continue", symbol("continue".to_string(), Token::Continue, Group::Keyword));

        // Operators
        m.insert("+", symbol("+".to_string(), Token::Plus, Group::Operator));
        m.insert("-", symbol("-".to_string(), Token::Minus, Group::Operator));
        m.insert("*", symbol("*".to_string(), Token::Asterisk, Group::Operator));
        m.insert("/", symbol("/".to_string(), Token::Slash, Group::Operator));
        m.insert("%", symbol("%".to_string(), Token::Percent, Group::Operator));
        m.insert("=", symbol("=".to_string(), Token::EqualTo, Group::Operator));
        m.insert("==", symbol("==".to_string(), Token::DoubleEqual, Group::Operator));
        m.insert("===", symbol("===".to_string(), Token::StrictEqual, Group::Operator));
        m.insert("!=", symbol("!=".to_string(), Token::NotEqual, Group::Operator));
        m.insert("!==", symbol("!==".to_string(), Token::StrictNotEqual, Group::Operator));
        m.insert(">", symbol(">".to_string(), Token::Greater, Group::Operator));
        m.insert("<", symbol("<".to_string(), Token::Less, Group::Operator));
        m.insert(">=", symbol(">=".to_string(), Token::GreaterEqual, Group::Operator));
        m.insert("<=", symbol("<=".to_string(), Token::LessEqual, Group::Operator));
        m.insert("&&", symbol("&&".to_string(), Token::And, Group::Operator));
        m.insert("||", symbol("||".to_string(), Token::Or, Group::Operator));
        m.insert("!", symbol("!".to_string(), Token::Not, Group::Operator));
        m.insert("=>", symbol("=>".to_string(), Token::Arrow, Group::Operator));

        // Delimiters
        m.insert("(", symbol("(".to_string(), Token::OpenParen, Group::Delimiter));
        m.insert(")", symbol(")".to_string(), Token::CloseParen, Group::Delimiter));
        m.insert("{", symbol("{".to_string(), Token::OpenBrace, Group::Delimiter));
        m.insert("}", symbol("}".to_string(), Token::CloseBrace, Group::Delimiter));
        m.insert("[", symbol("[".to_string(), Token::OpenBracket, Group::Delimiter));
        m.insert("]", symbol("]".to_string(), Token::CloseBracket, Group::Delimiter));
        m.insert(";", symbol(";".to_string(), Token::Semicolon, Group::Delimiter));
        m.insert(",", symbol(",".to_string(), Token::Comma, Group::Delimiter));
        m.insert(".", symbol(".".to_string(), Token::Dot, Group::Delimiter));

        // Comments (not stored in symbol table, but used in lexer)
        m.insert("//", symbol("//".to_string(), Token::LineComment, Group::Comment));
        m.insert("/*", symbol("/*".to_string(), Token::BlockComment, Group::Comment));

        m // return
    };
}
