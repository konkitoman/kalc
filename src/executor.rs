use crate::{lexer::Lexer, token::Token};

pub struct Executor {
    group: Token,
}

impl Executor {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            group: Token::Group(lexer.tokens),
        }
    }

    pub fn execute(&mut self) {
        let mut depths = Self::enter(&self.group);
        depths.sort_by(|a, b| a.1.cmp(&b.1));
        depths.reverse();
        for depth in depths {
            Self::calculate(&mut self.group, depth.0)
        }
    }

    pub fn get_i64(&self) -> i64 {
        match self.group {
            Token::I(num) => num,
            Token::F(num) => num as i64,
            _ => {
                eprintln!("You need to call execute first!");
                0
            }
        }
    }

    pub fn get_f64(&self) -> f64 {
        match self.group {
            Token::I(num) => num as f64,
            Token::F(num) => num,
            _ => {
                eprintln!("You need to call execute first!");
                0.0
            }
        }
    }

    pub fn enter(token: &Token) -> Vec<(Vec<usize>, usize)> {
        let mut depth = vec![(vec![], 0)];
        let mut tokens = Vec::new();
        match token {
            Token::Add(t1, t2) | Token::Div(t1, t2) | Token::Sub(t1, t2) | Token::Mul(t1, t2) => {
                tokens.push((t1.as_ref(), 0, 0));
                tokens.push((t2.as_ref(), 1, 0));
            }

            Token::Group(group) => {
                for (i, token) in group.iter().enumerate() {
                    tokens.push((token, i, 1))
                }
            }
            _ => {}
        }
        for (token, i, r) in tokens {
            for (mut d, real) in Self::enter(token) {
                d.push(i);
                if token.is_num() {
                    d.remove(0);
                }
                depth.push((d, r + real));
            }
        }
        depth
    }

    fn calculate(token: &mut Token, mut depths: Vec<usize>) {
        let depth = depths.pop();
        if let Some(depth) = depth {
            match token {
                Token::Add(t0, t1)
                | Token::Div(t0, t1)
                | Token::Sub(t0, t1)
                | Token::Mul(t0, t1) => {
                    if depth == 0 {
                        Self::calculate(t0, depths)
                    } else {
                        Self::calculate(t1, depths)
                    }
                }
                Token::Group(tokens) => {
                    if let Some(token) = tokens.get_mut(depth) {
                        Self::calculate(token, depths)
                    }
                }
                _ => {}
            }
        } else {
            token.calculate();
        }
    }
}
