use crate::errors::ErrorType;
use crate::data::{Ty, u8_f, u8_to_vf, vf_to_u8, A};

pub fn plus(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut r = A::new(Ty::F, a.r, a.l, a.s.clone(), vec![0; a.l]);
            let mut v = Vec::with_capacity(a.l);
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    let f = u8_f(&a.c[i..i + 4]) + u8_f(&b.c[i..i + 4]);
                    v.push(f);
                }
            }
            r.c = vf_to_u8(&v).to_vec();
            Ok(r)
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn minus(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut r = A::new(Ty::F, a.r, a.l, a.s.clone(), vec![0; a.l]);
            let mut v = Vec::with_capacity(a.l);
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    let f = u8_f(&a.c[i..i + 4]) - u8_f(&b.c[i..i + 4]);
                    v.push(f);
                }
            }
            r.c = vf_to_u8(&v).to_vec();
            Ok(r)
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn multiply(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut r = A::new(Ty::F, a.r, a.l, a.s.clone(), vec![0; a.l]);
            let mut v = Vec::with_capacity(a.l);
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    let f = u8_f(&a.c[i..i + 4]) * u8_f(&b.c[i..i + 4]);
                    v.push(f);
                }
            }
            r.c = vf_to_u8(&v).to_vec();
            Ok(r)
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn divide(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut r = A::new(Ty::F, a.r, a.l, a.s.clone(), vec![0; a.l]);
            let mut v = Vec::with_capacity(a.l);
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    let f = u8_f(&a.c[i..i + 4]) / u8_f(&b.c[i..i + 4]);
                    v.push(f);
                }
            }
            r.c = vf_to_u8(&v).to_vec();
            Ok(r)
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn and(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut t = 1.;
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    if u8_f(&a.c[i..i + 4]) <= 0. && u8_f(&b.c[i..i + 4]) <= 0. {
                        t = 0.;
                        break;
                    }
                }
            }
            Ok(A::new_f(t))
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn or(a: A, b: A) -> Result<A, ErrorType> {
    match (&a.ty, &b.ty) {
        (Ty::F, Ty::F) => {
            let mut t = 1.;
            for n in b.s.iter() {
                for i in 0..*n {
                    let i = i * 4;
                    if u8_f(&a.c[i..i + 4]) <= 0. || u8_f(&b.c[i..i + 4]) <= 0. {
                        t = 0.;
                        break;
                    }
                }
            }
            Ok(A::new_f(t))
        }
        _ => Err(ErrorType::Msg("Couldn't add values, not all values were numbers.".to_string())),
    }
}

pub fn iff(a: A) -> Result<(), ErrorType> {
    match (&a.ty, &a.r) {
        (Ty::F, 0) => Ok(()),
        _ => Err(ErrorType::Eval),
    }
}

pub fn print(a: &A) {
    match a.ty {
        Ty::F => println!("{:?}", u8_to_vf(&a.c)),
        Ty::C => println!("{:?}", std::str::from_utf8(&a.c)),
    }
}

pub fn equal(a: A, b: A) -> A {
    if a == b {
        A::new_f(1.)
    } else {
       A::new_f(0.)
    }
}


pub fn concat(a: A, b: A) -> A {
    let s: Vec<_> = a.s.iter().zip(b.s.iter()).map(|(a, b)| a + b).collect();
    let mut r = A::new(
        a.ty,
        a.r,
        a.l + b.l,
        s.clone(),
        Vec::with_capacity(a.l + b.l),
    );
    let mut a_i = 0;
    let mut b_i = 0;
    for i in 0..s.len() {
        let a_e = a.s[i] * a.ty.size();
        let b_e = b.s[i] * b.ty.size();
        let a_ = &a.c[a_i..a_e];
        let b_ = &b.c[b_i..b_e];
        let mut c = [b_, a_].concat();
        a_i = a.s[i];
        b_i = b.s[i];
        r.c.append(&mut c);
    }
    r
}
