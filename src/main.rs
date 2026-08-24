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

    #[regex(r"[a-z_]+", to_string)]
    pub type VarName = String;

    #[token("=")]
    pub struct Equals;

    #[regex(r"\n\s*", priority = 10)]
    pub struct Terminator;

    #[token("+")]
    #[left_associative]
    #[priority(0)]
    pub struct Plus;

    #[token("-")]
    #[left_associative]
    #[priority(0)]
    pub struct Minus;

    #[token("*")]
    #[left_associative]
    #[priority(1)]
    pub struct Times;

    #[token("/")]
    #[left_associative]
    #[priority(1)]
    pub struct DividedBy;

    #[token("^")]
    #[right_associative]
    #[priority(3)]
    pub struct Power;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    #[regex(r"\d+", parse)]
    #[regex(r"\d*\.\d+", parse)]
    #[token("PI", |_| std::f32::consts::PI)]
    pub type Num = f32;

    production!(Number: Expr -> Num);
    production!(Sum: Expr -> (Expr, Plus, Expr), |(left, _, right)| left + right);
    production!(Difference: Expr -> (Expr, Minus, Expr), |(left, _, right)| left - right);
    production!(Product: Expr -> (Expr, Times, Expr), |(left, _, right)| left * right);
    production!(Division: Expr -> (Expr, DividedBy, Expr), |(left, _, right)| left / right);
    production!(Exponent: Expr -> (Expr, Power, Expr), |(left, _, right)| left.powf(right));
    production!(Parentheses: Expr -> (OpenPar, Expr, ClosePar), |(_, expr, _)| expr);
    #[priority(2)]
    production!(Positive: Expr -> (Plus, Expr), |(_, expr)| expr);
    #[priority(2)]
    production!(Negative: Expr -> (Minus, Expr), |(_, expr)| -expr);
    production!(VariableValue: Expr -> VarName, |ctx, name| ctx[&name]);
    production!(
        Assignment: Statement -> (VarName, Equals, Expr, Terminator),
        |ctx, (name, _, expr, _)| {
            ctx.insert(name, expr);
        }
    );
    production!(EmptyLine: Statement -> Terminator, |_| {});
    ebnf!(RunProgram: Program -> (Vec<Statement>, Expr, Option<Terminator>), |(_, expr, _)| expr);
}

fn main() {
    let file = include_str!("../program.calc");
    let result: Result<(f32, calculator::VariablesTable), _> =
        calculator::Parser::lex_parse_default_ctx(file);
    match result {
        Ok((res, vars)) => {
            println!("the result is: {res}");
            println!("the variables used are the following: {vars:?}");
        }
        Err(err) => eprintln!("{err}"),
    }
}
