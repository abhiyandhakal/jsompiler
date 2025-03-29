use jsompiler_lexer::Lexer;
use jsompiler_parser::Parser;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./test.js");
    if file.is_err() {
        return;
    }
    let file = file.unwrap();
    println!("{file}");

    let mut lexer = Lexer::new(file);
    lexer.scan_all_tokens();
    println!("{:#?}", lexer.tokens);
    println!("{:#?}", lexer.errors);

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    println!("{:#?}", parser.ast);
    println!("{:#?}", parser.errors);
}
