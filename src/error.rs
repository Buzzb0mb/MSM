use std::io;
use std::num::ParseIntError;
use std::fmt;

// wrapper error type for convenience and/or lazyness

#[derive(Debug)]
pub enum MSMError {
    Parser(String),
    MissingSessionVariable(&'static str),
    IO(io::Error),
    ParseInt(ParseIntError),
    InvalidSessionIndex(usize)
}
pub type MSMResult<T> = std::result::Result<T, MSMError>;

impl fmt::Display for MSMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MSMError::Parser(s) => write!(f, "Parser error: {}", s),
            MSMError::MissingSessionVariable(s) => write!(f, "Missing session variable: {}", s),
            MSMError::IO(e) => write!(f, "IO error: {}", e.to_string()),
            MSMError::ParseInt(e) => write!(f, "Error parsing integer: {}", e.to_string()),
            MSMError::InvalidSessionIndex(i) => write!(f, "Invalid session index: {}", i)
        }
    }
}

impl std::error::Error for MSMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for MSMError {
    fn from(e: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
        MSMError::Parser(e.to_string())
    }
}

impl From<io::Error> for MSMError {
    fn from(e: io::Error) -> Self {
        MSMError::IO(e)
    }
}

impl From<ParseIntError> for MSMError {
    fn from(e: ParseIntError) -> Self {
        MSMError::ParseInt(e)
    }
}
