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
                    self.memory.push(char as u8 - 48)
                }
                ',' | '_' | ' ' => {}
                '.' => self.memory.push(10),
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
        if data.contains(&10) {
            let segments = data
                .split(|n| *n == 10)
                .map(|d| d.to_vec())
                .collect::<Vec<Vec<u8>>>();
            if segments.len() != 2 {
                return Err(format!("In a decimal number need to be only one ."));
            }
            let a = Self::parse_num(segments[0].to_vec())?;
            let b = Self::parse_num(segments[1].to_vec())?;
            Ok(Token::Decimal(Box::new(a), Box::new(b)))
        } else {
            Self::parse_num(data)
        }
    }

    fn parse_num(data: Vec<u8>) -> Result<Token, String> {
        if data.len() < 10 {
            let mut num = 0i32;
            let mut data = data;
            data.reverse();
            let mut i = 1;
            for b in data {
                if b > 0 {
                    num += b as i32 * i;
                }
                i *= 10;
            }
            return Ok(Token::I32(num));
        } else if data.len() < 19 {
            let mut num = 0i64;
            let mut data = data;
            data.reverse();
            let mut i = 1;
            for b in data {
                if b > 0 {
                    num += b as i64 * i;
                }
                i *= 10;
            }
            return Ok(Token::I64(num));
        } else {
            let (a, b) = data.split_at(18);
            let (a, b) = (a.to_vec(), b.to_vec());
            let a = Self::parse_num(a)?;
            let b = Self::parse_num(b)?;
            let mut res = vec![];
            match a {
                Token::Intiger(a) => {
                    for b in a {
                        res.push(b)
                    }
                }
                _ => res.push(a),
            }
            match b {
                Token::Intiger(a) => {
                    for b in a {
                        res.push(b)
                    }
                }
                _ => res.push(b),
            }
            return Ok(Token::Intiger(res));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::Lexer;

    #[test]
    fn parse_num() {
        assert_eq!(Lexer::parse_num(vec![]).unwrap(), Token::I32(0));
        assert_eq!(
            Lexer::parse_num(vec![9, 9, 9, 9, 9, 9, 9, 9, 9]).unwrap(),
            Token::I32(999999999)
        );
        assert_eq!(
            Lexer::parse_num(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 0]).unwrap(),
            Token::I64(9999999990)
        );
        assert_eq!(
            Lexer::parse_num(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]).unwrap(),
            Token::I64(999999999999999999)
        );
        assert_eq!(
            Lexer::parse_num(vec![
                9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0
            ])
            .unwrap(),
            Token::Intiger(vec![Token::I64(999999999999999999), Token::I32(0)])
        );
    }
}
