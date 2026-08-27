# Calculator

A scriptable arithmetic expression calculator and domain-specific language (DSL) built with the [Semasia](https://crates.io/crates/semasia) parser generator framework for Rust.

This project demonstrates how to define lexical tokens, operator precedence, contextual symbol tables, productions, and EBNF syntax rules in Semasia to parse and evaluate mathematical scripts.

---

## Features

- **Variables & Assignments**: Assign values to lowercase/snake_case variable names (`r = 2.5`).
- **Arithmetic Operators**:
  - Addition (`+`) and Subtraction (`-`) — left-associative, precedence 0
  - Multiplication (`*`) and Division (`/`) — left-associative, precedence 1
  - Unary Signs (`+`, `-`) — precedence 2
  - Exponentiation (`^`) — right-associative, precedence 3
- **Grouping**: Parentheses `(...)` for nested expressions.
- **Numbers & Constants**:
  - Integers (`10`)
  - Floating-point numbers (`2.5`, `.5`)
  - Built-in constants (`PI`)
- **Comments**: Single-line comments starting with `#`.
- **Contextual Execution**: Evaluates expressions on the fly while tracking variables in a shared `VariablesTable` context (`HashMap<String, f32>`).

---

## Example Script

A script consists of optional variable assignment statements followed by an expression to evaluate:

```calc
# this program calculates the volume of a cylinder:

r = 2.5 # r is the radius
h = 10 # h is the height

PI * r^2 * h # result
```

When executed, this outputs:

```text
the result is: 196.34955
the variables used are the following: {"r": 2.5, "h": 10.0}
```

---

## Grammar Overview

The parser is implemented in [`src/main.rs`](src/main.rs) using Semasia's attribute macros:

```rust
#[semasia::grammar]
#[logos(skip r"\s+")]
#[logos(skip(r"#.*", allow_greedy = true))]
mod calculator {
    use std::collections::HashMap;
    use semasia::*;

    #[context]
    pub type VariablesTable = HashMap<VarName, Expr>;

    #[start_symbol]
    #[non_terminal]
    pub type Program = f32;

    #[non_terminal]
    pub type Statement = ();

    #[non_terminal]
    pub type Expr = f32;

    // Lexical tokens & regex rules
    #[regex(r"[a-z_]+", to_string)]
    pub type VarName = String;

    #[token("=")]
    pub struct Equals;

    // Precedence and associativity annotations
    #[token("+")] #[left_associative] #[priority(0)] pub struct Plus;
    #[token("-")] #[left_associative] #[priority(0)] pub struct Minus;
    #[token("*")] #[left_associative] #[priority(1)] pub struct Times;
    #[token("/")] #[left_associative] #[priority(1)] pub struct DividedBy;
    #[token("^")] #[right_associative] #[priority(3)] pub struct Power;

    // Productions and EBNF rules
    production!(Sum: Expr -> (Expr, Plus, Expr), |(left, _, right)| left + right);
    // ...
    ebnf!(RunProgram: Program -> (Vec<Statement>, Expr, Option<Terminator>), |(_, expr, _)| expr);
}
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (2024 edition or newer)
- Cargo

### Running

Clone the repository and run using Cargo:

```bash
cargo run
```

To run your own calculations, edit [`program.calc`](program.calc) or load a custom string into `calculator::Parser::lex_parse_default_ctx(...)`.
