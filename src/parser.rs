use crate::{
    ast::{Expr, Stmt},
    token::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current().is_some() {
            stmts.push(self.statement()); // ✅ returns Stmt
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match self.current() {
            Some(Token::Let) => self.let_stmt(),
            Some(Token::Console) => self.console_log_stmt(),
            _ => panic!("Invalid statement"),
        }
    }

    fn let_stmt(&mut self) -> Stmt {
        self.advance(); // let

        let name = if let Some(Token::Identifier(n)) = self.current() {
            n.clone()
        } else {
            panic!("Expected identifier");
        };

        self.advance(); // identifier
        self.advance(); // =

        let expr = self.expression();

        self.advance(); // ;

        Stmt::Let(name, expr)
    }

    fn console_log_stmt(&mut self) -> Stmt {
        self.advance(); // console
        self.advance(); // .
        self.advance(); // log
        self.advance(); // (

        let expr = self.expression();

        self.advance(); // )
        self.advance(); // ;

        Stmt::ConsoleLog(expr)
    }

    fn expression(&mut self) -> Expr {
        let left = self.term();

        if let Some(Token::Plus) = self.current() {
            self.advance();
            let right = self.term();
            Expr::Binary(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    fn term(&mut self) -> Expr {
        match self.current() {
            Some(Token::Number(n)) => {
                let v = *n;
                self.advance();
                Expr::Number(v)
            }
            Some(Token::String(s)) => {
                let v = s.clone();
                self.advance();
                Expr::String(v)
            }
            Some(Token::Identifier(s)) => {
                let v = s.clone();
                self.advance();
                Expr::Variable(v)
            }
            _ => panic!("Invalid expression"),
        }
    }
}
