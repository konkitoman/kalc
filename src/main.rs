use inf::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::default();
    let args = std::env::args().skip(1).collect::<String>();
    println!("Args: {args}");
    lexer.parse(&args).unwrap();
    println!("Lexer: {lexer:?}");
}
