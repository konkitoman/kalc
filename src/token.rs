#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    I32(i32),
    I64(i64),
    Intiger(Vec<Token>),
    Decimal(Box<Token>, Box<Token>),

    Add(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
    Sub(Box<Token>, Box<Token>),
    Mul(Box<Token>, Box<Token>),

    Group(Vec<Token>),

    SAdd,
    SDiv,
    SSub,
    SMul,

    SGroupBeagin,
    SGroupEnd,

    Inf,
}
