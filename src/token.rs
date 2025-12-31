#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Console,
    Log,
    Identifier(String),
    Number(i64),
    String(String),
    Plus,
    Dot,
    Equal,
    Semicolon,
    LParen,
    RParen,
}
