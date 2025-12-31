use std::collections::HashMap;
use crate::ast::{Expr, Stmt};

#[derive(Clone, Debug)]
enum Value {
    Number(i64),
    String(String),
}

pub struct Interpreter {
    vars: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                let val = self.eval(expr);
                self.vars.insert(name, val);
            }

            Stmt::ConsoleLog(expr) => {
                let value = self.eval(expr);
                match value {
                    Value::Number(n) => println!("{n}"),
                    Value::String(s) => println!("{s}"),
                }
            }
        }
    }

    fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),

            Expr::String(s) => Value::String(s),

            Expr::Variable(name) => {
                self.vars
                    .get(&name)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name))
                    .clone()
            }

            Expr::Binary(left, right) => {
                match (self.eval(*left), self.eval(*right)) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                    _ => panic!("Type error: only numbers can be added"),
                }
            }
        }
    }
}
