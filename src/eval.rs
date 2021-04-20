use std::collections::HashMap;
use std::iter::FromIterator;

use crate::errors::ErrorType;
use crate::parser::{Ast, Builtins, DataType, Token};

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
    fn eval(&self, stack: &mut Vec<Val>) -> Result<Option<Op>, ErrorType> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    match last {
                        Val::Number(n) => println!("{:?}", n),
                        Val::Char(c) => println!("{}", String::from_iter(c)),
                    }
                }
                Ok(None)
            }

            Plus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a + b).collect();
                        stack.push(Val::Number(n));
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        if a == b {
                            stack.push(Val::Number(vec![1.]))
                        } else {
                            stack.push(Val::Number(vec![0.]))
                        }
                        Ok(None)
                    }
                    (Val::Char(a), Val::Char(b)) => {
                        if a == b {
                            stack.push(Val::Number(vec![1.]))
                        } else {
                            stack.push(Val::Number(vec![0.]))
                        }
                        Ok(None)
                    }
                    _ => panic!("Couldn't compare values, not all values were comparable."),
                }
            }

            Minus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a - b).collect();
                        stack.push(Val::Number(n));
                        Ok(None)
                    }
                    _ => panic!("Couldn't subtract values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a * b).collect();
                        stack.push(Val::Number(n));
                        Ok(None)
                    }
                    _ => panic!("Couldn't multiply values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        let n = b.iter().zip(a.iter().cycle()).map(|(a, b)| a / b).collect();
                        stack.push(Val::Number(n));
                        Ok(None)
                    }
                    _ => panic!("Couldn't divide values, not all values were numbers."),
                }
            }

            If => {
                let comparison = stack.pop().ok_or(ErrorType::Eval)?;
                match comparison {
                    Val::Number(cmp) => {
                        if cmp.iter().all(|x| *x == 0.) {
                            return Ok(Some(Op::ContinueToForward));
                        }
                        Ok(None)
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }

            And => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        stack.push(Val::Number(
                            b.iter()
                                .zip(a.iter().cycle())
                                .map(|(a, b)| if *a >= 1. && *b >= 1. { 1. } else { 0. })
                                .collect(),
                        ));
                        Ok(None)
                    }
                    _ => panic!("Both stack elements were not present, or were not numbers."),
                }
            }

            Or => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (n1, n2) {
                    (Val::Number(a), Val::Number(b)) => {
                        stack.push(Val::Number(
                            b.iter()
                                .zip(a.iter().cycle())
                                .map(|(a, b)| if *a >= 1. || *b >= 1. { 1. } else { 0. })
                                .collect(),
                        ));
                        Ok(None)
                    }
                    _ => panic!("Both stack elements were not present, or were not numbers."),
                }
            }

            Concat => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;

                match (n1, n2) {
                    (Val::Number(mut a), Val::Number(b)) => {
                        a.extend(b);
                        stack.push(Val::Number(a));
                        Ok(None)
                    }
                    (Val::Char(mut a), Val::Char(b)) => {
                        a.extend(b);
                        stack.push(Val::Char(a));
                        Ok(None)
                    }
                    _ => panic!("Wrong type combination or missing values in concatenation."),
                }
            }

            WordStart => Ok(Some(Op::ContinueToDefinitionEnd)),

            ArrayStart => Ok(Some(Op::ContinueToArrayEnd)),

            Duplicate => {
                let n = stack.pop().ok_or(ErrorType::Eval)?;
                stack.push(n.clone());
                stack.push(n);
                Ok(None)
            }

            Swap => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;

                stack.push(n1);
                stack.push(n2);
                Ok(None)
            }

            Clear => {
                stack.clear();
                Ok(None)
            }

            ClearButOne => {
                if let Some((last, _)) = stack.split_last() {
                    let l = last.clone();
                    stack.clear();
                    stack.push(l);
                }
                Ok(None)
            }

            Pop => {
                stack.pop().ok_or(ErrorType::Eval)?;
                Ok(None)
            }

            Forward => Ok(None),
            WordEnd => Ok(None),
            ArrayEnd => Ok(None),
        }
    }
}

fn next_of_type(ty: Builtins, tokens: &[Token]) -> Option<usize> {
    tokens.iter().position(|token| match token {
        Token::Builtin(t) => *t == ty,
        _ => false,
    })
}

fn eval_op(op: &Op, current_index: usize, ast: &Ast) -> usize {
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
    ast: &Ast,
    stack: &mut Vec<Val>,
    words: &HashMap<String, Ast>,
    debug: bool,
) -> Result<(), ErrorType> {
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
                if let Some(ref op) = func.eval(stack)? {
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

    Ok(())
}
