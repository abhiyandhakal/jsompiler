pub mod ast;
pub mod lexer;
pub mod parserr;

#[derive(Debug)]
pub enum ErrorKind {
    LexerError,
    SyntaxError,
    SemanticError,
    UnexpectedToken,
}

#[derive(Debug)]
pub struct Error {
    pub error_kind: ErrorKind,
    pub message: String,
    pub line_number: usize,
    pub pos: usize,
}

impl Error {
    pub fn new(error_kind: ErrorKind, message: String, line_number: usize, pos: usize) -> Self {
        Self {
            error_kind,
            message,
            line_number,
            pos,
        }
    }
}
