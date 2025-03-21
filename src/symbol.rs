use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Symbol {
    pub name: String,
    pub len: usize,
    pub token: Token,
    pub group: Group,
}

pub enum Group {
    Keyword,
    Operator,
    Identifier,
    Delimiter,
}

pub enum Token {
    Const,
    Let,
    Var,
    Function,

    Plus,
    EqualTo,

    Semicolon,
}

pub fn symbol(name: &str, token: Token, group: Group) -> Symbol {
    Symbol {
        name: name.to_string(),
        len: name.len(),
        token,
        group,
    }
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<&'static str, Symbol> = {
        let mut m = HashMap::new();
        m.insert("const", symbol("const", Token::Const, Group::Keyword));
        m.insert("let", symbol("let", Token::Let, Group::Keyword));
        m.insert("var", symbol("var", Token::Var, Group::Keyword));
        m.insert(
            "function",
            symbol("function", Token::Function, Group::Keyword),
        );
        m.insert("+", symbol("+", Token::Plus, Group::Operator));
        m.insert("=", symbol("=", Token::EqualTo, Group::Operator));
        return m;
    };
}
