use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CliError {
    msg: String,
}

pub type CliResult<T> = Result<T, CliError>;

impl CliError {
    pub fn new<T: Into<String>>(msg: T) -> CliError {
        CliError { msg: msg.into() }
    }
}

impl<E: Error> From<E> for CliError {
    fn from(e: E) -> CliError {
        CliError::new(e.description())
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
