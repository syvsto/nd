use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    Parse,
    Eval,
    Msg(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::Parse => write!(f, "Couldn't parse"),
            ErrorType::Eval => write!(f, "Couldn't eval"),
            ErrorType::Msg(m) => write!(f, "{}", m),
        }
    }
}

impl error::Error for ErrorType {}
