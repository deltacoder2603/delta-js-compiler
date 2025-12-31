# Delta – JavaScript-like Compiler & Interpreter (Rust)

Delta is an educational **JavaScript-like language compiler and interpreter** written in **Rust**. The project implements a real compiler pipeline — **lexer → parser → AST → interpreter** — and provides both a **runtime mode** and a **debug mode** to visualize internal compiler stages.

This project is designed as a **systems-level learning project**, focusing on correctness, clarity, and architecture rather than production completeness.

---

## ✨ Features

* JavaScript-like syntax (subset)
* Real compiler pipeline:

  * Lexical analysis (tokenizer)
  * Parsing into an AST
  * Tree-walk interpreter
* Variable declarations (`let`)
* Numeric and string literals
* Binary expressions (`+`)
* Built-in `console.log(...)`
* Interactive CLI (REPL-style)
* Debug mode to inspect:

  * Source code
  * Tokens
  * AST
* Written entirely in **Rust**

---

## 🚀 Example

### `index.js`

```js
let y = 10;
console.log(y);
console.log("Hello World");
console.log(y + 5);
```

### Normal Run

```text
delta> delta index.js
10
Hello World
15
```

### Debug Run

```text
delta> delta index.js -d

--------------------------------------------------
SOURCE CODE
--------------------------------------------------
let y = 10;
console.log(y);
console.log("Hello World");
console.log(y + 5);

--------------------------------------------------
LEXER OUTPUT (TOKENS)
--------------------------------------------------
Let
Identifier("y")
Equal
Number(10)
Semicolon
Console
Dot
Log
LParen
Identifier("y")
RParen
Semicolon
...

--------------------------------------------------
PARSER OUTPUT (AST)
--------------------------------------------------
Let("y", Number(10))
ConsoleLog(Variable("y"))
ConsoleLog(String("Hello World"))
ConsoleLog(Binary(Variable("y"), Number(5)))

--------------------------------------------------
PROGRAM OUTPUT
--------------------------------------------------
10
Hello World
15
```

---

## 🧠 Architecture

The project follows a standard compiler design used in real-world language implementations.

```
Source Code (.js)
   ↓
Lexer (Tokenizer)
   ↓
Parser
   ↓
Abstract Syntax Tree (AST)
   ↓
Interpreter (Tree-walk)
```

### Components

| Module           | Responsibility                      |
| ---------------- | ----------------------------------- |
| `lexer.rs`       | Converts raw characters into tokens |
| `token.rs`       | Defines token types                 |
| `parser.rs`      | Converts tokens into AST nodes      |
| `ast.rs`         | Defines AST structures              |
| `interpreter.rs` | Executes the AST                    |
| `main.rs`        | CLI, REPL loop, debug mode          |

---

## 🧪 Supported Language Features

* `let` variable declarations
* Integer literals
* String literals
* Binary addition (`+`)
* Variable lookup
* `console.log(expression)`

### Not Supported (Yet)

* Functions
* Conditionals (`if`, `while`)
* Objects
* Arrays
* Scopes / closures
* Bytecode / VM

---

## 🛠️ How to Run

### Requirements

* Rust (stable)

### Run the Compiler

```bash
cargo run
```

You will see:

```text
Delta JS Compiler
Type: delta <file.js>   (Press Ctrl+C to exit)
```

Then run:

```text
delta> delta index.js
```

For debug mode:

```text
delta> delta index.js -d
```

Press **Ctrl + C** to exit the CLI.

---

## 🎯 Learning Outcomes

This project demonstrates understanding of:

* Compiler design fundamentals
* Tokenization and parsing
* Abstract Syntax Trees
* Interpreters and runtime environments
* Rust enums, pattern matching, and ownership
* CLI application design

---

## 📌 Project Scope & Intent

Delta is **not** a production JavaScript engine.

It is intentionally:

* Minimal
* Explicit
* Easy to extend

The goal is to **learn how real compilers work**, not to reimplement V8.

---

## 🔮 Possible Extensions

* Add block scopes (`{}`)
* Implement `if` / `while`
* Add functions and call stack
* Convert AST to bytecode
* Build a virtual machine (VM)
* Add static type checking

---

## 📄 License

This project is open for educational use.

---

## 🙌 Acknowledgements

Inspired by:

* *Crafting Interpreters* – Robert Nystrom
* LLVM / compiler design literature
* Rust language book

---

## 👤 Author

**Srikant Pandey**
Rust & Systems Programming Enthusiast
