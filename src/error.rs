//! Error type for the crate.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// Data was truncated or a structure pointed outside the buffer.
    Truncated(&'static str),
    /// The file format could not be recognised / parsed.
    Format(String),
    Unsupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Truncated(what) => write!(f, "truncated data: {what}"),
            Error::Format(m) => write!(f, "format error: {m}"),
            Error::Unsupported(m) => write!(f, "unsupported: {m}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
