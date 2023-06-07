use kalc::{executor::Executor, lexer::Lexer};

fn main() {
    let mut lexer = Lexer::default();
    let args = std::env::args().skip(1).collect::<String>();
    println!("Args: {args}");
    lexer.parse(&args).unwrap();
    println!("Lexer: {lexer:?}");
    let mut executor = Executor::new(lexer);
    executor.execute();
    let resi = executor.get_i64();
    let resf = executor.get_f64();
    println!("Res i: {resi}");
    println!("Res f: {resf}");
}
