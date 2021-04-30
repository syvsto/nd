use crate::data::A;
use crate::errors::ErrorType;

pub fn plus(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ + b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn minus(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ - b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn multiply(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ - b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn divide(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ - b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn and(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ - b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn or(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => {
            let r: Vec<_> = a_
                .iter()
                .zip(b_.iter())
                .map(|(a__, b__)| a__ - b__)
                .collect();
            Ok(A::F(r))
        }
        _ => Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers.".to_string(),
        )),
    }
}

pub fn iff(a: A) -> Result<(), ErrorType> {
    match &a {
        A::F(f) => {
            if f[0] > 0. {
                Ok(())
            } else {
                Ok(())
            }
        }
        _ => Err(ErrorType::Eval),
    }
}

pub fn print(a: &A) {
    match a {
        A::F(f) => println!("{:?}", &f),
        A::C(f) => println!("{:?}", f.clone().into_iter().collect::<String>()),
    }
}

pub fn equal(a: A, b: A) -> A {
    if a == b {
        A::F(vec![1.])
    } else {
        A::F(vec![0.])
    }
}

pub fn concat(a: A, b: A) -> Result<A, ErrorType> {
    match (&a, &b) {
        (A::F(a_), A::F(b_)) => Ok(A::F(a_.iter().chain(b_.iter()).map(|x| *x).collect())),
        (A::C(a_), A::C(b_)) => Ok(A::C(a_.iter().chain(b_.iter()).map(|x| *x).collect())),
        _ => Err(ErrorType::Msg(
            "Couldn't match type of arrays to concatenate".to_string(),
        )),
    }
}
