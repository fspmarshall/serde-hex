//! Miscellaneous type used by this crate.
use std::{io,fmt,result,error};


/// An alias of `std::result::Result` with this crate's
/// `Error` type inserted by default.
pub type Result<T> = result::Result<T,Error>;

/// enum representing the possible errors that may
/// occur while parsing a hexadecimal string.
#[derive(Debug)]
pub enum Error {
    /// A wrapper around an `std::io::Error`.  This error indicates 
    /// a failure to write to a buffer when converting a type to hex.
    IoError(io::Error),
    /// Indicates that a buffer of an unexpected size was received.
    /// For strict implementations, this is anything other than the
    /// exact expected size.  For compact implementations, this is
    /// anything larger than max size, or less than one.
    BadSize(usize),
    /// Indicates that a non hexadecimal character was encountered.
    BadChar(char),
    /// Indicates that a byte was encounered with a larger value than
    /// can be represented by a single character (16 or greater).
    /// If this error is generated, your implementation is probably wrong.
    BadByte(u8)
}



// implement `Display` to allow user-facing errors.  Required
// by the `std::error::Error` trait.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::BadSize(ref val) => write!(f, "Invalid Hex Size: {}", val),
            Error::BadChar(ref val) => write!(f, "Invalid Hex Char: {}", val),
            Error::BadByte(ref val) => write!(f, "Invalid Byte Val: {}", val)
        }
    }
}

// implement the standard error trait for hexadecimal errors.
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::BadSize(_) => "hex string was not within allowable size range",
            Error::BadChar(_) => "encountered a non-hexadecimal character during parsing",
            Error::BadByte(_) => "encountered byte outside of character range (0x0 - 0xf)"
        }
    }

    fn cause(&self) -> Option<&error::Error> { 
        match *self {
            Error::IoError(ref err) => Some(err),
            _ => None
        }
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}
