use crate::builtins;
use crate::data::A;
use crate::errors::ErrorType;
use crate::parser::{Ast, Builtins, Token};
use std::collections::HashMap;

enum ControlFlow {
    SkipToThen,
    Continue,
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<A>) -> Result<ControlFlow, ErrorType> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    builtins::print(last);
                }
            }

            Plus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::plus(n2, n1)?;
                stack.push(r);
            }

            Equal => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::equal(n1, n2);
                stack.push(r);
            }

            Minus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::minus(n2, n1)?;
                stack.push(r);
            }

            Multiply => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::multiply(n2, n1)?;
                stack.push(r);
            }

            Divide => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::divide(n2, n1)?;
                stack.push(r);
            }

            If => {
                let a = stack.pop().ok_or(ErrorType::Eval)?;
                let c = builtins::iff(a)?;
                if c {
                    return Ok(ControlFlow::SkipToThen);
                }
            }

            And => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::and(n1, n2)?;
                stack.push(r);
            }

            Or => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::or(n1, n2)?;
                stack.push(r);
            }

            Concat => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let r = builtins::concat(n1, n2)?;
                stack.push(r);
            }

            Duplicate => {
                let n = stack.pop().ok_or(ErrorType::Eval)?;
                stack.push(n.clone());
                stack.push(n);
            }

            Swap => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
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
                stack.pop().ok_or(ErrorType::Eval)?;
            }

            Forward => {}
        }
        Ok(ControlFlow::Continue)
    }
}

pub fn eval(
    ast: &Ast,
    stack: &mut Vec<A>,
    words: &HashMap<String, Ast>,
    debug: bool,
) -> Result<(), ErrorType> {
    use Token::*;

    let mut i = 0;

    while i < ast.len() {
        match &ast[i] {
            Word(name) => {
                if let Some(word) = words.get(name.as_str()) {
                    let _ = eval(word, stack, words, debug);
                }
            }
            Data(data) => stack.push(data.clone()),
            Builtin(func) => match func.eval(stack)? {
                ControlFlow::SkipToThen => {
                    i += ast[i..]
                        .iter()
                        .position(|x| x == &Builtin(Builtins::Forward))
                        .ok_or(ErrorType::Eval)?;
                }
                ControlFlow::Continue => {}
            },
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
