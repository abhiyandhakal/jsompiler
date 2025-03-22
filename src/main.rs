use jsompiler::lexer::Lexer;

fn main() {
    println!("Hello, world!");
    Lexer::new("const val = 01+23".to_string()).scan_all_tokens();
}
