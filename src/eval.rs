// use std::rc::Rc;
// use std::cell::RefCell;
use std::collections::HashMap;

use crate::parser::{Builtins, Token, TokenStack};

#[derive(Debug)]
enum Control {
    ContinueToThen,
    DefineWord,
}

#[derive(Debug)]
enum Val {
    Float(f64),
    Bool(bool),
}




impl Builtins {
    fn eval(&self, stack: &mut Vec<Val>) -> Option<Control> {
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
                            return Some(Control::ContinueToThen);
                        }
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }

            WordStart => {
                return Some(Control::DefineWord);
            }

            WordEnd => {}
        }
        None
    }
}

fn next_of_type(ty: Builtins, tokens: &[Token]) -> Option<usize> {
   tokens.iter()
       .position(|token| match token {
           Token::Builtin(t) => *t == ty,
           _ => false
       })
}

fn peek_word(tokens: &[Token]) -> Option<String> {
    match &tokens[0] {
        Token::Word(name) => Some(name.clone()),
        _ => None,
    }
}

pub fn eval(ast: &TokenStack, debug: bool) {
    use Token::*;

    let mut words = HashMap::new();
    let mut stack = Vec::new();

    let mut i = 0;
    while i < ast.tokens.len() {
        match &ast.tokens[i] {
            Word(name) => {
                if let Some(word) = words.get(name) {
                    eval(word, debug);
                }
            }
            Number(n) => stack.push(Val::Float(*n)),
            Builtin(func) => {
                if let Some(ctrl) = func.eval(&mut stack) {
                    match ctrl {
                        Control::ContinueToThen => {
                            if let Some(v) = next_of_type(Builtins::Then, &ast.tokens) {
                                i = v;
                            }
                        }
                        Control::DefineWord => {
                            let mut word = TokenStack { tokens:  vec![] };
                            let name = peek_word(&ast.tokens[i..]).expect("First token in function definition wasn't a word.");
                            while ast.tokens[i] != Builtin(Builtins::WordEnd) {
                                word.tokens.push(ast.tokens[i].clone())
                            }
                            words.insert(name, word);
                        }
                    }
                }
            }
        }
        if debug {
            println!("{:?}", stack);
        }
        i += 1;
    }
}
