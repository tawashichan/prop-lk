
use crate::logic::{Formula,Sequent};

enum Token {
    Atom(&'static str), // 原子式
    RightArrow, // ->
    And, // ∧
    Or, // ∨
    Comma, // ,
    Space, // 
    Bar, // -
}

fn parse(chars: &[char]) -> Vec<Token> {
    match chars {
        ['a',_] => vec![],
        _ => vec![]
    }
}

fn parse_tokens(tokens: &[Token]) -> Vec<Sequent> {
    vec![]
}