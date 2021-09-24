use std::convert;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    /// Simple IO error
    Io(io::Error),
    /// Read unusual data from sysfs file.
    Unexpected(String),
    /// Invalid Path
    InvalidPath(String),
    /// Operation not supported on target os
    Unsupported(String),
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Unexpected(ref s) => write!(f, "Unexpected: {}", s),
            Error::InvalidPath(ref s) => write!(f, "Invalid Path: {}", s),
            Error::Unsupported(ref s) => write!(f, "Operation not supported on target os: {}", s),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
#[cfg(not(target_os = "wasi"))]
impl convert::From<nix::Error> for Error {
    fn from(e: nix::Error) -> Error {
        Error::Io(e.into())
    }
}
