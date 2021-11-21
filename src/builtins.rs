use crate::array::A;
use crate::errors::ErrorType;

pub fn plus(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x + y)
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg(
            "Couldn't add values, not all values were numbers."
        ))
    }
}

pub fn minus(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x - y)
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg(
            "Couldn't subtract values, not all values were numbers."
        ))
    }
}

pub fn multiply(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x * y)
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg(
            "Couldn't divide values, not all values were numbers."
        ))
    }
}

pub fn divide(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x / y)
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg(
            "Couldn't divide values, not all values were numbers."
        ))
    }
}

pub fn and(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x.and(y))
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg("Couldn't compare values."))
    }
}

pub fn or(a: A, b: A) -> Result<A, ErrorType> {
    let d: Option<Vec<_>> =
        a.d.iter()
            .zip(b.d.iter().cycle())
            .map(|(x, y)| x.or(y))
            .collect();

    if let Some(d) = d {
        Ok(A { rank: a.rank, d })
    } else {
        Err(ErrorType::Msg("Couldn't compare values."))
    }
}

pub fn iff(a: A) -> Result<bool, ErrorType> {
    if a.d.iter().fold(0., |acc, x| x + acc) > 0. {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn print(a: &A) {
    if let Some(v) = a.d.first() {
        if v.is_num() {
            let fs: Option<Vec<_>> = a.d.iter().map(|x| x.as_f64()).collect();
            println!("{:#?}", fs.unwrap());
        } else {
            let fs: Option<String> = a.d.iter().map(|x| x.as_char()).collect();
            println!("{:#?}", fs.unwrap());
        }
    }
}

pub fn equal(a: A, b: A) -> A {
    A { 
        rank: a.rank,
        d: a.d.iter().zip(b.d.iter()).map(|(x, y)| { x.eql(y) }).collect(),
    }
}

pub fn concat(a: A, b: A) -> Result<A, ErrorType> {
    let x = a.d.first().ok_or(ErrorType::Msg("No elements in first argument array"))?;
    let y = b.d.first().ok_or(ErrorType::Msg("No elements in first argument array"))?;

    if !x.eq_type(y) { return Err(ErrorType::Msg("Mismatched types."))}

    Ok(A {
        rank: a.rank,
        d: a.d.into_iter().chain(b.d.into_iter()).map(|x| x).collect()
    })
}
