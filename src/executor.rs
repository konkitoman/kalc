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
        let mut results = Self::enter(&self.group);
        // use sort by priority
        results.sort_by(|a, b| a.1.cmp(&b.1));
        results.reverse();
        let mut steps = Vec::with_capacity(results.len());
        steps.push(self.group.clone());
        for depth in results {
            Self::calculate(&mut self.group, depth.0);
            if let Some(last) = steps.last() {
                if *last == self.group {
                    continue;
                }
            }
            steps.push(self.group.clone())
        }

        #[cfg(feature = "debug")]
        for (i, step) in steps.iter().enumerate() {
            println!("Step{i}: {step}")
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
        let mut results = vec![(vec![], 0)];
        let mut tokens = Vec::new();
        match token {
            Token::Add(t1, t2)
            | Token::Div(t1, t2)
            | Token::Sub(t1, t2)
            | Token::Mul(t1, t2)
            | Token::Pow(t1, t2) => {
                tokens.push((t1.as_ref(), 0, 0));
                tokens.push((t2.as_ref(), 1, 0));
            }

            Token::Sin(t0) | Token::Cos(t0) | Token::Sqrt(t0) => tokens.push((t0.as_ref(), 0, 0)),

            Token::Group(group) => {
                for (index, token) in group.iter().enumerate() {
                    tokens.push((token, index, 1))
                }
            }
            _ => {}
        }
        for (token, index, current_preority) in tokens {
            for (mut d, preority) in Self::enter(token) {
                d.push(index);
                if token.is_num() {
                    d.remove(0);
                }
                results.push((d, current_preority + preority));
            }
        }
        results
    }

    fn calculate(token: &mut Token, mut indexes: Vec<usize>) {
        let index = indexes.pop();
        if let Some(index) = index {
            match token {
                Token::Add(t0, t1)
                | Token::Div(t0, t1)
                | Token::Sub(t0, t1)
                | Token::Mul(t0, t1)
                | Token::Pow(t0, t1) => {
                    if index == 0 {
                        Self::calculate(t0, indexes)
                    } else {
                        Self::calculate(t1, indexes)
                    }
                }
                Token::Sin(t0) | Token::Cos(t0) | Token::Sqrt(t0) => Self::calculate(t0, indexes),
                Token::Group(tokens) => {
                    if let Some(token) = tokens.get_mut(index) {
                        Self::calculate(token, indexes)
                    }
                }
                _ => {}
            }
        } else {
            token.calculate();
        }
    }
}
