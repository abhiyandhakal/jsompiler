pub mod parser;

#[derive(Debug)]
pub enum ErrorKind {
    LexerError,
    SyntaxError,
    SemanticError,
}

#[derive(Debug)]
pub struct Error {
    pub error_kind: ErrorKind,
    pub message: String,
    pub line_number: usize,
    pub pos: usize,
}
