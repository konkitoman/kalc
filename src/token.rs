use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    I(i64),
    F(f64),

    Add(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
    Sub(Box<Token>, Box<Token>),
    Mul(Box<Token>, Box<Token>),

    Sin(Box<Token>),
    Cos(Box<Token>),
    Pow(Box<Token>, Box<Token>),
    Sqrt(Box<Token>),

    Group(Vec<Token>),

    SAdd,
    SDiv,
    SSub,
    SMul,

    SSin,
    SCos,
    SPow,
    SSqrt,

    SGroupBeagin,
    SGroupEnd,

    Inf,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::I(t0) => write!(f, "{t0}"),
            Token::F(t0) => write!(f, "{t0}"),
            Token::Add(t0, t1) => write!(f, "{t0} + {t1}"),
            Token::Div(t0, t1) => write!(f, "{t0} / {t1}"),
            Token::Sub(t0, t1) => write!(f, "{t0} - {t1}"),
            Token::Mul(t0, t1) => write!(f, "{t0} * {t1}"),
            Token::Sin(t0) => write!(f, "sin({t0})"),
            Token::Cos(t0) => write!(f, "cos({t0})"),
            Token::Pow(t0, t1) => write!(f, "pow({t0}, {t1})"),
            Token::Sqrt(t0) => write!(f, "sqrt({t0})"),
            Token::SAdd => f.write_str("+"),
            Token::SDiv => f.write_str("/"),
            Token::SSub => f.write_str("-"),
            Token::SMul => f.write_str("*"),
            Token::SSin => f.write_str("sin"),
            Token::SCos => f.write_str("cos"),
            Token::SPow => f.write_str("pow"),
            Token::SSqrt => f.write_str("sqrt"),
            Token::SGroupBeagin => f.write_str("("),
            Token::SGroupEnd => f.write_str(")"),
            Token::Inf => f.write_str("inf"),
            Token::Group(tokens) => {
                write!(f, "( ")?;
                for token in tokens {
                    write!(f, "{token} ")?
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

impl Token {
    pub fn a(a: Token, b: Token) -> Token {
        Token::Add(Box::new(a), Box::new(b))
    }
    pub fn s(a: Token, b: Token) -> Token {
        Token::Sub(Box::new(a), Box::new(b))
    }
    pub fn m(a: Token, b: Token) -> Token {
        Token::Mul(Box::new(a), Box::new(b))
    }
    pub fn d(a: Token, b: Token) -> Token {
        Token::Div(Box::new(a), Box::new(b))
    }

    pub fn sin(token: Token) -> Token {
        Token::Sin(Box::new(token))
    }
    pub fn cos(token: Token) -> Token {
        Token::Cos(Box::new(token))
    }

    pub fn pow(a: Token, b: Token) -> Token {
        Token::Pow(Box::new(a), Box::new(b))
    }
    pub fn sqrt(token: Token) -> Token {
        Token::Sqrt(Box::new(token))
    }
}

impl Token {
    pub fn is_end(&self) -> bool {
        match self {
            Token::Add(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Div(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Sub(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Mul(t0, t1) => t0.is_num() && t1.is_num(),

            Token::Sin(t0) => t0.is_num(),
            Token::Cos(t0) => t0.is_num(),

            Token::Pow(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Sqrt(t0) => t0.is_num(),

            Token::Group(tokens) => {
                let a: usize = tokens.iter().map(|token| token.is_num() as usize).sum();
                a == tokens.len()
            }
            _ => false,
        }
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Token::I(_) | Token::F(_))
    }

    pub fn is_calculabile(&self) -> bool {
        matches!(
            self,
            Token::I(_)
                | Token::F(_)
                | Token::Group(_)
                | Token::Add(_, _)
                | Token::Sub(_, _)
                | Token::Mul(_, _)
                | Token::Div(_, _)
                | Token::Sin(_)
                | Token::Cos(_)
                | Token::Pow(_, _)
                | Token::Sqrt(_)
        )
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Token::I(t) => *t == 0,
            Token::F(t) => *t == 0.0,
            _ => false,
        }
    }

    pub fn calculate(&mut self) {
        match self {
            Token::Add(t1, t2) => {
                if t1.is_num() && t2.is_num() {
                    *self = t1.as_ref().clone() + t2.as_ref().clone()
                }
            }
            Token::Div(t1, t2) => {
                if t1.is_num() && t2.is_num() {
                    if t2.is_zero() {
                        *self = Token::I(0)
                    } else {
                        *self = t1.as_ref().clone() / t2.as_ref().clone()
                    }
                }
            }
            Token::Sub(t1, t2) => {
                if t1.is_num() && t2.is_num() {
                    *self = t1.as_ref().clone() - t2.as_ref().clone()
                }
            }
            Token::Mul(t1, t2) => {
                if t1.is_num() && t2.is_num() {
                    *self = t1.as_ref().clone() * t2.as_ref().clone()
                }
            }

            Token::Sin(token) => match token.as_ref() {
                Token::I(t1) => *self = Token::F((*t1 as f64).sin()),
                Token::F(t1) => *self = Token::F(t1.sin()),
                _ => {}
            },
            Token::Cos(token) => match token.as_ref() {
                Token::I(t1) => *self = Token::F((*t1 as f64).cos()),
                Token::F(t1) => *self = Token::F(t1.cos()),
                _ => {}
            },

            Token::Pow(t1, t2) => match t1.as_ref() {
                Token::I(t1) => match t2.as_ref() {
                    Token::I(t2) => *self = Token::F((*t1 as f64).powf(*t2 as f64)),
                    Token::F(t2) => *self = Token::F((*t1 as f64).powf(*t2)),
                    _ => {}
                },
                Token::F(t1) => match t2.as_ref() {
                    Token::I(t2) => *self = Token::F(t1.powf(*t2 as f64)),
                    Token::F(t2) => *self = Token::F(t1.powf(*t2)),
                    _ => {}
                },
                _ => {}
            },
            Token::Sqrt(t1) => match t1.as_ref() {
                Token::I(t1) => *self = Token::F((*t1 as f64).sqrt()),
                Token::F(t1) => *self = Token::F(t1.sqrt()),
                _ => {}
            },

            Token::Group(tokens) => {
                if tokens.len() == 1 {
                    if let Some(token) = tokens.pop() {
                        *self = token;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Token::I(s) => match rhs {
                Token::I(s1) => Token::I(s + s1),
                Token::F(s1) => Token::F(s as f64 + s1),
                _ => {
                    panic!()
                }
            },
            Token::F(s) => match rhs {
                Token::I(s1) => Token::F(s + s1 as f64),
                Token::F(s1) => Token::F(s + s1),
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
    }
}

impl Sub for Token {
    type Output = Token;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Token::I(s) => match rhs {
                Token::I(s1) => Token::I(s - s1),
                Token::F(s1) => Token::F(s as f64 - s1),
                _ => {
                    panic!()
                }
            },
            Token::F(s) => match rhs {
                Token::I(s1) => Token::F(s - s1 as f64),
                Token::F(s1) => Token::F(s - s1),
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
    }
}
impl Mul for Token {
    type Output = Token;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Token::I(s) => match rhs {
                Token::I(s1) => Token::I(s * s1),
                Token::F(s1) => Token::F(s as f64 * s1),
                _ => {
                    panic!()
                }
            },
            Token::F(s) => match rhs {
                Token::I(s1) => Token::F(s * s1 as f64),
                Token::F(s1) => Token::F(s * s1),
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
    }
}
impl Div for Token {
    type Output = Token;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Token::I(s) => match rhs {
                Token::I(s1) => Token::I(s / s1),
                Token::F(s1) => Token::F(s as f64 / s1),
                _ => {
                    panic!()
                }
            },
            Token::F(s) => match rhs {
                Token::I(s1) => Token::F(s / s1 as f64),
                Token::F(s1) => Token::F(s / s1),
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
    }
}
