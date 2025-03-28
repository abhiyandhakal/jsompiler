use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
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
    NewLine,      // \n
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

        // Additional keywords
        m.insert("class", lexeme("class".to_string(), Token::Keyword(KeywordToken::Class)));
        m.insert("new", lexeme("new".to_string(), Token::Keyword(KeywordToken::New)));
        m.insert("this", lexeme("this".to_string(), Token::Keyword(KeywordToken::This)));
        m.insert("super", lexeme("super".to_string(), Token::Keyword(KeywordToken::Super)));
        m.insert("import", lexeme("import".to_string(), Token::Keyword(KeywordToken::Import)));
        m.insert("export", lexeme("export".to_string(), Token::Keyword(KeywordToken::Export)));
        m.insert("default", lexeme("default".to_string(), Token::Keyword(KeywordToken::Default)));
        m.insert("try", lexeme("try".to_string(), Token::Keyword(KeywordToken::Try)));
        m.insert("catch", lexeme("catch".to_string(), Token::Keyword(KeywordToken::Catch)));
        m.insert("finally", lexeme("finally".to_string(), Token::Keyword(KeywordToken::Finally)));
        m.insert("throw", lexeme("throw".to_string(), Token::Keyword(KeywordToken::Throw)));

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

        // Additional operators
        m.insert("+=", lexeme("+=".to_string(), Token::Operator(OperatorToken::PlusEqual)));
        m.insert("-=", lexeme("-=".to_string(), Token::Operator(OperatorToken::MinusEqual)));
        m.insert("*=", lexeme("*=".to_string(), Token::Operator(OperatorToken::AsteriskEqual)));
        m.insert("/=", lexeme("/=".to_string(), Token::Operator(OperatorToken::SlashEqual)));
        m.insert("%=", lexeme("%=".to_string(), Token::Operator(OperatorToken::PercentEqual)));
        m.insert("++", lexeme("++".to_string(), Token::Operator(OperatorToken::Increment)));
        m.insert("--", lexeme("--".to_string(), Token::Operator(OperatorToken::Decrement)));
        m.insert("?", lexeme("?".to_string(), Token::Operator(OperatorToken::QuestionMark)));
        m.insert(":", lexeme(":".to_string(), Token::Operator(OperatorToken::Colon)));
        m.insert("??", lexeme("??".to_string(), Token::Operator(OperatorToken::NullishCoalesce)));
        m.insert("?.", lexeme("?.".to_string(), Token::Operator(OperatorToken::OptionalChain)));

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

impl std::fmt::Display for OperatorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            // Arithmetic
            OperatorToken::Plus => "+",
            OperatorToken::Minus => "-",
            OperatorToken::Asterisk => "*",
            OperatorToken::Slash => "/",
            OperatorToken::Percent => "%",

            // Assignment
            OperatorToken::EqualTo => "=",

            // Comparison
            OperatorToken::DoubleEqual => "==",
            OperatorToken::StrictEqual => "===",
            OperatorToken::NotEqual => "!=",
            OperatorToken::StrictNotEqual => "!==",
            OperatorToken::Greater => ">",
            OperatorToken::Less => "<",
            OperatorToken::GreaterEqual => ">=",
            OperatorToken::LessEqual => "<=",

            // Logical
            OperatorToken::And => "&&",
            OperatorToken::Or => "||",
            OperatorToken::Not => "!",

            // Other
            OperatorToken::Arrow => "=>",

            // Additional Operators
            OperatorToken::PlusEqual => "+=",
            OperatorToken::MinusEqual => "-=",
            OperatorToken::AsteriskEqual => "*=",
            OperatorToken::SlashEqual => "/=",
            OperatorToken::PercentEqual => "%=",
            OperatorToken::Increment => "++",
            OperatorToken::Decrement => "--",
            OperatorToken::QuestionMark => "?",
            OperatorToken::Colon => ":",
            OperatorToken::NullishCoalesce => "??",
            OperatorToken::OptionalChain => "?.",
        };

        write!(f, "{}", symbol)
    }
}
