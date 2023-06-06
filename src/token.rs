use std::ops::Add;

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

impl Token {
    pub fn is_end(&self) -> bool {
        match self {
            Token::Add(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Div(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Sub(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Mul(t0, t1) => t0.is_num() && t1.is_num(),
            Token::Group(tokens) => {
                let a: usize = tokens.iter().map(|token| token.is_num() as usize).sum();
                a == tokens.len()
            }
            _ => false,
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            Token::I32(_) | Token::I64(_) | Token::Intiger(_) | Token::Decimal(_, _) => true,
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
            Token::Div(_, _) => {}
            Token::Sub(_, _) => {}
            Token::Mul(_, _) => {}
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
            Token::I32(s) => match rhs {
                Token::I32(s1) => {
                    if s < i32::MAX / 2 && s1 < i32::MAX / 2 {
                        Token::I32(s + s1)
                    } else {
                        Token::I64(s as i64 + s1 as i64)
                    }
                }
                Token::I64(s1) => Token::I64(s as i64 + s1),
                Token::Intiger(_) => todo!(),
                Token::Decimal(_, _) => todo!(),
                _ => {
                    panic!()
                }
            },
            Token::I64(s) => match rhs {
                Token::I32(s1) => Token::I64(s + s1 as i64),
                Token::I64(s1) => Token::I64(s + s1),
                Token::Intiger(_) => todo!(),
                Token::Decimal(_, _) => todo!(),
                _ => {
                    panic!()
                }
            },
            Token::Intiger(_) => todo!(),
            Token::Decimal(_, _) => todo!(),
            _ => {
                panic!()
            }
        }
    }
}
