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
        println!("Depths: {:?}", depths);
        for depth in depths {
            Self::calculate(&mut self.group, depth.0)
        }
        println!("Result: {:?}", self.group)
    }

    pub fn enter(token: &Token) -> Vec<(Vec<usize>, usize)> {
        let mut depth = vec![(vec![], 0)];
        let mut tokens = Vec::new();
        let mut real_depth = 0;
        match token {
            Token::Add(t1, t2) | Token::Div(t1, t2) | Token::Sub(t1, t2) | Token::Mul(t1, t2) => {
                println!("OR: t1: {t1:?}, t2: {t2:?}");
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

    fn visit(token: &mut Token, mut depths: Vec<usize>) -> Option<&mut Token> {
        let depth = depths.pop();
        if let Some(depth) = depth {
            match token {
                Token::Add(t0, t1)
                | Token::Div(t0, t1)
                | Token::Sub(t0, t1)
                | Token::Mul(t0, t1) => {
                    if depth == 0 {
                        return Self::visit(t0, depths);
                    } else {
                        return Self::visit(t1, depths);
                    }
                }
                Token::Group(tokens) => {
                    if let Some(token) = tokens.get_mut(depth) {
                        return Self::visit(token, depths);
                    }
                }
                _ => {}
            }
        } else {
            return Some(token);
        }
        None
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
