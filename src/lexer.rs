use crate::token::Token;

#[derive(Default, Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub memory: String,
    pub i: usize,
}

impl Lexer {
    pub fn parse(&mut self, data: &str) -> Result<(), String> {
        for (i, char) in data.chars().enumerate() {
            self.i = i;
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.memory.push(char),
                ',' | '_' | ' ' => {}
                '.' => self.memory.push(char),
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
                    self.process()?;
                    self.tokens.push(Token::SGroupBeagin);
                    self.process()?;
                }
                ')' => {
                    self.process()?;
                    self.tokens.push(Token::SGroupEnd);
                    self.process()?;
                }
                _ => return Err(format!("Invalid character '{char}' at: {}", i + 1)),
            }
        }
        self.process()?;
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        let i = self.i;
        if !self.memory.is_empty() {
            let memory = std::mem::take(&mut self.memory);
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
                            | Token::SGroupBeagin => {}
                            _ => self.tokens.push(Token::SMul),
                        }
                    }

                    self.tokens.push(t);
                }
                Token::SGroupEnd => {
                    self.tokens.pop();
                    let mut buffer = Vec::new();
                    while let Some(token) = self.tokens.pop() {
                        if let Token::SGroupBeagin = token {
                            break;
                        }
                        buffer.push(token)
                    }
                    buffer.reverse();
                    self.tokens.push(Token::Group(buffer));
                }
                _ => {}
            }
        }

        let len = self.tokens.len();
        if len > 2 {
            let a = self.tokens.pop().unwrap();
            let b = self.tokens.pop().unwrap();
            let c = self.tokens.pop().unwrap();

            if a.is_calculabile() && c.is_calculabile() {
                match b {
                    Token::SAdd => self.tokens.push(Token::Add(Box::new(c), Box::new(a))),
                    Token::SSub => self.tokens.push(Token::Sub(Box::new(c), Box::new(a))),
                    Token::SMul => self.tokens.push(Token::Mul(Box::new(c), Box::new(a))),
                    Token::SDiv => self.tokens.push(Token::Div(Box::new(c), Box::new(a))),
                    _ => {
                        self.tokens.push(c);
                        self.tokens.push(b);
                        self.tokens.push(a);
                    }
                }
            } else {
                self.tokens.push(c);
                self.tokens.push(b);
                self.tokens.push(a);
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
}
