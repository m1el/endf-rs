//! Errors emited by the reader.

use ::std::num::{ParseFloatError,ParseIntError};

/// Errors emited by the reader
#[derive(Debug)]
pub enum ReadError {
    /// Failed to parse an Int
    BadInt(ParseIntError),
    /// Failed to parse a Float
    BadFloat(ParseFloatError),
    /// I/O Error
    IoError(::std::io::Error),
    /// Section was not followed by SEND record (MT=0, NS=99999)
    MissingSectionTerminator,
    /// A record is not 80 characters long
    RecordTooShort,
    /// Invalid number of elements in tabular/list data
    InvalidElementCount,
    /// Invalid interpolation number
    InvalidInterpolation,
    /// Unexpected end of file
    Eof,
}

impl From<ParseIntError> for ReadError {
    fn from(x: ParseIntError) -> ReadError {
        ReadError::BadInt(x)
    }
}

impl From<ParseFloatError> for ReadError {
    fn from(x: ParseFloatError) -> ReadError {
        ReadError::BadFloat(x)
    }
}

impl From<::std::io::Error> for ReadError {
    fn from(x: ::std::io::Error) -> ReadError {
        ReadError::IoError(x)
    }
}
