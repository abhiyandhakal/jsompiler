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
    ContextualKeyword(ContextualKeywordToken),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordToken {
    Switch,
    Null,
    Const,
    Var,
    Function,
    Void,
    Return,
    Typeof,
    If,
    In,
    Else,
    Instanceof,
    While,
    For,
    Break,
    Continue,
    Debugger,
    Case,
    Class,
    Delete,
    Do,
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
    Enum,
    Extends,
    Exports,
    False,
    True,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContextualKeywordToken {
    Get,
    Set,
    Meta,
    Target,
    Async,
    Await,
    From,
    As,
    Of,
    Yield,
    Static,
    With,
    Implements,
    Let,
    Interface,
    Package,
    Private,
    Protected,
    Public,
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
    // Bitwise logical
    BitwiseNot, // ~
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    // Shift
    LeftShift,          // <<
    RightShift,         // >>
    UnsignedRightShift, // >>>
    // Other
    Arrow,
    Spread, // ...
    // Additional operators
    PlusEqual,        // +=
    MinusEqual,       // -=
    AsteriskEqual,    // *=
    AsteriskAsterisk, // **
    SlashEqual,       // /=
    PercentEqual,     // %=
    Increment,        // ++
    Decrement,        // --
    QuestionMark,     // ?
    Colon,            // :
    NullishCoalesce,  // ??
    OptionalChain,    // ?.
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
    Tilde,        // `
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralToken {
    Number(NumberLiteral),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberLiteral {
    Value(f64),
    NaN,
    Infinity,
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
        m.insert("var", lexeme("var".to_string(), Token::Keyword(KeywordToken::Var)));
        m.insert("function", lexeme("function".to_string(), Token::Keyword(KeywordToken::Function)));
        m.insert("void", lexeme("void".to_string(), Token::Keyword(KeywordToken::Void)));
        m.insert("typeof", lexeme("typeof".to_string(), Token::Keyword(KeywordToken::Typeof)));
        m.insert("return", lexeme("return".to_string(), Token::Keyword(KeywordToken::Return)));
        m.insert("if", lexeme("if".to_string(), Token::Keyword(KeywordToken::If)));
        m.insert("else", lexeme("else".to_string(), Token::Keyword(KeywordToken::Else)));
        m.insert("instanceof", lexeme("instanceof".to_string(), Token::Keyword(KeywordToken::Instanceof)));
        m.insert("in", lexeme("in".to_string(), Token::Keyword(KeywordToken::In)));
        m.insert("while", lexeme("while".to_string(), Token::Keyword(KeywordToken::While)));
        m.insert("for", lexeme("for".to_string(), Token::Keyword(KeywordToken::For)));
        m.insert("do", lexeme("do".to_string(), Token::Keyword(KeywordToken::Do)));
        m.insert("true", lexeme("true".to_string(), Token::Keyword(KeywordToken::True)));
        m.insert("false", lexeme("false".to_string(), Token::Keyword(KeywordToken::False)));
        m.insert("exports", lexeme("exports".to_string(), Token::Keyword(KeywordToken::Exports)));
        m.insert("break", lexeme("break".to_string(), Token::Keyword(KeywordToken::Break)));
        m.insert("continue", lexeme("continue".to_string(), Token::Keyword(KeywordToken::Continue)));
        m.insert("switch", lexeme("switch".to_string(), Token::Keyword(KeywordToken::Switch)));
        m.insert("null", lexeme("null".to_string(), Token::Keyword(KeywordToken::Null)));
        m.insert("case", lexeme("case".to_string(), Token::Keyword(KeywordToken::Case)));
        m.insert("debugger", lexeme("debugger".to_string(), Token::Keyword(KeywordToken::Debugger)));
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
        m.insert("enum", lexeme("enum".to_string(), Token::Keyword(KeywordToken::Enum)));
        m.insert("extends", lexeme("extends".to_string(), Token::Keyword(KeywordToken::Extends)));
        m.insert("delete", lexeme("delete".to_string(), Token::Keyword(KeywordToken::Delete)));

        // Contextual keywords
        m.insert("let", lexeme("let".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Let)));
        m.insert("package", lexeme("package".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Package)));
        m.insert("interface", lexeme("interface".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Interface)));
        m.insert("get", lexeme("get".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Get)));
        m.insert("set", lexeme("set".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Set)));
        m.insert("public", lexeme("public".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Public)));
        m.insert("protected", lexeme("protected".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Protected)));
        m.insert("private", lexeme("private".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Private)));
        m.insert("meta", lexeme("meta".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Meta)));
        m.insert("target", lexeme("target".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Target)));
        m.insert("async", lexeme("async".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Async)));
        m.insert("await", lexeme("await".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Await)));
        m.insert("from", lexeme("from".to_string(), Token::ContextualKeyword(ContextualKeywordToken::From)));
        m.insert("as", lexeme("as".to_string(), Token::ContextualKeyword(ContextualKeywordToken::As)));
        m.insert("of", lexeme("of".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Of)));
        m.insert("yield", lexeme("yield".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Yield)));
        m.insert("static", lexeme("static".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Static)));
        m.insert("with", lexeme("with".to_string(), Token::ContextualKeyword(ContextualKeywordToken::With)));
        m.insert("implements", lexeme("implements".to_string(), Token::ContextualKeyword(ContextualKeywordToken::Implements)));

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
        m.insert("&", lexeme("&".to_string(), Token::Operator(OperatorToken::BitwiseAnd)));
        m.insert("|", lexeme("|".to_string(), Token::Operator(OperatorToken::BitwiseOr)));
        m.insert("~", lexeme("~".to_string(), Token::Operator(OperatorToken::BitwiseNot)));
        m.insert("^", lexeme("^".to_string(), Token::Operator(OperatorToken::BitwiseXor)));
        m.insert("<<", lexeme("<<".to_string(), Token::Operator(OperatorToken::LeftShift)));
        m.insert(">>", lexeme(">>".to_string(), Token::Operator(OperatorToken::RightShift)));
        m.insert(">>>", lexeme(">>>".to_string(), Token::Operator(OperatorToken::UnsignedRightShift)));
        m.insert("=>", lexeme("=>".to_string(), Token::Operator(OperatorToken::Arrow)));

        // Additional operators
        m.insert("+=", lexeme("+=".to_string(), Token::Operator(OperatorToken::PlusEqual)));
        m.insert("-=", lexeme("-=".to_string(), Token::Operator(OperatorToken::MinusEqual)));
        m.insert("*=", lexeme("*=".to_string(), Token::Operator(OperatorToken::AsteriskEqual)));
        m.insert("**", lexeme("**".to_string(), Token::Operator(OperatorToken::AsteriskAsterisk)));
        m.insert("/=", lexeme("/=".to_string(), Token::Operator(OperatorToken::SlashEqual)));
        m.insert("%=", lexeme("%=".to_string(), Token::Operator(OperatorToken::PercentEqual)));
        m.insert("++", lexeme("++".to_string(), Token::Operator(OperatorToken::Increment)));
        m.insert("--", lexeme("--".to_string(), Token::Operator(OperatorToken::Decrement)));
        m.insert("?", lexeme("?".to_string(), Token::Operator(OperatorToken::QuestionMark)));
        m.insert(":", lexeme(":".to_string(), Token::Operator(OperatorToken::Colon)));
        m.insert("??", lexeme("??".to_string(), Token::Operator(OperatorToken::NullishCoalesce)));
        m.insert("?.", lexeme("?.".to_string(), Token::Operator(OperatorToken::OptionalChain)));
        m.insert("...", lexeme("...".to_string(), Token::Operator(OperatorToken::Spread)));

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

        // Literal Types
        m.insert("undefined", lexeme("/*".to_string(), Token::Literal(LiteralToken::Undefined)));
        m.insert("null", lexeme("/*".to_string(), Token::Literal(LiteralToken::Null)));
        m.insert("NaN", lexeme("/*".to_string(), Token::Literal(LiteralToken::Number(NumberLiteral::NaN))));
        m.insert("Infinity", lexeme("/*".to_string(), Token::Literal(LiteralToken::Number(NumberLiteral::Infinity))));

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

            // Bitwise Logical
            OperatorToken::BitwiseAnd => "&",
            OperatorToken::BitwiseOr => "|",
            OperatorToken::BitwiseNot => "~",
            OperatorToken::BitwiseXor => "^",

            // Shift
            OperatorToken::LeftShift => "<<",
            OperatorToken::RightShift => ">>",
            OperatorToken::UnsignedRightShift => ">>>",

            // Other
            OperatorToken::Arrow => "=>",

            // Additional Operators
            OperatorToken::PlusEqual => "+=",
            OperatorToken::MinusEqual => "-=",
            OperatorToken::AsteriskEqual => "*=",
            OperatorToken::AsteriskAsterisk => "**",
            OperatorToken::SlashEqual => "/=",
            OperatorToken::PercentEqual => "%=",
            OperatorToken::Increment => "++",
            OperatorToken::Decrement => "--",
            OperatorToken::QuestionMark => "?",
            OperatorToken::Colon => ":",
            OperatorToken::NullishCoalesce => "??",
            OperatorToken::OptionalChain => "?.",
            OperatorToken::Spread => "...",
        };

        write!(f, "{}", symbol)
    }
}
