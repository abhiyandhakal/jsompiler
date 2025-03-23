use jsompiler::parser::lexer::Lexer;

fn main() {
    println!("Hello, world!");
    let mut lexer = Lexer::new("const val \"Hello\"".to_string());
    lexer.scan_all_tokens();
    println!("{:#?}", lexer.tokens)
}
