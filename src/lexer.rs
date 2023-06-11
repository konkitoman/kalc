use crate::token::Token;

#[derive(Default, Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub data: String,
    pub number_memory: String,
    pub memory: String,
    pub i: usize,
}

impl Lexer {
    pub fn parse(&mut self, data: &str) -> Result<(), String> {
        self.data.push_str(data);
        for (i, char) in data.chars().enumerate() {
            self.i = i;
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.number_memory.push(char)
                }
                ',' => {
                    self.process()?;
                }
                '_' | ' ' => {}
                '.' => self.number_memory.push(char),
                '-' => {
                    self.process()?;
                    self.tokens.push(Token::SSub);
                }
                '+' => {
                    self.process()?;
                    self.tokens.push(Token::SAdd);
                }
                '*' => {
                    self.process()?;
                    self.tokens.push(Token::SMul);
                }
                '/' => {
                    self.process()?;
                    self.tokens.push(Token::SDiv);
                }
                '(' => {
                    match self.memory.trim() {
                        "sin" => self.tokens.push(Token::SSin),
                        "cos" => self.tokens.push(Token::SCos),
                        "pow" => self.tokens.push(Token::SPow),
                        "sqrt" => self.tokens.push(Token::SSqrt),
                        _ => {
                            if !self.memory.is_empty() {
                                return Err(format!(
                                    "Invalid function: \"{}\", at: {}",
                                    self.memory, self.i
                                ));
                            }
                            self.memory.clear();
                            self.process()?;
                            self.tokens.push(Token::SGroupBeagin);
                            self.process()?;
                        }
                    }
                    self.memory.clear();
                }
                ')' => {
                    self.process()?;
                    self.tokens.push(Token::SGroupEnd);
                    self.process()?;
                }
                _ => self.memory.push(char),
            }
        }
        self.process()?;
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        let i = self.i;
        if !self.number_memory.is_empty() {
            let memory = std::mem::take(&mut self.number_memory);
            match Self::parse_group(memory) {
                Ok(res) => self.tokens.push(res),
                Err(err) => return Err(format!("Cannot parse number at: {i}, error: {err}")),
            }
        }

        if let Some(token) = self.tokens.last() {
            match token {
                Token::SGroupBeagin => {
                    let t = self.tokens.pop().unwrap();
                    if let Some(token) = self.tokens.last() {
                        match token {
                            Token::SAdd
                            | Token::SDiv
                            | Token::SSub
                            | Token::SMul
                            | Token::SSin
                            | Token::SCos
                            | Token::SPow
                            | Token::SSqrt
                            | Token::SGroupBeagin => {}
                            _ => self.tokens.push(Token::SMul),
                        }
                    }

                    self.tokens.push(t);
                }
                Token::SGroupEnd => {
                    self.tokens.pop();
                    let mut buffer = Vec::new();
                    's: {
                        while let Some(token) = self.tokens.pop() {
                            if let Token::SGroupBeagin = token {
                                buffer.reverse();
                                self.tokens.push(Token::Group(buffer));
                                break 's;
                            }
                            buffer.push(token)
                        }
                        while let Some(t) = buffer.pop() {
                            self.tokens.push(t)
                        }
                    }
                }
                _ => {}
            }
        }

        for i in 1..self.tokens.len() + 1 {
            let i = self.tokens.len() - i;
            if let Some(token) = self.tokens.get(i) {
                if self.tokens.len() > i + 1 {
                    let a = self.tokens.get(i + 1).unwrap();
                    if a.is_calculabile() {
                        match token {
                            Token::SAdd => self.add(i),
                            Token::SDiv => self.div(i),
                            Token::SSub => self.sub(i),
                            Token::SMul => self.mul(i),
                            Token::SSin => self.sin(i),
                            Token::SCos => self.cos(i),
                            Token::SPow => self.pow(i),
                            Token::SSqrt => self.sqrt(i),
                            _ => continue,
                        }
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "debug")]
        println!("Lexer State: {}", Token::Group(self.tokens.clone()));
        Ok(())
    }

    fn parse_group(data: String) -> Result<Token, String> {
        if data.contains('.') {
            match data.parse::<f64>() {
                Ok(num) => Ok(Token::F(num)),
                Err(_) => Err("Cannot parse number".into()),
            }
        } else {
            match data.parse::<i64>() {
                Ok(num) => Ok(Token::I(num)),
                Err(_) => Err("Cannot parse number".into()),
            }
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token)
    }

    fn add(&mut self, i: usize) {
        if let Some((a, b)) = self.get_ab(i) {
            self.push(Token::a(a, b))
        }
    }

    fn sub(&mut self, i: usize) {
        if let Some((a, b)) = self.get_ab(i) {
            self.push(Token::s(a, b))
        }
    }

    fn mul(&mut self, i: usize) {
        if let Some((a, b)) = self.get_ab(i) {
            self.push(Token::m(a, b))
        }
    }

    fn div(&mut self, i: usize) {
        if let Some((a, b)) = self.get_ab(i) {
            self.push(Token::d(a, b))
        }
    }

    fn sin(&mut self, i: usize) {
        if let Some(a) = self.get_a(i) {
            self.push(Token::sin(a))
        }
    }

    fn cos(&mut self, i: usize) {
        if let Some(a) = self.get_a(i) {
            self.push(Token::cos(a))
        }
    }

    fn pow(&mut self, i: usize) {
        if let Some((a, b)) = self.get_ab_liniar(i) {
            self.push(Token::pow(a, b))
        }
    }

    fn sqrt(&mut self, i: usize) {
        if let Some(a) = self.get_a(i) {
            self.push(Token::sqrt(a))
        }
    }

    fn get_ab_liniar(&mut self, i: usize) -> Option<(Token, Token)> {
        let Some(a) = self.tokens.get(i + 1) else {return None};
        let Some(b) = self.tokens.get(i + 2) else {return None};
        if a.is_calculabile() && b.is_calculabile() && self.tokens.len() > 2 {
            self.tokens.remove(i);
            let a = self.tokens.remove(i);
            let b = self.tokens.remove(i);
            return Some((a, b));
        }
        None
    }

    fn get_ab(&mut self, i: usize) -> Option<(Token, Token)> {
        if i > 0 {
            let Some(a) = self.tokens.get(i + 1) else {return None};
            let Some(b) = self.tokens.get(i - 1) else {return None};
            if a.is_calculabile() && b.is_calculabile() && self.tokens.len() > 2 {
                let a = self.tokens.remove(i - 1);
                self.tokens.remove(i - 1);
                let b = self.tokens.remove(i - 1);
                return Some((a, b));
            }
        }
        None
    }

    fn get_a(&mut self, i: usize) -> Option<Token> {
        if self.tokens.len() > 1 {
            self.tokens.remove(i);
            let a = self.tokens.remove(i);
            Some(a)
        } else {
            None
        }
    }
}
