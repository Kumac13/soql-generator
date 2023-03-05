use std::{
    error::Error,
    fmt::{self, Display},
    string::ParseError,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidOption,
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidOption => {
                write!(f, "ParseError: invalid option")
            }
            ParseError::Empty => write!(f, "ParseError: empty option"),
        }
    }
}
