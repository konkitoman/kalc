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
                ',' | '_' | ' ' => {}
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

        let len = self.tokens.len();
        if len > 1 {
            if let Some(token) = self.tokens.get(len - 2) {
                let a = self.tokens.last().unwrap();
                if a.is_calculabile() {
                    match token {
                        Token::SAdd => self.add(),
                        Token::SDiv => self.div(),
                        Token::SSub => self.sub(),
                        Token::SMul => self.mul(),
                        Token::SSin => self.sin(),
                        Token::SCos => self.cos(),
                        _ => {}
                    }
                }
            }
        }
        #[cfg(feature = "debug")]
        println!("Tokens: {:?}", self.tokens);
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

    fn add(&mut self) {
        if let Some((a, b)) = self.get_ab() {
            self.push(Token::a(a, b))
        }
    }

    fn sub(&mut self) {
        if let Some((a, b)) = self.get_ab() {
            self.push(Token::s(a, b))
        }
    }

    fn mul(&mut self) {
        if let Some((a, b)) = self.get_ab() {
            self.push(Token::m(a, b))
        }
    }

    fn div(&mut self) {
        if let Some((a, b)) = self.get_ab() {
            self.push(Token::d(a, b))
        }
    }

    fn sin(&mut self) {
        let a = self.get_a();
        self.push(Token::sin(a))
    }

    fn cos(&mut self) {
        let a = self.get_a();
        self.push(Token::cos(a))
    }

    fn get_ab(&mut self) -> Option<(Token, Token)> {
        let len = self.tokens.len();
        if len > 2 {
            if let Some(b) = self.tokens.get(len - 3) {
                if b.is_calculabile() {
                    let b = self.tokens.pop().unwrap();
                    self.tokens.pop();
                    let a = self.tokens.pop().unwrap();
                    return Some((a, b));
                }
            }
        }
        None
    }

    fn get_a(&mut self) -> Token {
        let a = self.tokens.pop().unwrap();
        self.tokens.pop().unwrap();
        a
    }
}
