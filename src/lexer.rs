use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current() {
            match c {
                ' ' | '\n' | '\t' => self.advance(),

                '+' => {
                    self.advance();
                    return Some(Token::Plus);
                }
                '.' => {
                    self.advance();
                    return Some(Token::Dot);
                }
                '=' => {
                    self.advance();
                    return Some(Token::Equal);
                }
                ';' => {
                    self.advance();
                    return Some(Token::Semicolon);
                }
                '(' => {
                    self.advance();
                    return Some(Token::LParen);
                }
                ')' => {
                    self.advance();
                    return Some(Token::RParen);
                }
                '"' => return Some(self.string()),
                '0'..='9' => return Some(self.number()),
                'a'..='z' => return Some(self.identifier()),

                _ => panic!("Unexpected character: {}", c),
            }
        }
        None
    }

    fn number(&mut self) -> Token {
        let mut num = 0;
        while let Some(c) = self.current() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i64;
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num)
    }

    fn identifier(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.current() {
            if c.is_alphabetic() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match s.as_str() {
            "let" => Token::Let,
            "console" => Token::Console,
            "log" => Token::Log,
            _ => Token::Identifier(s),
        }
    }

    fn string(&mut self) -> Token {
        self.advance(); 
        let mut s = String::new();

        while let Some(c) = self.current() {
            if c == '"' {
                break;
            }
            s.push(c);
            self.advance();
        }

        self.advance();
        Token::String(s)
    }
}
