mod executor;
mod lexer;
mod token;

pub use executor::Executor;
pub use lexer::Lexer;
pub use token::Token;

pub fn kalc(formula: &str) -> Result<Executor, String> {
    let mut lexer = Lexer::default();
    lexer.parse(formula)?;
    let mut executor = Executor::new(lexer);
    executor.execute();
    Ok(executor)
}

pub fn kalc_i64(formula: &str) -> Result<i64, String> {
    Ok(kalc(formula)?.get_i64())
}

pub fn kalc_f64(formula: &str) -> Result<f64, String> {
    Ok(kalc(formula)?.get_f64())
}
