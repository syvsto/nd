use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::parser::{Builtins, Token, TokenStack};

#[derive(Debug)]
enum Op {
    ContinueToThen,
    DefineWord,
}

#[derive(Debug)]
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
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a + b)),
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
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
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a - b)),
                    _ => panic!("Couldn't subtract values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a * b)),
                    _ => panic!("Couldn't multiply values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a / b)),
                    _ => panic!("Couldn't divide values, not all values were numbers."),
                }
            }

            Then => {}

            If => {
                let comparison = stack.pop().expect("Couldn't pop from stack");
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
                return Some(Op::DefineWord);
            }

            WordEnd => {}
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

fn eval_op(op: &Op, current_index: usize, ast: &TokenStack, words: Rc<RefCell<HashMap<String, TokenStack>>>) -> usize {
    use Op::*;

    match op {
        ContinueToThen => {
            if let Some(v) = next_of_type(Builtins::Then, &ast.tokens) {
                return v;
            }
        }
        DefineWord => {
            let pos = ast.tokens.iter().position(|token| *token == Token::Builtin(Builtins::WordEnd)).expect("Couldn't find end to definition");
            if let Some((name, definition)) = ast.tokens[current_index+1..pos].split_first() {
                if let Token::Word(n) = name {
                    words.borrow_mut().insert(n.clone(), TokenStack::new_from_tokens(definition));
                }
            }
        }
    }

    current_index
}

pub fn eval(ast: &TokenStack, stack: &mut Vec<Val>, words: Rc<RefCell<HashMap<String, TokenStack>>>, debug: bool) {
    use Token::*;

    let mut i = 0;

    while i < ast.tokens.len() {
        match &ast.tokens[i] {
            Word(name) => {
                if let Some(word) = words.borrow().get(name) {
                    eval(word, stack, words.clone(), debug);
                }
            }
            Number(n) => stack.push(Val::Float(*n)),
            Builtin(func) => {
                if let Some(ref op) = func.eval(stack) {
                    i = eval_op(op, i, &ast, words.clone());
                }
            }
        }
        i += 1;
    }

    if debug {
        println!("{:?}", stack);
        println!("{:?}", words.borrow());
    }

}


