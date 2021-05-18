use std::str::Utf8Error;

#[derive(Debug)]
pub enum ErrorType {
    ParseError,
    InvalidNameIndex,
    Utf8Error(Utf8Error),
    IntegerConversion
}

impl From<Utf8Error> for ErrorType {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}
