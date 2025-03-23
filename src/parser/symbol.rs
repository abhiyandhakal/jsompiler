use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub len: usize,
    pub token: Token,
    pub group: TokenGroup,
}

#[derive(Debug)]
pub enum TokenGroup {
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
    Keyword(KeywordToken),
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Literal(LiteralToken),
    Comment(CommentToken),
    Identifier(String),
    WhiteSpace(String),
    RegExp { pattern: String, flags: String },
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum KeywordToken {
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
    // Additional keywords
    Class,
    New,
    This,
    Super,
    Import,
    Export,
    Default,
    Try,
    Catch,
    Finally,
    Throw,
}

#[derive(Debug, PartialEq)]
pub enum OperatorToken {
    // Arithmetic
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    // Assignment
    EqualTo,
    // Comparison
    DoubleEqual,
    StrictEqual,
    NotEqual,
    StrictNotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    // Logical
    And,
    Or,
    Not,
    // Other
    Arrow,
    // Additional operators
    PlusEqual,      // +=
    MinusEqual,     // -=
    AsteriskEqual,  // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    Increment,      // ++
    Decrement,      // --
    QuestionMark,   // ?
    Colon,          // :
    NullishCoalesce,// ??
    OptionalChain,  // ?.
}

#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
}

#[derive(Debug, PartialEq)]
pub enum LiteralToken {
    Number(String),
    String(StringLiteral),
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug, PartialEq)]
pub enum StringLiteral {
    Single(String),   // 'string'
    Double(String),   // "string"
    Template(String), // `string`
}

#[derive(Debug, PartialEq)]
pub enum CommentToken {
    Line(String),
    Block(String),
}

pub fn symbol(name: String, token: Token, group: TokenGroup) -> Symbol {
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
        m.insert("const", symbol("const".to_string(), Token::Keyword(KeywordToken::Const), TokenGroup::Keyword));
        m.insert("let", symbol("let".to_string(), Token::Keyword(KeywordToken::Let), TokenGroup::Keyword));
        m.insert("var", symbol("var".to_string(), Token::Keyword(KeywordToken::Var), TokenGroup::Keyword));
        m.insert("function", symbol("function".to_string(), Token::Keyword(KeywordToken::Function), TokenGroup::Keyword));
        m.insert("return", symbol("return".to_string(), Token::Keyword(KeywordToken::Return), TokenGroup::Keyword));
        m.insert("if", symbol("if".to_string(), Token::Keyword(KeywordToken::If), TokenGroup::Keyword));
        m.insert("else", symbol("else".to_string(), Token::Keyword(KeywordToken::Else), TokenGroup::Keyword));
        m.insert("while", symbol("while".to_string(), Token::Keyword(KeywordToken::While), TokenGroup::Keyword));
        m.insert("for", symbol("for".to_string(), Token::Keyword(KeywordToken::For), TokenGroup::Keyword));
        m.insert("break", symbol("break".to_string(), Token::Keyword(KeywordToken::Break), TokenGroup::Keyword));
        m.insert("continue", symbol("continue".to_string(), Token::Keyword(KeywordToken::Continue), TokenGroup::Keyword));

        // Operators
        m.insert("+", symbol("+".to_string(), Token::Operator(OperatorToken::Plus), TokenGroup::Operator));
        m.insert("-", symbol("-".to_string(), Token::Operator(OperatorToken::Minus), TokenGroup::Operator));
        m.insert("*", symbol("*".to_string(), Token::Operator(OperatorToken::Asterisk), TokenGroup::Operator));
        m.insert("/", symbol("/".to_string(), Token::Operator(OperatorToken::Slash), TokenGroup::Operator));
        m.insert("%", symbol("%".to_string(), Token::Operator(OperatorToken::Percent), TokenGroup::Operator));
        m.insert("=", symbol("=".to_string(), Token::Operator(OperatorToken::EqualTo), TokenGroup::Operator));
        m.insert("==", symbol("==".to_string(), Token::Operator(OperatorToken::DoubleEqual), TokenGroup::Operator));
        m.insert("===", symbol("===".to_string(), Token::Operator(OperatorToken::StrictEqual), TokenGroup::Operator));
        m.insert("!=", symbol("!=".to_string(), Token::Operator(OperatorToken::NotEqual), TokenGroup::Operator));
        m.insert("!==", symbol("!==".to_string(), Token::Operator(OperatorToken::StrictNotEqual), TokenGroup::Operator));
        m.insert(">", symbol(">".to_string(), Token::Operator(OperatorToken::Greater), TokenGroup::Operator));
        m.insert("<", symbol("<".to_string(), Token::Operator(OperatorToken::Less), TokenGroup::Operator));
        m.insert(">=", symbol(">=".to_string(), Token::Operator(OperatorToken::GreaterEqual), TokenGroup::Operator));
        m.insert("<=", symbol("<=".to_string(), Token::Operator(OperatorToken::LessEqual), TokenGroup::Operator));
        m.insert("&&", symbol("&&".to_string(), Token::Operator(OperatorToken::And), TokenGroup::Operator));
        m.insert("||", symbol("||".to_string(), Token::Operator(OperatorToken::Or), TokenGroup::Operator));
        m.insert("!", symbol("!".to_string(), Token::Operator(OperatorToken::Not), TokenGroup::Operator));
        m.insert("=>", symbol("=>".to_string(), Token::Operator(OperatorToken::Arrow), TokenGroup::Operator));

        // Delimiters
        m.insert("(", symbol("(".to_string(), Token::Delimiter(DelimiterToken::OpenParen), TokenGroup::Delimiter));
        m.insert(")", symbol(")".to_string(), Token::Delimiter(DelimiterToken::CloseParen), TokenGroup::Delimiter));
        m.insert("{", symbol("{".to_string(), Token::Delimiter(DelimiterToken::OpenBrace), TokenGroup::Delimiter));
        m.insert("}", symbol("}".to_string(), Token::Delimiter(DelimiterToken::CloseBrace), TokenGroup::Delimiter));
        m.insert("[", symbol("[".to_string(), Token::Delimiter(DelimiterToken::OpenBracket), TokenGroup::Delimiter));
        m.insert("]", symbol("]".to_string(), Token::Delimiter(DelimiterToken::CloseBracket), TokenGroup::Delimiter));
        m.insert(";", symbol(";".to_string(), Token::Delimiter(DelimiterToken::Semicolon), TokenGroup::Delimiter));
        m.insert(",", symbol(",".to_string(), Token::Delimiter(DelimiterToken::Comma), TokenGroup::Delimiter));
        m.insert(".", symbol(".".to_string(), Token::Delimiter(DelimiterToken::Dot), TokenGroup::Delimiter));

        // Comments (not stored in symbol table, but used in lexer)
        m.insert("//", symbol("//".to_string(), Token::Comment(CommentToken::Line("//".to_string())), TokenGroup::Comment));
        m.insert("/*", symbol("/*".to_string(), Token::Comment(CommentToken::Block("/*".to_string())), TokenGroup::Comment));

        m // return
    };
}
