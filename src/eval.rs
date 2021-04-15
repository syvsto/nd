use std::collections::HashMap;

use crate::parser::{Builtins, Token, TokenStack};

#[derive(Debug)]
enum Op {
    ContinueToThen,
    ContinueToDefinitionEnd,
}

#[derive(Debug, Clone)]
pub enum Val {
    Float(f64),
    Bool(bool),
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<Val>) -> Option<Op> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    println!("{:?}", last);
                }
            }

            Plus => {
                let n1 = stack.pop().expect("Couldn't pop first value for addition from stack");
                let n2 = stack.pop().expect("Couldn't pop second value for addition from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a + b)),
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop().expect("Couldn't pop first value for comparison from stack");
                let n2 = stack.pop().expect("Couldn't pop second value for comparison from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => {
                        if a == b {
                            stack.push(Val::Bool(true))
                        } else {
                            stack.push(Val::Bool(false))
                        }
                    }
                    _ => panic!("Couldn't compare values, not all values were numbers."),
                }
            }

            Minus => {
                let n1 = stack.pop().expect("Couldn't pop first value for subtraction from stack");
                let n2 = stack.pop().expect("Couldn't pop second value for subtraction from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a - b)),
                    _ => panic!("Couldn't subtract values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop().expect("Couldn't pop first value for multiplication from stack");
                let n2 = stack.pop().expect("Couldn't pop second value for multiplication from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a * b)),
                    _ => panic!("Couldn't multiply values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop().expect("Couldn't pop first value for division from stack");
                let n2 = stack.pop().expect("Couldn't pop second value for division from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a / b)),
                    _ => panic!("Couldn't divide values, not all values were numbers."),
                }
            }

            If => {
                let comparison = stack.pop().expect("Couldn't pop boolean value for comparison from stack");
                match comparison {
                    Val::Bool(cmp) => {
                        if !cmp {
                            return Some(Op::ContinueToThen);
                        }
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }

            WordStart => {
                return Some(Op::ContinueToDefinitionEnd);
            }

            Duplicate => {
                let n = stack.pop().expect("Couldn't duplicate element, nothing on stack");
                stack.push(n.clone());
                stack.push(n);
            }

            _ => {}
        }
        None
    }
}

fn next_of_type(ty: Builtins, tokens: &[Token]) -> Option<usize> {
    tokens.iter().position(|token| match token {
        Token::Builtin(t) => *t == ty,
        _ => false,
    })
}

fn eval_op(
    op: &Op,
    current_index: usize,
    ast: &TokenStack,
) -> usize {
    use Op::*;

    match op {
        ContinueToThen => {
            if let Some(v) = next_of_type(Builtins::Then, &ast.tokens) {
                return v;
            }
        }
        ContinueToDefinitionEnd => {
            if let Some(v) = next_of_type(Builtins::WordEnd, &ast.tokens) {
                return v;
            }
        }
    }

    current_index
}

pub fn eval(
    ast: &TokenStack,
    stack: &mut Vec<Val>,
    words: &HashMap<String, TokenStack>,
    debug: bool,
) {
    use Token::*;

    let mut i = 0;

    while i < ast.tokens.len() {
        match &ast.tokens[i] {
            Word(name) => {
                if let Some(word) = words.get(name) {
                    eval(word, stack, &words, debug);
                }
            }
            Number(n) => stack.push(Val::Float(*n)),
            Builtin(func) => {
                if let Some(ref op) = func.eval(stack) {
                    i = eval_op(op, i, &ast);
                }
            }
        }
        i += 1;
    }

    if debug {
        println!("{:?}", stack);
        println!("{:?}", words);
    }
}
