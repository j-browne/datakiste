use std::{
    convert::From,
    num::{ParseIntError, ParseFloatError},
    io,
};

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error{
    Int(ParseIntError),
    Float(ParseFloatError),
    Io(io::Error),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::Int(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::Float(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
