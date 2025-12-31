mod lexer;
mod parser;
mod token;
mod ast;
mod interpreter;

use std::fs;
use std::io::{self, Write};

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn separator(title: &str) {
    println!("\n--------------------------------------------------");
    println!("{title}");
    println!("--------------------------------------------------");
}

fn main() {
    println!("Delta JS Compiler");
    println!("Commands:");
    println!("  delta <file.js>        → run");
    println!("  delta <file.js> -d     → debug mode");
    println!("Press Ctrl+C to exit\n");

    loop {
        // ---- PROMPT ----
        print!("delta> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        // delta file.js
        // delta file.js -d
        if parts.len() < 2 || parts.len() > 3 || parts[0] != "delta" {
            println!("Usage: delta <file.js> [-d]");
            continue;
        }

        let filename = parts[1];
        let debug = parts.len() == 3 && parts[2] == "-d";

        let code = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                println!("Error: could not read file `{filename}`");
                continue;
            }
        };

        // ---- LEXER ----
        let mut lexer = Lexer::new(&code);
        let mut tokens = Vec::new();
        while let Some(tok) = lexer.next_token() {
            tokens.push(tok);
        }

        // ---- PARSER ----
        let mut parser = Parser::new(tokens.clone());
        let ast = parser.parse();

        // ---- DEBUG MODE OUTPUT ----
        if debug {
            separator("SOURCE CODE");
            println!("{code}");

            separator("LEXER OUTPUT (TOKENS)");
            for tok in &tokens {
                println!("{:?}", tok);
            }

            separator("PARSER OUTPUT (AST)");
            for stmt in &ast {
                println!("{:?}", stmt);
            }

            separator("PROGRAM OUTPUT");
        }

        // ---- INTERPRETER ----
        let mut interpreter = Interpreter::new();
        interpreter.run(ast);

        if debug {
            separator("END");
        }
    }
}
