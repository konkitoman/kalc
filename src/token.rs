use std::ops::{Add, Div, Mul, Sub};

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

    Group(Vec<Token>),

    SAdd,
    SDiv,
    SSub,
    SMul,

    SSin,
    SCos,

    SGroupBeagin,
    SGroupEnd,

    Inf,
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
                | Token::Cos(_),
        )
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
                    *self = t1.as_ref().clone() / t2.as_ref().clone()
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
