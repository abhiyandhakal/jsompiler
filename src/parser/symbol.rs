use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub text: String,
    pub len: usize,
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
    PlusEqual,       // +=
    MinusEqual,      // -=
    AsteriskEqual,   // *=
    SlashEqual,      // /=
    PercentEqual,    // %=
    Increment,       // ++
    Decrement,       // --
    QuestionMark,    // ?
    Colon,           // :
    NullishCoalesce, // ??
    OptionalChain,   // ?.
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralToken {
    Number(String),
    String(StringLiteral),
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringLiteral {
    Regular(String),  // 'string' or "string"
    Template(String), // `string`
}

#[derive(Debug, PartialEq, Clone)]
pub enum CommentToken {
    Line(String),
    Block(String),
}

pub fn lexeme(text: String, token: Token) -> Lexeme {
    Lexeme {
        text: text.clone(),
        len: text.len(),
        token,
    }
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<&'static str, Lexeme> = {
        let mut m = HashMap::new();

        // Keywords
        m.insert("const", lexeme("const".to_string(), Token::Keyword(KeywordToken::Const)));
        m.insert("let", lexeme("let".to_string(), Token::Keyword(KeywordToken::Let)));
        m.insert("var", lexeme("var".to_string(), Token::Keyword(KeywordToken::Var)));
        m.insert("function", lexeme("function".to_string(), Token::Keyword(KeywordToken::Function)));
        m.insert("return", lexeme("return".to_string(), Token::Keyword(KeywordToken::Return)));
        m.insert("if", lexeme("if".to_string(), Token::Keyword(KeywordToken::If)));
        m.insert("else", lexeme("else".to_string(), Token::Keyword(KeywordToken::Else)));
        m.insert("while", lexeme("while".to_string(), Token::Keyword(KeywordToken::While)));
        m.insert("for", lexeme("for".to_string(), Token::Keyword(KeywordToken::For)));
        m.insert("break", lexeme("break".to_string(), Token::Keyword(KeywordToken::Break)));
        m.insert("continue", lexeme("continue".to_string(), Token::Keyword(KeywordToken::Continue)));

        // Operators
        m.insert("+", lexeme("+".to_string(), Token::Operator(OperatorToken::Plus)));
        m.insert("-", lexeme("-".to_string(), Token::Operator(OperatorToken::Minus)));
        m.insert("*", lexeme("*".to_string(), Token::Operator(OperatorToken::Asterisk)));
        m.insert("/", lexeme("/".to_string(), Token::Operator(OperatorToken::Slash)));
        m.insert("%", lexeme("%".to_string(), Token::Operator(OperatorToken::Percent)));
        m.insert("=", lexeme("=".to_string(), Token::Operator(OperatorToken::EqualTo)));
        m.insert("==", lexeme("==".to_string(), Token::Operator(OperatorToken::DoubleEqual)));
        m.insert("===", lexeme("===".to_string(), Token::Operator(OperatorToken::StrictEqual)));
        m.insert("!=", lexeme("!=".to_string(), Token::Operator(OperatorToken::NotEqual)));
        m.insert("!==", lexeme("!==".to_string(), Token::Operator(OperatorToken::StrictNotEqual)));
        m.insert(">", lexeme(">".to_string(), Token::Operator(OperatorToken::Greater)));
        m.insert("<", lexeme("<".to_string(), Token::Operator(OperatorToken::Less)));
        m.insert(">=", lexeme(">=".to_string(), Token::Operator(OperatorToken::GreaterEqual)));
        m.insert("<=", lexeme("<=".to_string(), Token::Operator(OperatorToken::LessEqual)));
        m.insert("&&", lexeme("&&".to_string(), Token::Operator(OperatorToken::And)));
        m.insert("||", lexeme("||".to_string(), Token::Operator(OperatorToken::Or)));
        m.insert("!", lexeme("!".to_string(), Token::Operator(OperatorToken::Not)));
        m.insert("=>", lexeme("=>".to_string(), Token::Operator(OperatorToken::Arrow)));

        // Delimiters
        m.insert("(", lexeme("(".to_string(), Token::Delimiter(DelimiterToken::OpenParen)));
        m.insert(")", lexeme(")".to_string(), Token::Delimiter(DelimiterToken::CloseParen)));
        m.insert("{", lexeme("{".to_string(), Token::Delimiter(DelimiterToken::OpenBrace)));
        m.insert("}", lexeme("}".to_string(), Token::Delimiter(DelimiterToken::CloseBrace)));
        m.insert("[", lexeme("[".to_string(), Token::Delimiter(DelimiterToken::OpenBracket)));
        m.insert("]", lexeme("]".to_string(), Token::Delimiter(DelimiterToken::CloseBracket)));
        m.insert(";", lexeme(";".to_string(), Token::Delimiter(DelimiterToken::Semicolon)));
        m.insert(",", lexeme(",".to_string(), Token::Delimiter(DelimiterToken::Comma)));
        m.insert(".", lexeme(".".to_string(), Token::Delimiter(DelimiterToken::Dot)));

        // Comments (not stored in symbol table, but used in lexer)
        m.insert("//", lexeme("//".to_string(), Token::Comment(CommentToken::Line("//".to_string()))));
        m.insert("/*", lexeme("/*".to_string(), Token::Comment(CommentToken::Block("/*".to_string()))));

        m // return
    };
}
