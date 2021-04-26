use std::collections::HashMap;
use crate::errors::ErrorType;
use crate::parser::{Ast, Builtins, Token};
use crate::data::{Ty, A, u8_f, vf_to_u8, u8_to_vf};

#[derive(Debug)]
enum Op {
    ContinueToDefinitionEnd,
    ContinueToArrayEnd,
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<A>) -> Result<Option<Op>, ErrorType> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    match last.ty {
                        Ty::F => println!("{:?}", u8_to_vf(&last.c)),
                        Ty::C => println!("{:?}", std::str::from_utf8(&last.c)),
                    }
                }
                Ok(None)
            }

            Plus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut r = A::new(Ty::F,n1.r,n1.l,n1.s.clone(),vec![0;n1.l]);
                        let mut v = Vec::with_capacity(n1.l);
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                let f = u8_f(&n1.c[i..i+4]) + u8_f(&n2.c[i..i+4]);
                                v.push(f);
                            }
                        }
                        r.c = vf_to_u8(&v).to_vec();
                        stack.push(r);
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                if n1 == n2 {
                    stack.push(A::new_f(1.))
                } else {
                    stack.push(A::new_f(0.))
                }
                Ok(None)
            }

            Minus => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut r = A::new(Ty::F,n1.r,n1.l,n1.s.clone(),vec![0;n1.l]);
                        let mut v = Vec::with_capacity(n1.l);
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                let f = u8_f(&n1.c[i..i+4]) - u8_f(&n2.c[i..i+4]);
                                v.push(f);
                            }
                        }
                        r.c = vf_to_u8(&v).to_vec();
                        stack.push(r);
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut r = A::new(Ty::F,n1.r,n1.l,n1.s.clone(),vec![0;n1.l]);
                        let mut v = Vec::with_capacity(n1.l);
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                let f = u8_f(&n1.c[i..i+4]) * u8_f(&n2.c[i..i+4]);
                                v.push(f);
                            }
                        }
                        r.c = vf_to_u8(&v).to_vec();
                        stack.push(r);
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut r = A::new(Ty::F,n1.r,n1.l,n1.s.clone(),vec![0;n1.l]);
                        let mut v = Vec::with_capacity(n1.l);
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                let f = u8_f(&n1.c[i..i+4]) / u8_f(&n2.c[i..i+4]);
                                v.push(f);
                            }
                        }
                        r.c = vf_to_u8(&v).to_vec();
                        stack.push(r);
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            If => {
                let a = stack.pop().ok_or(ErrorType::Eval)?;
                match (&a.ty, &a.r) {
                    (Ty::F, 0) => Ok(None),
                    _ => Err(ErrorType::Eval),
                }
            }

            And => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut t = 1.;
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                if u8_f(&n1.c[i..i+4]) <= 0. && u8_f(&n2.c[i..i+4]) <= 0. {
                                    t = 0.;
                                    break;
                                }
                            }
                        }
                        stack.push(A::new_f(t));
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Or => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                match (&n1.ty, &n2.ty) {
                    (Ty::F, Ty::F) => {
                        let mut t = 1.;
                        for n in n2.s.iter() {
                            for i in 0..*n {
                                let i = i * 4;
                                if u8_f(&n1.c[i..i+4]) <= 0. || u8_f(&n2.c[i..i+4]) <= 0. {
                                    t = 0.;
                                    break;
                                }
                            }
                        }
                        stack.push(A::new_f(t));
                        Ok(None)
                    }
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Concat => {
                let n1 = stack.pop().ok_or(ErrorType::Eval)?;
                let n2 = stack.pop().ok_or(ErrorType::Eval)?;
                let s: Vec<_> = n1.s.iter().zip(n2.s.iter()).map(|(a,b)| a + b).collect();
                let mut r = A::new(n1.ty,n1.r,n1.l+n2.l,s.clone(),Vec::with_capacity(n1.l+n2.l));
                let mut a_i = 0; 
                let mut b_i = 0;
                for i in 0..s.len() {
                    let a_e = n1.s[i] * n1.ty.size();
                    let b_e = n2.s[i] * n2.ty.size();
                    let a = &n1.c[a_i..a_e];
                    let b = &n2.c[b_i..b_e];
                    let mut c = [b,a].concat();
                    a_i = n1.s[i]; b_i = n2.s[i];
                    r.c.append(&mut c);
                }
                stack.push(r);
                Ok(None)
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
        ContinueToDefinitionEnd => {
            if let Some(v) = next_of_type(Builtins::WordEnd, &ast) {
                return v;
            }
        }
        ContinueToArrayEnd => {
            if let Some(v) = next_of_type(Builtins::ArrayEnd, &ast) {
                return v;
            }
        }
    }

    current_index
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
