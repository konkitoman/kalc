use crate::token::Token;

#[derive(Default, Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub memory: Vec<u8>,
    pub i: usize,
}

impl Lexer {
    pub fn parse(&mut self, data: &str) -> Result<(), String> {
        for (i, char) in data.chars().enumerate() {
            self.i = i;
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.memory.push(char as u8)
                }
                ',' | '_' | ' ' => {}
                '.' => self.memory.push(char as u8),
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
                    self.tokens.push(Token::SGroupBeagin);
                }
                ')' => {
                    self.process()?;
                    self.tokens.push(Token::SGroupEnd);
                    self.process()?;
                }
                _ => return Err(format!("Invalid character at: {i}")),
            }
        }
        self.process();
        Ok(())
    }

    fn process(&mut self) -> Result<(), String> {
        let i = self.i;
        if !self.memory.is_empty() {
            let memory = std::mem::replace(&mut self.memory, Vec::new());
            match Self::parse_group(memory) {
                Ok(res) => self.tokens.push(res),
                Err(err) => return Err(format!("Cannot parse number at: {i}, error: {err}")),
            }
        }

        let len = self.tokens.len();
        if let Some(Token::SGroupEnd) = self.tokens.last() {
            self.tokens.pop();
            let mut buffer = Vec::new();
            while let Some(token) = self.tokens.pop() {
                if let Token::SGroupBeagin = token {
                    break;
                } else {
                    buffer.push(token)
                }
            }
            buffer.reverse();
            self.tokens.push(Token::Group(buffer));
        }
        if len > 2 {
            let a = self.tokens.pop().unwrap();
            let b = self.tokens.pop().unwrap();
            let c = self.tokens.pop().unwrap();

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
        }
        Ok(())
    }

    fn parse_group(data: Vec<u8>) -> Result<Token, String> {
        // if is float 10 reprezents '.'
        if data.contains(&b'.') {
            let data = String::from_utf8(data).unwrap();
            match data.parse::<f64>() {
                Ok(num) => Ok(Token::F(num)),
                Err(_) => Err("Cannot parse number".into()),
            }
        } else {
            let data = String::from_utf8(data).unwrap();
            match data.parse::<i64>() {
                Ok(num) => Ok(Token::I(num)),
                Err(_) => Err("Cannot parse number".into()),
            }
        }
    }
}
