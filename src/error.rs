use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadConstruction,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadConstruction => write!(f, "BadConstruction"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::BadConstruction => "BadConstruction",
        }
    }
}
