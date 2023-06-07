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
        depths.sort_by(|a, b| a.len().cmp(&b.len()));
        depths.reverse();
        println!("Depths: {:?}", depths);
        for depth in depths {
            Self::calculate(&mut self.group, depth)
        }
        println!("Result: {:?}", self.group)
    }

    pub fn enter(token: &Token) -> Vec<Vec<usize>> {
        let mut depth = vec![vec![]];
        let mut tokens = Vec::new();
        match token {
            Token::Add(t1, t2) | Token::Div(t1, t2) | Token::Sub(t1, t2) | Token::Mul(t1, t2) => {
                println!("OR: t1: {t1:?}, t2: {t2:?}");
                tokens.push((t1.as_ref(), 0));
                tokens.push((t2.as_ref(), 1));
            }

            Token::Group(group) => {
                for (i, token) in group.iter().enumerate() {
                    tokens.push((token, i))
                }
            }
            _ => {}
        }
        for (token, i) in tokens {
            for mut d in Self::enter(token) {
                d.push(i);
                if token.is_num() {
                    d.remove(0);
                }
                depth.push(d);
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
            match token {
                _ => {
                    println!("End Token: {token:?}")
                }
            }
        }
    }
}
