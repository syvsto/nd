use std::collections::HashMap;
use std::iter::FromIterator;

use crate::parser::{Builtins, DataType, Token, TokenStack};

#[derive(Debug)]
enum Op {
    ContinueToForward,
    ContinueToDefinitionEnd,
    ContinueToArrayEnd,
}

#[derive(Debug, Clone)]
pub enum Val {
    Number(Vec<f64>),
    Char(Vec<char>),
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<Val>) -> Option<Op> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    match last {
                        Val::Number(n) => println!("{:?}", n),
                        Val::Char(c) => println!("{}", String::from_iter(c)),
                    }
                }
            }

            Plus => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a + b).collect();
                        stack.push(Val::Number(n))
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        if a == b {
                            stack.push(Val::Number(vec![1.]))
                        } else {
                            stack.push(Val::Number(vec![0.]))
                        }
                    }
                    (Val::Char(a), Val::Char(b)) => {
                        if a == b {
                            stack.push(Val::Number(vec![1.]))
                        } else {
                            stack.push(Val::Number(vec![0.]))
                        }
                    }
                    _ => panic!("Couldn't compare values, not all values were comparable."),
                }
            }

            Minus => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a - b).collect();
                        stack.push(Val::Number(n))
                    }
                    _ => panic!("Couldn't subtract values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a * b).collect();
                        stack.push(Val::Number(n))
                    }
                    _ => panic!("Couldn't multiply values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a / b).collect();
                        stack.push(Val::Number(n))
                    }
                    _ => panic!("Couldn't divide values, not all values were numbers."),
                }
            }

            If => {
                let comparison = stack.pop()?;
                match comparison {
                    Val::Number(cmp) => {
                        if cmp.iter().all(|x| *x == 0.) {
                            return Some(Op::ContinueToForward);
                        }
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }

            And => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        stack.push(Val::Number(
                            b.iter()
                                .zip(a.iter().cycle())
                                .map(|(a, b)| if *a == 1. && *b == 1. { 1. } else { 0. })
                                .collect(),
                        ));
                    }
                    _ => panic!("Both stack elements were not present, or were not numbers."),
                }
            }

            Or => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        stack.push(Val::Number(
                            b.iter()
                                .zip(a.iter().cycle())
                                .map(|(a, b)| if *a == 1. || *b == 1. { 1. } else { 0. })
                                .collect(),
                        ));
                    }
                    _ => panic!("Both stack elements were not present, or were not numbers."),
                }
            }

            Concat => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;

                match (n1, n2) {
                    (Val::Number(mut a), Val::Number(b)) => {
                        a.extend(b);
                        stack.push(Val::Number(a));
                    }
                    (Val::Char(mut a), Val::Char(b)) => {
                        a.extend(b);
                        stack.push(Val::Char(a));
                    }
                    _ => panic!("Wrong type combination or missing values in concatenation."),
                }
            }

            WordStart => {
                return Some(Op::ContinueToDefinitionEnd);
            }

            ArrayStart => {
                return Some(Op::ContinueToArrayEnd);
            }

            Duplicate => {
                let n = stack.pop()?;
                stack.push(n.clone());
                stack.push(n);
            }

            Swap => {
                let n1 = stack.pop()?;
                let n2 = stack.pop()?;

                stack.push(n1);
                stack.push(n2);
            }

            Clear => {
                stack.clear();
            }

            ClearButOne => {
                if let Some((last, _)) = stack.split_last() {
                    let l = last.clone();
                    stack.clear();
                    stack.push(l);
                }
            }

            Pop => {
                stack.pop()?;
            }

            Forward => {}
            WordEnd => {}
            ArrayEnd => {}
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

fn eval_op(op: &Op, current_index: usize, ast: &TokenStack) -> usize {
    use Op::*;

    match op {
        ContinueToForward => {
            if let Some(v) = next_of_type(Builtins::Forward, &ast.tokens) {
                return v;
            }
        }
        ContinueToDefinitionEnd => {
            if let Some(v) = next_of_type(Builtins::WordEnd, &ast.tokens) {
                return v;
            }
        }
        ContinueToArrayEnd => {
            if let Some(v) = next_of_type(Builtins::ArrayEnd, &ast.tokens) {
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
                if let Some(word) = words.get(name.as_str()) {
                    eval(word, stack, words, debug);
                }
            }
            Data(data) => match data {
                DataType::Number(n) => stack.push(Val::Number(n.to_vec())),
                DataType::Char(c) => stack.push(Val::Char(c.to_vec())),
            },
            Builtin(func) => {
                if let Some(ref op) = func.eval(stack) {
                    i = eval_op(op, i, &ast);
                }
            }
            Definition(_) => {}
        }
        i += 1;
    }

    if debug {
        println!("{:?}", stack);
        println!("{:?}", words);
    }
}
